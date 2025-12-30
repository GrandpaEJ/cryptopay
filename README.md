# CryptoPay - BscScan Payment Gateway Library

A comprehensive Rust library for integrating with BscScan API to verify and monitor cryptocurrency payments on Binance Smart Chain (BSC).

[![Crates.io](https://img.shields.io/crates/v/cryptopay.svg)](https://crates.io/crates/cryptopay)
[![Documentation](https://docs.rs/cryptopay/badge.svg)](https://docs.rs/cryptopay)
[![License](https://img.shields.io/crates/l/cryptopay.svg)](LICENSE)

## Features

- 🔗 **Full BscScan API Integration** - Complete support for accounts, transactions, tokens, and gas tracking
- 💰 **Payment Verification** - Verify BNB and BEP20 token payments with automatic confirmation tracking
- 👀 **Payment Monitoring** - Monitor pending payments with customizable callbacks
- ⚡ **Rate Limiting** - Built-in token bucket rate limiter respecting BscScan's 5 req/s limit
- 💾 **Caching** - In-memory LRU cache to minimize API calls and improve performance
- 🔄 **API Key Rotation** - Automatic round-robin rotation for multiple API keys
- 🎯 **Type Safety** - Strongly-typed models with helper methods for common conversions
- 📦 **Library-First** - Designed as a lib crate for easy integration into any Rust project

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
cryptopay = "0.1"
rust_decimal = "1.33"
tokio = { version = "1", features = ["full"] }
```

### Basic Payment Verification

```rust
use cryptopay::*;
use rust_decimal::Decimal;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with your BscScan API key
    let client = BscScanClient::new("your-api-key")?;
    let verifier = PaymentVerifier::new(client);
    
    // Create payment request
    let payment = PaymentRequest {
        amount: Decimal::from_str("0.1")?,
        currency: Currency::BNB,
        recipient_address: "0x...".to_string(),
        required_confirmations: 12,
        timeout_seconds: Some(1800),
    };
    
    // Verify payment
    match verifier.verify_payment(&payment).await? {
        VerificationResult::Confirmed { tx_hash } => {
            println!("Payment confirmed: {}", tx_hash);
        }
        VerificationResult::Pending { confirmations, .. } => {
            println!("Pending: {} confirmations", confirmations);
        }
        _ => println!("No payment found"),
    }
    
    Ok(())
}
```

### Monitor Payments with Callbacks

```rust
use cryptopay::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BscScanClient::new("your-api-key")?;
    let monitor = PaymentMonitor::builder()
        .client(client)
        .poll_interval(Duration::from_secs(10))
        .build();
    
    let payment_request = PaymentRequest::bnb(
        Decimal::new(1, 1), // 0.1 BNB
        "0x...",
        12,
    );
    
    monitor.start_monitoring(payment_request, |status| {
        println!("Payment status: {:?}", status);
    }).await?;
    
    Ok(())
}
```

### BEP20 Token Payments

```rust
use cryptopay::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BscScanClient::new("your-api-key")?;
    let verifier = PaymentVerifier::new(client);
    
    // Use predefined USDT currency
    let payment = PaymentRequest {
        amount: Decimal::from(100),
        currency: Currency::usdt(), // BSC USDT
        recipient_address: "0x...".to_string(),
        required_confirmations: 6,
        timeout_seconds: Some(3600),
    };
    
    let result = verifier.verify_payment(&payment).await?;
    println!("{:?}", result);
    
    Ok(())
}
```

## Configuration

### Using Environment Variables

```bash
export BSCSCAN_API_KEY="your-api-key"
export BSCSCAN_RATE_LIMIT=5
export BSCSCAN_CACHE_TTL=300
```

```rust
use cryptopay::ClientConfig;

let config = ClientConfig::from_env()?;
let client = BscScanClient::with_config(config)?;
```

### Using Builder Pattern

```rust
use cryptopay::*;

let config = ClientConfig::builder()
    .api_key("key1")
    .api_key("key2")  // Multiple keys for rotation
    .rate_limit(10)   // 10 req/s (for PRO plan)
    .timeout(60)
    .cache_ttl(600)
    .testnet()        // Use BSC testnet
    .build()?;

let client = BscScanClient::with_config(config)?;
```

## API Documentation

### BscScan Client

```rust
// Account endpoints
let balance = client.get_balance("0x...").await?;
let transactions = client.get_transactions("0x...", 0, 99999999, 1, 100, "desc").await?;

// Transaction endpoints
let tx = client.get_transaction("0xtxhash").await?;
let confirmations = client.get_confirmations("0xtxhash").await?;

// Token endpoints
let token_transfers = client.get_token_transfers(
    "0x...",  // address
    Some("0xcontract"),  // filter by token contract
    0,  // start block
    99999999,  // end block
    1,  // page
    100,  // offset
    "desc"  // sort
).await?;

// Gas tracker
let gas_oracle = client.get_gas_oracle().await?;
println!("Fast gas price: {} gwei", gas_oracle.fast_gwei());
```

### Payment Processing

```rust
// Create payment request
let payment = PaymentRequest::bnb(amount, recipient, confirmations);

// Or with timeout
let payment = PaymentRequest::bnb(amount, recipient, confirmations)
    .with_timeout(1800);  // 30 minutes

// Verify payment
let verifier = PaymentVerifier::new(client);
let result = verifier.verify_payment(&payment).await?;

// Monitor payment
let monitor = PaymentMonitor::new(client, Duration::from_secs(10));
monitor.start_monitoring(payment, |status| {
    // Handle status updates
}).await?;
```

### Predefined Currencies

```rust
Currency::BNB                              // Native BNB
Currency::usdt()                           // BSC USDT
Currency::usdc()                           // BSC USDC
Currency::busd()                           // BSC BUSD
Currency::bep20("0xcontract", 18)          // Custom token
```

## Examples

Run the examples with your BscScan API key:

```bash
# Basic payment verification
BSCSCAN_API_KEY=your-key cargo run --example basic_payment

# Token payment verification
BSCSCAN_API_KEY=your-key cargo run --example token_payment

# Payment monitoring with callbacks
BSCSCAN_API_KEY=your-key cargo run --example payment_monitor
```

## Testing

### Unit Tests

```bash
cargo test
```

### Integration Tests with Real API

```bash
BSCSCAN_API_KEY=your-key cargo test --features integration-tests -- --ignored
```

### BSC Testnet

Get testnet BNB from the faucet:
- https://testnet.bnbchain.org/faucet-smart

Use testnet configuration:

```rust
let client = BscScanClient::testnet("your-api-key")?;
```

## Rate Limits

- **Free tier**: 5 requests/second
- **PRO tier**: Higher limits (configure via `rate_limit()`)

The library automatically enforces rate limits to prevent exceeding BscScan's limits.

## Getting a BscScan API Key

1. Go to [BscScan](https://bscscan.com/)
2. Create an account
3. Navigate to API-KEYs section
4. Generate a new API key (free)

## Feature Flags

```toml
[dependencies]
cryptopay = { version = "0.1", features = ["postgres-storage"] }
```

Available features:
- `postgres-storage` - PostgreSQL payment storage implementation
- `sqlite-storage` - SQLite payment storage implementation
- `redis-cache` - Redis-backed distributed cache (TBD)

## Use Cases

- **E-commerce**: Accept BNB/BEP20 payments for online stores
- **Payment Gateways**: Build custom crypto payment solutions
- **DeFi Applications**: Monitor on-chain transactions
- **Wallets**: Track address balances and transaction history
- **Analytics**: Analyze blockchain data and gas prices
- **Bots**: Automated trading and transaction monitoring

## Architecture

- **Library crate** - Integrate into any Rust application
- **Async/await** - Built on Tokio for high performance
- **Rate limiting** - Token bucket algorithm with Governor
- **Caching** - In-memory LRU cache with Moka
- **Type safety** - Comprehensive error handling with thiserror

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Disclaimer

This library is for educational and development purposes. Always test thoroughly on testnet before using with real funds.
