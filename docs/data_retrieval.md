# Data Retrieval Guide

CryptoPay provides direct access to the Etherscan API for retrieving raw blockchain data. This is useful for building wallets, analytics dashboards, or custom explorers.

## Core Concepts

Data retrieval is handled by the `EtherscanClient` through specific traits:
- `AccountEndpoints`: Balances and transaction history
- `TransactionEndpoints`: Individual transaction details and receipts
- `TokenEndpoints`: ERC20 token balances and transfers
- `GasEndpoints`: Gas prices and estimation

## 1. Retrieving Account Data

### Get Native Balance (ETH)

```rust
let balance = client.get_balance("0xUserAddress...").await?;

println!("Balance in Wei: {}", balance.wei());
println!("Balance in ETH: {}", balance.bnb()); // Helper returns Decimal
```

### Get Transaction History

Retrieve headers of normal transactions.

```rust
let history = client.get_transactions(
    "0xUserAddress...",
    0,          // Start Block (0 for genesis)
    99999999,   // End Block
    1,          // Page number
    100,        // Transactions per page & offset
    "desc"      // Sort order: "asc" or "desc"
).await?;

for tx in history {
    println!("Tx Hash: {}", tx.hash);
    println!("Value: {} ETH", tx.value_bnb());
    println!("From: {}, To: {}", tx.from, tx.to);
}
```

### Get Internal Transactions

Useful for detecting contract interactions and transfers not visible in standard tx list.

```rust
let internal_txs = client.get_internal_transactions(
    "0xUserAddress...",
    0, 99999999, 1, 100, "desc"
).await?;
```

## 2. Retrieving Token Data (ERC20)

### Get Token Balance

Check balance of a specific ERC20 token for an address.

```rust
let usdt_contract = "0xdAC17F958D2ee523a2206206994597C13D831ec7";
let user = "0xUserAddress...";

let token_balance = client.get_token_balance(user, usdt_contract).await?;

println!("USDT Balance: {}", token_balance.balance); // Returns raw amount (check decimals!)
```

### Get Token Transfers

Get history of ERC20 token transfers.

```rust
let transfers = client.get_token_transfers(
    "0xUserAddress...",
    Some(usdt_contract), // Filter by contract (Optional: None for all tokens)
    0,
    99999999,
    1,
    100,
    "desc"
).await?;

for transfer in transfers {
    println!("Token: {}", transfer.token_symbol);
    println!("Amount: {}", transfer.value);
    println!("Tx: {}", transfer.hash);
}
```

## 3. Retrieving Transaction Details

### Get Transaction by Hash

Verify details of a specific transaction.

```rust
let tx = client.get_transaction("0xTxHash...").await?;

println!("Block: {}", tx.block_number);
println!("Gas Used: {}", tx.gas);
println!("Nonce: {}", tx.nonce);
```

### Get Transaction Receipt

Check status (success/fail) and logs.

```rust
let receipt = client.get_transaction_receipt("0xTxHash...").await?;

if receipt.status == "1" {
    println!("Transaction Successful");
} else {
    println!("Transaction Failed");
}
```

## 4. Retrieving Gas Data

### Get Gas Oracle

Current gas prices for different speeds.

```rust
let oracle = client.get_gas_oracle().await?;

println!("Safe Low: {} gwei", oracle.safe_gwei());
println!("Standard: {} gwei", oracle.propose_gwei());
println!("Fast: {} gwei", oracle.fast_gwei());
```

### Estimate Gas Price

Helpers to get a specific price directly.

```rust
use cryptopay::client::GasSpeed;

let fast_price = client.estimate_gas_price(GasSpeed::Fast).await?;
```

## Error Handling

All retrieval methods return `Result<T, Error>`.

```rust
match client.get_transaction("0xInvalidHash").await {
    Ok(tx) => process(tx),
    Err(Error::ApiError(msg)) => println!("Etherscan Error: {}", msg),
    Err(Error::RateLimitExceeded) => println!("Slow down!"),
    Err(e) => println!("Unknown error: {}", e),
}
```
