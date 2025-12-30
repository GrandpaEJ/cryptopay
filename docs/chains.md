# Available Chains

CryptoPay is designed efficiently for **Ethereum (Mainnet & Sepolia)** but is compatible with any EVM blockchain that supports the Etherscan API standard.

## Officially Supported

These chains have built-in helper methods and predefined configurations.

| Chain | Network ID | Base URL | Supported Currency |
|-------|------------|----------|-------------------|
| **Ethereum Mainnet** | 1 | `api.etherscan.io` | ETH, ERC20 |
| **Sepolia Testnet** | 11155111 | `api-sepolia.etherscan.io` | ETH, ERC20 |

### Using Supported Chains

```rust
// Ethereum Mainnet (Default)
let client = EtherscanClient::new("your-api-key")?;

// Sepolia Testnet
let client = EtherscanClient::testnet("your-api-key")?;
```

## Etherscan-Compatible Chains

You can use CryptoPay with other Etherscan variants (BscScan, PolygonScan, Snowtrace, etc.) by providing a custom `base_url` in the configuration.

### Configuration

To use a different chain, configure the client with the appropriate Base URL.

```rust
let config = ClientConfig::builder()
    .api_key("YOUR_BSCSCAN_KEY")
    .base_url("https://api.bscscan.com/api") // Override base URL
    .build()?;

let client = EtherscanClient::with_config(config)?;
```

### Common Compatible APIs

| Chain | Explorer | Base URL | Currency |
|-------|----------|----------|----------|
| **BNB Smart Chain** | BscScan | `https://api.bscscan.com/api` | BNB |
| **Polygon** | PolygonScan | `https://api.polygonscan.com/api` | MATIC |
| **Avalanche C-Chain** | Snowtrace | `https://api.snowtrace.io/api` | AVAX |
| **Fantom** | FTMScan | `https://api.ftmscan.com/api` | FTM |
| **Arbitrum** | Arbiscan | `https://api.arbiscan.io/api` | ETH |
| **Optimism** | Etherscan | `https://api-optimistic.etherscan.io/api` | ETH |

### Example: Connect to BSC

```rust
use cryptopay::{ClientConfig, EtherscanClient, PaymentVerifier, PaymentRequest, Currency};
use rust_decimal::Decimal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Configure for BSC
    let config = ClientConfig::builder()
        .api_key("YOUR_BSCSCAN_API_KEY")
        .base_url("https://api.bscscan.com/api")
        .build()?;

    let client = EtherscanClient::with_config(config)?;
    let verifier = PaymentVerifier::new(client);

    // 2. Create Payment Request (Note: Currency is labeled ETH in the enum, but represents the native gas token)
    // For BSC, Currency::ETH represents BNB.
    let payment = PaymentRequest::eth(
        Decimal::from_str("0.1")?,
        "0xRecipientAddress...",
        15 // BSC usually requires ~15 blocks for safety
    );

    // 3. Verify
    let result = verifier.verify_payment(&payment).await?;
    println!("BSC Payment Result: {:?}", result);

    Ok(())
}
```

> **Note:** When using other chains, `Currency::ETH` refers to the chain's native currency (BNB, MATIC, AVAX, etc.).

## Verification Thresholds

Different chains have different block times and security models. We recommend adjusting `required_confirmations` accordingly:

- **Ethereum**: 12 confirmations (~3 mins)
- **BSC**: 15 confirmations (~45 secs)
- **Polygon**: 128 confirmations (~5 mins) - *High due to frequent reorgs*
- **Arbitrum/Optimism**: 1-2 confirmations (Instant finality on L2, though safe to wait for L1 checkpoint)
