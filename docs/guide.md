# CryptoPay User Guide

Complete guide for integrating Ethereum payment verification into your Rust application.

## Table of Contents

- [Installation](#installation)
- [Basic Usage](#basic-usage)
- [Payment Verification](#payment-verification)
- [Payment Monitoring](#payment-monitoring)
- [Configuration](#configuration)
- [Error Handling](#error-handling)
- [Best Practices](#best-practices)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cryptopay = "0.1"
tokio = { version = "1", features = ["full"] }
rust_decimal = "1.33"
```

## Basic Usage

### Initialize the Client

```rust
use cryptopay::*;

// Simple initialization
let client = EtherscanClient::new("your-api-key")?;

// Or from environment
let config = ClientConfig::from_env()?;
let client = EtherscanClient::with_config(config)?;
```

### Verify a Payment

```rust
use rust_decimal::Decimal;

let verifier = PaymentVerifier::new(client);

let payment = PaymentRequest {
    amount: Decimal::new(1, 1), // 0.1 ETH
    currency: Currency::ETH,
    recipient_address: "0x...".to_string(),
    required_confirmations: 12,
    timeout_seconds: Some(1800),
};

let result = verifier.verify_payment(&payment).await?;
```

## Payment Verification

### ETH Payments

```rust
// Using helper method
let payment = PaymentRequest::eth(
    Decimal::new(5, 2),  // 0.05 ETH
    "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb0",
    12,
);

// Verify
match verifier.verify_payment(&payment).await? {
    VerificationResult::Confirmed { tx_hash, confirmations } => {
        println!("✅ Confirmed! TX: {}", tx_hash);
    }
    VerificationResult::Pending { confirmations, .. } => {
        println!("⏳ Pending: {}/12 confirmations", confirmations);
    }
    VerificationResult::NotFound => {
        println!("❌ No matching transaction found");
    }
    VerificationResult::Failed { reason } => {
        println!("❌ Failed: {}", reason);
    }
}
```

### ERC20 Token Payments

```rust
// USDT payment
let payment = PaymentRequest {
    amount: Decimal::from(100),
    currency: Currency::usdt(),
    recipient_address: "0x...".to_string(),
    required_confirmations: 6,
    timeout_seconds: Some(3600),
};

// Or custom token
let payment = PaymentRequest::token(
    Decimal::from(50),
    "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
    6,  // decimals
    "0x...",
    6,
);
```

### Payment Timeouts

```rust
let payment = PaymentRequest::eth(amount, recipient, 12)
    .with_timeout(1800); // 30 minutes

if payment.is_expired(payment.created_at) {
    println!("Payment expired!");
}
```

## Payment Monitoring

Monitor payments in real-time with callbacks:

```rust
use std::time::Duration;

let monitor = PaymentMonitor::builder()
    .client(client)
    .poll_interval(Duration::from_secs(10))
    .build();

let payment = PaymentRequest::eth(amount, recipient, 12);

monitor.start_monitoring(payment, |status| {
    match status {
        PaymentStatus::Detected { tx_hash, confirmations } => {
            println!("📥 Detected: {} ({} confirms)", tx_hash, confirmations);
        }
        PaymentStatus::Confirmed { tx_hash, .. } => {
            println!("✅ Confirmed: {}", tx_hash);
        }
        _ => {}
    }
}).await?;
```

### One-Time Check

For a single status check without monitoring:

```rust
let status = monitor.check_payment_status(&payment).await?;
println!("Current status: {:?}", status);
```

## Configuration

### Environment Variables

Create a `.env` file:

```bash
ETHERSCAN_API_KEY=your_api_key_here
ETHERSCAN_RATE_LIMIT=5
ETHERSCAN_TIMEOUT=30
ETHERSCAN_CACHE_TTL=300
ETHERSCAN_CACHE_MAX_SIZE=1000
```

Load configuration:

```rust
let config = ClientConfig::from_env()?;
let client = EtherscanClient::with_config(config)?;
```

### Builder Configuration

```rust
let config = ClientConfig::builder()
    .api_key("key1")
    .api_key("key2")        // Multiple keys for rotation
    .api_key("key3")
    .rate_limit(10)         // Requests per second
    .timeout(60)            // Request timeout in seconds
    .cache_ttl(600)         // Cache TTL in seconds
    .cache_max_size(5000)   // Max cache entries
    .testnet()              // Use Sepolia testnet
    .build()?;
```

### Testnet Usage

```rust
// Sepolia testnet
let client = EtherscanClient::testnet("your-api-key")?;

// Or via builder
let config = ClientConfig::builder()
    .api_key("your-key")
    .testnet()
    .build()?;
```

## Error Handling

The library uses a custom `Result<T>` type:

```rust
use cryptopay::{Error, Result};

match verifier.verify_payment(&payment).await {
    Ok(result) => {
        // Handle verification result
    }
    Err(Error::ApiError(msg)) => {
        eprintln!("API error: {}", msg);
    }
    Err(Error::RateLimitExceeded) => {
        eprintln!("Rate limit exceeded, retrying...");
    }
    Err(Error::InvalidAddress(addr)) => {
        eprintln!("Invalid address: {}", addr);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

### Common Errors

- `ApiError` - Etherscan API returned an error
- `RateLimitExceeded` - Rate limit hit (client handles automatically)
- `InvalidAddress` - Invalid Ethereum address format
- `HttpRequest` - Network/HTTP error
- `Serialization` - JSON parsing error
- `InvalidConfig` - Invalid configuration

## Best Practices

### 1. Rate Limiting

The client automatically handles rate limiting:

```rust
// Free tier: 5 req/s (default)
let client = EtherscanClient::new("key")?;

// PRO tier: configure higher limit
let config = ClientConfig::builder()
    .api_key("pro-key")
    .rate_limit(20)  // Adjust based on your plan
    .build()?;
```

### 2. API Key Rotation

Use multiple API keys for better reliability:

```rust
let config = ClientConfig::builder()
    .api_key("key1")
    .api_key("key2")
    .api_key("key3")
    .build()?;

// Client will rotate through keys automatically
```

### 3. Caching

Enable caching to reduce API calls:

```rust
let config = ClientConfig::builder()
    .api_key("key")
    .cache_ttl(300)       // Cache for 5 minutes
    .cache_max_size(2000) // Store up to 2000 entries
    .build()?;

let client = EtherscanClient::with_config(config)?;

// Clear cache if needed
client.clear_cache().await;

// Check cache stats
let (entries, size) = client.cache_stats();
```

### 4. Confirmation Thresholds

Choose appropriate confirmation counts:

```rust
// ETH payments: 12-15 confirmations (finality)
let eth_payment = PaymentRequest::eth(amount, recipient, 12);

// Token payments: 6-12 confirmations (faster confirmation)
let token_payment = PaymentRequest::token(amount, contract, decimals, recipient, 6);

// High-value: 20+ confirmations (extra security)
let secure_payment = PaymentRequest::eth(large_amount, recipient, 20);
```

### 5. Amount Handling

Use `Decimal` for precise amounts:

```rust
use rust_decimal::Decimal;
use std::str::FromStr;

// From string
let amount = Decimal::from_str("0.05")?;

// From integer (value, decimal places)
let amount = Decimal::new(5, 2);  // 0.05

// Arithmetic
let total = amount * Decimal::from(2);  // 0.10
```

### 6. Error Recovery

Implement retry logic for transient errors:

```rust
use tokio::time::{sleep, Duration};

let max_retries = 3;
for attempt in 0..max_retries {
    match verifier.verify_payment(&payment).await {
        Ok(result) => return Ok(result),
        Err(Error::HttpRequest(_)) if attempt < max_retries - 1 => {
            sleep(Duration::from_secs(2u64.pow(attempt as u32))).await;
            continue;
        }
        Err(e) => return Err(e),
    }
}
```

### 7. Testing

Test on Sepolia testnet first:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_payment_verification() {
        let client = EtherscanClient::testnet("test-key").unwrap();
        let verifier = PaymentVerifier::new(client);
        
        // Use testnet addresses and transactions
        let payment = PaymentRequest::eth(
            Decimal::new(1, 3),
            "0x...",  // Testnet address
            3,        // Fewer confirmations on testnet
        );
        
        let result = verifier.verify_payment(&payment).await;
        assert!(result.is_ok());
    }
}
```

## Advanced Usage

### Direct API Access

Access raw Etherscan API endpoints:

```rust
use cryptopay::client::AccountEndpoints;

// Get balance
let balance = client.get_balance("0x...").await?;
println!("Balance: {} ETH", balance.bnb());

// Get transactions
let txs = client.get_transactions(
    "0x...",
    0,      // start block
    999999999,  // end block
    1,      // page
    100,    // limit
    "desc", // sort
).await?;

// Get transaction by hash
let tx = client.get_transaction("0x...").await?;
```

### Gas Price Estimation

```rust
use cryptopay::client::GasEndpoints;

// Get gas oracle
let gas = client.get_gas_oracle().await?;
println!("Fast gas: {} gwei", gas.fast_gwei());

// Estimate for specific speed
let price = client.estimate_gas_price(GasSpeed::Fast).await?;
```

## Next Steps

- Check out the [examples/](../examples/) directory for complete working examples
- Read the [API documentation](https://docs.rs/cryptopay) for detailed API reference
- See [CHANGELOG.md](../CHANGELOG.md) for version history
