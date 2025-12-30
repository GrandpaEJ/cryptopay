# API Reference

Complete API documentation for CryptoPay.

## Core Types

### `EtherscanClient`

Main client for interacting with Etherscan API.

```rust
impl EtherscanClient {
    pub fn new(api_key: impl Into<String>) -> Result<Self>;
    pub fn testnet(api_key: impl Into<String>) -> Result<Self>;
    pub fn with_config(config: ClientConfig) -> Result<Self>;
    pub async fn clear_cache(&self);
    pub fn cache_stats(&self) -> (u64, u64);
}
```

### `PaymentVerifier`

Verifies payment transactions on the blockchain.

```rust
impl PaymentVerifier {
    pub fn new(client: EtherscanClient) -> Self;
    pub async fn verify_payment(&self, request: &PaymentRequest) -> Result<VerificationResult>;
    pub async fn check_confirmations(&self, tx_hash: &str) -> Result<u64>;
    pub async fn find_matching_transaction(&self, request: &PaymentRequest) -> Result<Option<String>>;
}
```

### `PaymentMonitor`

Monitors payments with callback support.

```rust
impl PaymentMonitor {
    pub fn new(client: EtherscanClient, poll_interval: Duration) -> Self;
    pub fn builder() -> PaymentMonitorBuilder;
    pub async fn start_monitoring<F>(&self, request: PaymentRequest, callback: F) -> Result<()>
        where F: Fn(PaymentStatus) + Send + Sync;
    pub async fn check_payment_status(&self, request: &PaymentRequest) -> Result<PaymentStatus>;
}
```

## Models

### `Currency`

Payment currency type.

```rust
pub enum Currency {
    ETH,
    ERC20 {
        contract_address: String,
        decimals: u8,
    },
}

impl Currency {
    pub fn erc20(contract_address: impl Into<String>, decimals: u8) -> Self;
    pub fn usdt() -> Self;  // 0xdAC17F958D2ee523a2206206994597C13D831ec7
    pub fn usdc() -> Self;  // 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
    pub fn dai() -> Self;   // 0x6B175474E89094C44Da98b954EedeAC495271d0F
}
```

### `PaymentRequest`

Payment request specification.

```rust
pub struct PaymentRequest {
    pub amount: Decimal,
    pub currency: Currency,
    pub recipient_address: String,
    pub required_confirmations: u64,
    pub timeout_seconds: Option<u64>,
}

impl PaymentRequest {
    pub fn eth(amount: Decimal, recipient_address: impl Into<String>, required_confirmations: u64) -> Self;
    pub fn token(amount: Decimal, contract_address: impl Into<String>, decimals: u8, recipient_address: impl Into<String>, required_confirmations: u64) -> Self;
    pub fn with_timeout(self, timeout_seconds: u64) -> Self;
    pub fn is_expired(&self, created_at: DateTime<Utc>) -> bool;
}
```

### `PaymentStatus`

Payment status enumeration.

```rust
pub enum PaymentStatus {
    Pending,
    Detected {
        confirmations: u64,
        tx_hash: String,
    },
    Confirmed {
        tx_hash: String,
        confirmations: u64,
    },
    Failed {
        reason: String,
    },
    Expired,
}

impl PaymentStatus {
    pub fn is_finalized(&self) -> bool;
    pub fn is_successful(&self) -> bool;
}
```

### `VerificationResult`

Payment verification result.

```rust
pub enum VerificationResult {
    NotFound,
    Pending {
        tx_hash: String,
        confirmations: u64,
    },
    Confirmed {
        tx_hash: String,
        confirmations: u64,
    },
    Failed {
        reason: String,
    },
}
```

### `Payment`

Complete payment record.

```rust
pub struct Payment {
    pub id: Uuid,
    pub request: PaymentRequest,
    pub status: PaymentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

impl Payment {
    pub fn new(request: PaymentRequest) -> Self;
    pub fn update_status(&mut self, status: PaymentStatus);
    pub fn is_expired(&self) -> bool;
    pub fn with_metadata(self, metadata: serde_json::Value) -> Self;
}
```

## Configuration

### `ClientConfig`

Client configuration.

```rust
pub struct ClientConfig {
    pub api_keys: Vec<String>,
    pub base_url: String,
    pub rate_limit_per_second: u32,
    pub timeout_seconds: u64,
    pub cache_ttl_seconds: u64,
    pub cache_max_size: u64,
}

impl ClientConfig {
    pub fn new(api_key: impl Into<String>) -> Self;
    pub fn testnet(api_key: impl Into<String>) -> Self;
    pub fn from_env() -> Result<Self>;
    pub fn builder() -> ClientConfigBuilder;
    pub fn validate(&self) -> Result<()>;
    pub fn timeout(&self) -> Duration;
    pub fn cache_ttl(&self) -> Duration;
}
```

### `ClientConfigBuilder`

Builder for `ClientConfig`.

