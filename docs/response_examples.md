# Etherscan API Response Examples

This document lists the actual JSON structures and Rust debug outputs for various Etherscan API V2 endpoints on Ethereum Mainnet.

## Account Balance

**Method:** `get_balance`
**Endpoint:** `account` > `balance`

```rust
Balance {
    wei: "4054000000000000",
}
```

## Transaction List

**Method:** `get_transactions`
**Endpoint:** `account` > `txlist`

```rust
Transaction {
    block_number: "23697845",
    time_stamp: "1761919835",
    hash: "0x65a0983ecc0797cfb43cec86c667429f0bf43ee85a8ce09cbeebe3037fc82b46",
    nonce: "6",
    block_hash: "0x3de5aa35a67f35817a9832255ae1bcf3dd61bf87997e38587b77df0bf0311583",
    transaction_index: "166",
    from: "0xd565b3a7166495183a25ad3381f8d01c58c33ab7",
    to: "0x742d35cc6634c0532925a3b844bc9e7595f0beb0",
    value: "217000000000000", // 0.000217 ETH
    gas: "21000",
    gas_price: "2409786389",
    is_error: "0",
    txreceipt_status: "1", // Success
    input: "0x",
    contract_address: "",
    cumulative_gas_used: "19314678",
    gas_used: "21000",
    confirmations: "428139",
    method_id: "0x",
    function_name: "",
}
```

## Single Transaction (Proxy)

**Method:** `get_transaction`
**Endpoint:** `proxy` > `eth_getTransactionByHash`

Note: Proxy endpoints return Hex strings and fewer enriched fields.

```rust
Transaction {
    block_number: "23697845",
    time_stamp: "", // Not available in proxy response
    hash: "0x65a0983ecc0797cfb43cec86c667429f0bf43ee85a8ce09cbeebe3037fc82b46",
    nonce: "6",
    block_hash: "0x3de5aa35a67f35817a9832255ae1bcf3dd61bf87997e38587b77df0bf0311583",
    transaction_index: "166",
    from: "0xd565b3a7166495183a25ad3381f8d01c58c33ab7",
    to: "0x742d35cc6634c0532925a3b844bc9e7595f0beb0",
    value: "217000000000000", // Converted from Hex 0xc55c3fea9000
    gas: "21000",
    gas_price: "2409786389",
    is_error: "0",
    txreceipt_status: "", // Not available in proxy response
    input: "0x",
    contract_address: "",
    cumulative_gas_used: "",
    gas_used: "",
    confirmations: "0", // Not available in proxy response
    method_id: "",
    function_name: "",
}
```

## Transaction Receipt (Proxy)

**Method:** `get_transaction_receipt`
**Endpoint:** `proxy` > `eth_getTransactionReceipt`

```rust
TransactionReceipt {
    block_hash: "0x3de5aa35a67f35817a9832255ae1bcf3dd61bf87997e38587b77df0bf0311583",
    block_number: "0x16999b5",
    contract_address: None,
    cumulative_gas_used: "0x126b7f6",
    gas_used: "0x5208",
    logs: [],
    status: "0x1", // Success
    transaction_hash: "0x65a0983ecc0797cfb43cec86c667429f0bf43ee85a8ce09cbeebe3037fc82b46",
    transaction_index: "0xa6",
}
```

## Token Transfer (ERC20)

**Method:** `get_token_transfers`
**Endpoint:** `account` > `tokentx`

```rust
TokenTransfer {
    block_number: "23974375",
    time_stamp: "1765273091",
    hash: "0x1aea105691afc60e5afbda8abaef2eaa4d19edf6064ee087f6d566e7cf157aa7",
    nonce: "145",
    block_hash: "0x47dff786283537f56dee17b8c36679c2b31df0c808d71166c34ca506215169c2",
    from: "0x4c2c0f0bb2631b02ac9299c59690914ee7a200b8",
    contract_address: "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
    to: "0x742d35cc6634c0532925a3b844bc9e7595f0beb0",
    value: "1000000", // 1.000000 USDC
    token_name: "USDC",
    token_symbol: "USDC",
    token_decimal: "6",
    transaction_index: "180",
    gas: "108392",
    gas_price: "262807005",
    gas_used: "53401",
    cumulative_gas_used: "16263632",
    input: "deprecated",
    confirmations: "151619",
}
```

## Token Balance

**Method:** `get_token_balance`
**Endpoint:** `account` > `tokenbalance`

```rust
TokenBalance {
    contract_address: "0xdAC17F958D2ee523a2206206994597C13D831ec7",
    token_name: "", // Not returned by V2 API
    token_symbol: "", // Not returned by V2 API
    token_decimal: "18", // Default
    balance: "0",
}
```

## Gas Oracle

**Method:** `get_gas_oracle`
**Endpoint:** `gastracker` > `gasoracle`

```rust
GasOracle {
    safe_gas_price: "20.5",
    propose_gas_price: "22.1",
    fast_gas_price: "25.0",
    suggest_base_fee: "19.8",
    gas_used_ratio: "0.5,0.4,0.3,0.9,0.8",
}
```
