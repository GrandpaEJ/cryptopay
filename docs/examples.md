# CryptoPay Examples

Collection of examples demonstrating various use cases.

## Basic Examples

### 1. Simple ETH Payment Verification

```rust
use cryptopay::*;
use rust_decimal::Decimal;

#[tokio::main]
async fn main() -> Result<()> {
    let client = EtherscanClient::new("your-api-key")?;
    let verifier = PaymentVerifier::new(client);
    
    let payment = PaymentRequest::eth(
        Decimal::new(1, 1),  // 0.1 ETH
        "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb0",
        12,
    );
    
    match verifier.verify_payment(&payment).await? {
        VerificationResult::Confirmed { tx_hash, .. } => {
            println!("✅ Payment confirmed: {}", tx_hash);
        }
        _ => println!("⏳ Payment not yet confirmed"),
    }
    
    Ok(())
}
```

### 2. ERC20 Token Payment

```rust
use cryptopay::*;
use rust_decimal::Decimal;

#[tokio::main]
async fn main() -> Result<()> {
    let client = EtherscanClient::new("your-api-key")?;
    let verifier = PaymentVerifier::new(client);
    
    // USDT payment
    let payment = PaymentRequest {
        amount: Decimal::from(100),
        currency: Currency::usdt(),
        recipient_address: "0x...".to_string(),
        required_confirmations: 6,
        timeout_seconds: Some(3600),
    };
    
    let result = verifier.verify_payment(&payment).await?;
    println!("{:?}", result);
    
    Ok(())
}
```

### 3. Payment Monitoring with Callbacks

```rust
use cryptopay::*;
use rust_decimal::Decimal;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let client = EtherscanClient::new("your-api-key")?;
    
    let monitor = PaymentMonitor::builder()
        .client(client)
        .poll_interval(Duration::from_secs(10))
        .build();
    
    let payment = PaymentRequest::eth(
        Decimal::new(1, 1),
        "0x...",
        12,
    );
    
    monitor.start_monitoring(payment, |status| {
        match status {
            PaymentStatus::Detected { tx_hash, confirmations } => {
                println!("📥 Detected: {} ({} confirms)", tx_hash, confirmations);
            }
            PaymentStatus::Confirmed { tx_hash, .. } => {
                println!("✅ Confirmed: {}", tx_hash);
                // Process order, update database, etc.
            }
            _ => {}
        }
    }).await?;
    
    Ok(())
}
```

## Advanced Examples

### 4. Multiple API Keys with Rotation

```rust
let config = ClientConfig::builder()
    .api_key("key1")
    .api_key("key2")
    .api_key("key3")
    .rate_limit(15)  // Combined limit
    .build()?;

let client = EtherscanClient::with_config(config)?;
```

### 5. Custom Configuration

```rust
let config = ClientConfig::builder()
    .api_key("pro-key")
    .rate_limit(20)           // PRO plan limit
    .timeout(60)              // Longer timeout
    .cache_ttl(600)           // Cache for 10 minutes
    .cache_max_size(5000)     // Larger cache
    .build()?;

let client = EtherscanClient::with_config(config)?;
```

### 6. Error Handling

```rust
match verifier.verify_payment(&payment).await {
    Ok(VerificationResult::Confirmed { tx_hash, .. }) => {
        // Payment successful
        process_payment(tx_hash).await?;
    }
    Ok(VerificationResult::Pending { confirmations, .. }) => {
        // Still pending
        println!("Waiting: {}/12 confirmations", confirmations);
    }
    Ok(VerificationResult::NotFound) => {
        // No transaction found
        println!("Payment not received yet");
    }
    Err(Error::RateLimitExceeded) => {
        // Wait and retry
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    Err(e) => {
        // Handle other errors
        eprintln!("Error: {}", e);
    }
}
```

### 7. Batch Payment Verification

```rust
async fn verify_multiple_payments(
    verifier: &PaymentVerifier,
    payments: Vec<PaymentRequest>,
) -> Result<Vec<VerificationResult>> {
    let mut results = Vec::new();
    
    for payment in payments {
        let result = verifier.verify_payment(&payment).await?;
        results.push(result);
        
        // Small delay to respect rate limits
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    
    Ok(results)
}
```