```rust
impl ClientConfigBuilder {
    pub fn api_key(self, key: impl Into<String>) -> Self;
    pub fn base_url(self, url: impl Into<String>) -> Self;
    pub fn rate_limit(self, limit: u32) -> Self;
    pub fn timeout(self, seconds: u64) -> Self;
    pub fn cache_ttl(self, seconds: u64) -> Self;
    pub fn cache_max_size(self, size: u64) -> Self;
    pub fn testnet(self) -> Self;
    pub fn build(self) -> Result<ClientConfig>;
}
```

## Error Types

### `Error`

Main error type.

```rust
pub enum Error {
    ApiError(String),
    HttpRequest(reqwest::Error),
    Serialization(serde_json::Error),
    RateLimitExceeded,
    CacheError(String),
    InvalidConfig(String),
    InvalidAddress(String),
    InvalidTransactionHash(String),
    PaymentNotFound,
    PaymentVerificationFailed(String),
}

impl Error {
    pub fn api_error(message: impl Into<String>) -> Self;
    pub fn generic(message: impl Into<String>) -> Self;
}
```

## Utility Functions

### Amount Conversions

```rust
pub fn wei_to_ether(wei: u128) -> Decimal;
pub fn ether_to_wei(ether: Decimal) -> u128;
pub fn gwei_to_wei(gwei: Decimal) -> u128;
pub fn wei_to_gwei(wei: u128) -> Decimal;
pub fn token_to_raw(amount: Decimal, decimals: u8) -> u128;
pub fn raw_to_token(raw_amount: u128, decimals: u8) -> Decimal;
```

### Validation

```rust
pub fn is_valid_address(address: &str) -> bool;
pub fn is_valid_tx_hash(hash: &str) -> bool;
pub fn amounts_match(expected: Decimal, actual: Decimal, tolerance_percent: Decimal) -> bool;
pub fn amount_sufficient(expected: Decimal, actual: Decimal, min_percent: Decimal) -> bool;
```

## Endpoint Traits

### `AccountEndpoints`

```rust
#[async_trait]
pub trait AccountEndpoints {
    async fn get_balance(&self, address: &str) -> Result<Balance>;
    async fn get_transactions(&self, address: &str, start_block: u64, end_block: u64, page: u32, offset: u32, sort: &str) -> Result<Vec<Transaction>>;
    async fn get_internal_transactions(&self, address: &str, start_block: u64, end_block: u64, page: u32, offset: u32, sort: &str) -> Result<Vec<InternalTransaction>>;
}
```

### `TransactionEndpoints`

```rust
#[async_trait]
pub trait TransactionEndpoints {
    async fn get_transaction(&self, tx_hash: &str) -> Result<Transaction>;
    async fn get_transaction_receipt(&self, tx_hash: &str) -> Result<TransactionReceipt>;
    async fn get_confirmations(&self, tx_hash: &str) -> Result<u64>;
    async fn get_block_number(&self) -> Result<u64>;
}
```

### `TokenEndpoints`

```rust
#[async_trait]
pub trait TokenEndpoints {
    async fn get_token_transfers(&self, address: &str, contract_address: Option<&str>, start_block: u64, end_block: u64, page: u32, offset: u32, sort: &str) -> Result<Vec<TokenTransfer>>;
    async fn get_token_balance(&self, address: &str, contract_address: &str) -> Result<TokenBalance>;
}
```

### `GasEndpoints`

```rust
#[async_trait]
pub trait GasEndpoints {
    async fn get_gas_oracle(&self) -> Result<GasOracle>;
    async fn estimate_gas_price(&self, speed: GasSpeed) -> Result<Decimal>;
}
```

## Response Types

### `Transaction`

```rust
pub struct Transaction {
    pub block_number: String,
    pub time_stamp: String,
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas: String,
    pub gas_price: String,
    pub is_error: String,
    pub confirmations: String,
}

impl Transaction {
    pub fn value_bnb(&self) -> Decimal;
    pub fn is_successful(&self) -> bool;
    pub fn confirmations_u64(&self) -> u64;
}
```

### `TokenTransfer`

```rust
pub struct TokenTransfer {
    pub block_number: String,
    pub time_stamp: String,
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub contract_address: String,
    pub token_name: String,
    pub token_symbol: String,
    pub token_decimal: String,
    pub confirmations: String,
}

impl TokenTransfer {
    pub fn value_tokens(&self) -> Decimal;
    pub fn confirmations_u64(&self) -> u64;
}
```

### `Balance`

```rust
pub struct Balance {
    pub account: String,
    pub balance: String,
}

impl Balance {
    pub fn bnb(&self) -> Decimal;
    pub fn wei(&self) -> u128;
}
```

### `GasOracle`

```rust
pub struct GasOracle {
    #[serde(rename = "SafeGasPrice")]
    pub safe_gas_price: String,
    #[serde(rename = "ProposeGasPrice")]
    pub propose_gas_price: String,
    #[serde(rename = "FastGasPrice")]
    pub fast_gas_price: String,
}

impl GasOracle {
    pub fn safe_gwei(&self) -> Decimal;
    pub fn propose_gwei(&self) -> Decimal;
    pub fn fast_gwei(&self) -> Decimal;
}
```

### `GasSpeed`

```rust
pub enum GasSpeed {
    Safe,
    Propose,
    Fast,
}
```
