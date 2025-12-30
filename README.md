# CryptoPay - Etherscan Payment Gateway

[![Crates.io](https://img.shields.io/crates/v/cryptopay.svg)](https://crates.io/crates/cryptopay)
[![Documentation](https://docs.rs/cryptopay/badge.svg)](https://docs.rs/cryptopay)
[![License](https://img.shields.io/crates/l/cryptopay.svg)](LICENSE-MIT)

A Rust library for verifying and monitoring cryptocurrency payments on Ethereum using the Etherscan API.

## Features

- ✅ **Etherscan API Integration** - Complete API coverage for accounts, transactions, tokens, and gas
- 💰 **Payment Verification** - Verify ETH and ERC20 token payments with confirmation tracking
- 👀 **Payment Monitoring** - Real-time monitoring with customizable callbacks
- ⚡ **Rate Limiting** - Built-in token bucket rate limiter (5 req/s default)
- 💾 **Caching** - In-memory LRU cache with configurable TTL
- 🔄 **API Key Rotation** - Round-robin rotation for multiple API keys
- 🎯 **Type Safety** - Strongly-typed models with helper methods

## Quick Start

```toml
[dependencies]
cryptopay = "0.1"
tokio = { version = "1", features = ["full"] }
rust_decimal = "1.33"
```

```rust
use cryptopay::*;
use rust_decimal::Decimal;

#[tokio::main]
async fn main() -> Result<()> {
    let client = EtherscanClient::new("your-api-key")?;
    let verifier = PaymentVerifier::new(client);
    
    let payment = PaymentRequest::eth(
        Decimal::new(1, 1), // 0.1 ETH
        "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb0",
        12, // confirmations
    );
    
    match verifier.verify_payment(&payment).await? {
        VerificationResult::Confirmed { tx_hash, .. } => {
            println!("✅ Payment confirmed: {}", tx_hash);
        }
        _ => println!("⏳ Payment pending or not found"),
    }
    
    Ok(())
}
```

## Documentation

- 📚 [API Documentation](https://docs.rs/cryptopay)
- 📖 [User Guide](docs/guide.md)
- 🚀 [Examples](examples/)
- 📝 [CHANGELOG](CHANGELOG.md)

## Examples

See the [`examples/`](examples/) directory for complete examples:

- [`basic_payment.rs`](examples/basic_payment.rs) - ETH payment verification
- [`token_payment.rs`](examples/token_payment.rs) - ERC20 token verification  
- [`payment_monitor.rs`](examples/payment_monitor.rs) - Real-time monitoring with callbacks

Run examples:
```bash
ETHERSCAN_API_KEY=your-key cargo run --example basic_payment
```

## Configuration

### Environment Variables

```bash
export ETHERSCAN_API_KEY="your-api-key"
export ETHERSCAN_RATE_LIMIT=5
export ETHERSCAN_CACHE_TTL=300
```

### Builder Pattern

```rust
let config = ClientConfig::builder()
    .api_key("key1")
    .api_key("key2")  // Multiple keys for rotation
    .rate_limit(10)   // Higher limit for PRO plan
    .testnet()        // Use Sepolia testnet
    .build()?;

let client = EtherscanClient::with_config(config)?;
```

## Supported Currencies

```rust
// Native ETH
Currency::ETH

// Predefined tokens
Currency::usdt()  // Tether (6 decimals)
Currency::usdc()  // USD Coin (6 decimals)
Currency::dai()   // Dai (18 decimals)

// Custom ERC20
Currency::erc20("0xcontract...", 18)
```

## Getting an API Key

1. Visit [Etherscan](https://etherscan.io/)
2. Create an account
3. Go to API-KEYs section
4. Generate a free API key

## License

Licensed under either of:

- MIT license [LICENSE-MIT](LICENSE-MIT)

at your option.

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.