### 8. Custom Amount Tolerance

```rust
use cryptopay::payment::utils::amounts_match;
use rust_decimal::Decimal;

let expected = Decimal::from(100);
let actual = Decimal::from_str("100.50")?;
let tolerance = Decimal::from(1); // 1% tolerance

if amounts_match(expected, actual, tolerance) {
    println!("Amount matches within tolerance");
}
```

### 9. Gas Price Monitoring

```rust
use cryptopay::client::GasEndpoints;

let client = EtherscanClient::new("api-key")?;

// Get current gas prices
let gas = client.get_gas_oracle().await?;
println!("Safe: {} gwei", gas.safe_gwei());
println!("Normal: {} gwei", gas.propose_gwei());
println!("Fast: {} gwei", gas.fast_gwei());

// Estimate for fast transaction
let price = client.estimate_gas_price(GasSpeed::Fast).await?;
println!("Estimated price: {} gwei", price);
```

### 10. Transaction History

```rust
use cryptopay::client::AccountEndpoints;

let client = EtherscanClient::new("api-key")?;

// Get last 100 transactions
let txs = client.get_transactions(
    "0x...",
    0,
    99999999,
    1,
    100,
    "desc",
).await?;

for tx in txs {
    if tx.is_successful() {
        println!("TX: {} - {} ETH", tx.hash, tx.value_bnb());
    }
}
```

## Integration Examples

### 11. Web Server Integration (Axum)

```rust
use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    verifier: PaymentVerifier,
}

#[derive(Deserialize)]
struct CheckPaymentRequest {
    recipient: String,
    amount: String,
}

#[derive(Serialize)]
struct CheckPaymentResponse {
    status: String,
    tx_hash: Option<String>,
}

async fn check_payment(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CheckPaymentRequest>,
) -> Json<CheckPaymentResponse> {
    let amount = Decimal::from_str(&req.amount).unwrap();
    let payment = PaymentRequest::eth(amount, req.recipient, 12);
    
    match state.verifier.verify_payment(&payment).await {
        Ok(VerificationResult::Confirmed { tx_hash, .. }) => {
            Json(CheckPaymentResponse {
                status: "confirmed".to_string(),
                tx_hash: Some(tx_hash),
            })
        }
        _ => Json(CheckPaymentResponse {
            status: "pending".to_string(),
            tx_hash: None,
        }),
    }
}

#[tokio::main]
async fn main() {
    let client = EtherscanClient::new("api-key").unwrap();
    let state = Arc::new(AppState {
        verifier: PaymentVerifier::new(client),
    });
    
    let app = Router::new()
        .route("/check-payment", post(check_payment))
        .with_state(state);
    
    // Run server...
}
```

### 12. Background Worker

```rust
use tokio::time::{interval, Duration};

async fn payment_worker(monitor: PaymentMonitor, pending_payments: Vec<PaymentRequest>) {
    let mut interval = interval(Duration::from_secs(30));
    
    loop {
        interval.tick().await;
        
        for payment in &pending_payments {
            match monitor.check_payment_status(payment).await {
                Ok(PaymentStatus::Confirmed { tx_hash, .. }) => {
                    println!("Payment confirmed: {}", tx_hash);
                    // Update database, send notification, etc.
                }
                Ok(status) => {
                    println!("Status: {:?}", status);
                }
                Err(e) => {
                    eprintln!("Error checking payment: {}", e);
                }
            }
        }
    }
}
```

## Running Examples

All examples are in the [`examples/`](../examples/) directory:

```bash
# Set your API key
export ETHERSCAN_API_KEY=your_key_here

# Run basic payment example
cargo run --example basic_payment

# Run token payment example
cargo run --example token_payment

# Run payment monitor example
cargo run --example payment_monitor
```

## Testing on Sepolia Testnet

```bash
# Get testnet ETH from faucet:
# https://sepoliafaucet.com/

# Use testnet in your code
let client = EtherscanClient::testnet("api-key")?;

# Lower confirmation requirements for faster testing
let payment = PaymentRequest::eth(amount, recipient, 3);
```
