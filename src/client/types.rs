//! Type definitions for Etherscan API responses

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Transaction information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub block_number: String,
    pub time_stamp: String,
    pub hash: String,
    pub nonce: String,
    pub block_hash: String,
    pub transaction_index: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas: String,
    pub gas_price: String,
    pub is_error: String,
    #[serde(rename = "txreceipt_status")]
    pub txreceipt_status: String,
    pub input: String,
    pub contract_address: String,
    pub cumulative_gas_used: String,
    pub gas_used: String,
    pub confirmations: String,
    #[serde(default)]
    pub method_id: String,
    #[serde(default)]
    pub function_name: String,
}

impl Transaction {
    /// Get confirmations as u64
    pub fn confirmations_u64(&self) -> u64 {
        self.confirmations.parse().unwrap_or(0)
    }

    /// Get value as Decimal (in BNB)
    pub fn value_bnb(&self) -> Decimal {
        let wei: u128 = self.value.parse().unwrap_or(0);
        Decimal::from(wei) / Decimal::from(1_000_000_000_000_000_000u128)
    }

    /// Check if transaction was successful
    pub fn is_successful(&self) -> bool {
        self.is_error == "0" && self.txreceipt_status == "1"
    }
}

/// Internal transaction (contract internal transfers)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransaction {
    pub block_number: String,
    pub time_stamp: String,
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub contract_address: String,
    pub input: String,
    #[serde(rename = "type")]
    pub tx_type: String,
    pub gas: String,
    pub gas_used: String,
    pub trace_id: String,
    pub is_error: String,
    pub err_code: String,
}

/// ERC20 token transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenTransfer {
    pub block_number: String,
    pub time_stamp: String,
    pub hash: String,
    pub nonce: String,
    pub block_hash: String,
    pub from: String,
    pub contract_address: String,
    pub to: String,
    pub value: String,
    pub token_name: String,
    pub token_symbol: String,
    pub token_decimal: String,
    pub transaction_index: String,
    pub gas: String,
    pub gas_price: String,
    pub gas_used: String,
    pub cumulative_gas_used: String,
    pub input: String,
    pub confirmations: String,
}

impl TokenTransfer {
    /// Get confirmations as u64
    pub fn confirmations_u64(&self) -> u64 {
        self.confirmations.parse().unwrap_or(0)
    }

    /// Get token decimals as u8
    pub fn decimals(&self) -> u8 {
        self.token_decimal.parse().unwrap_or(18)
    }

    /// Get value as Decimal (in token units)
    pub fn value_tokens(&self) -> Decimal {
        let raw_value: u128 = self.value.parse().unwrap_or(0);
        let decimals = self.decimals();
        let divisor = 10u128.pow(decimals as u32);
        Decimal::from(raw_value) / Decimal::from(divisor)
    }
}

/// Account balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    #[serde(rename = "balance")]
    pub wei: String,
}

impl Balance {
    /// Get balance as Decimal (in BNB)
    pub fn bnb(&self) -> Decimal {
        let wei: u128 = self.wei.parse().unwrap_or(0);
        Decimal::from(wei) / Decimal::from(1_000_000_000_000_000_000u128)
    }
}

/// Token balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBalance {
    pub contract_address: String,
    pub token_name: String,
    pub token_symbol: String,
    pub token_decimal: String,
    pub balance: String,
}

impl TokenBalance {
    /// Get balance as Decimal (in token units)
    pub fn value_tokens(&self) -> Decimal {
        let raw_value: u128 = self.balance.parse().unwrap_or(0);
        let decimals: u8 = self.token_decimal.parse().unwrap_or(18);
        let divisor = 10u128.pow(decimals as u32);
        Decimal::from(raw_value) / Decimal::from(divisor)
    }
}

/// Transaction receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceipt {
    pub block_hash: String,
    pub block_number: String,
    pub contract_address: Option<String>,
    pub cumulative_gas_used: String,
    pub gas_used: String,
    pub logs: Vec<Log>,
    pub status: String,
    pub transaction_hash: String,
    pub transaction_index: String,
}

/// Transaction log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    pub block_number: String,
    pub transaction_hash: String,
    pub transaction_index: String,
    pub block_hash: String,
    pub log_index: String,
    pub removed: bool,
}

/// Gas oracle information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GasOracle {
    pub safe_gas_price: String,
    pub propose_gas_price: String,
    pub fast_gas_price: String,
    #[serde(rename = "suggestBaseFee")]
    pub suggest_base_fee: String,
    pub gas_used_ratio: String,
}

impl GasOracle {
    /// Get safe gas price in gwei
    pub fn safe_gwei(&self) -> Decimal {
        self.safe_gas_price.parse().unwrap_or(Decimal::ZERO)
    }

    /// Get proposed gas price in gwei
    pub fn propose_gwei(&self) -> Decimal {
        self.propose_gas_price.parse().unwrap_or(Decimal::ZERO)
    }

    /// Get fast gas price in gwei
    pub fn fast_gwei(&self) -> Decimal {
        self.fast_gas_price.parse().unwrap_or(Decimal::ZERO)
    }
}

/// Block information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub number: String,
    pub timestamp: String,
    pub hash: String,
    pub parent_hash: String,
    pub nonce: String,
    pub miner: String,
    pub difficulty: String,
    pub total_difficulty: String,
    pub size: String,
    pub gas_limit: String,
    pub gas_used: String,
    pub transaction_count: usize,
}

/// Block number response (simple string)
pub type BlockNumber = String;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_conversion() {
        let balance = Balance {
            wei: "1000000000000000000".to_string(), // 1 BNB in wei
        };
        assert_eq!(balance.bnb(), Decimal::from(1));
    }

    #[test]
    fn test_transaction_value_conversion() {
        let mut tx = Transaction {
            value: "500000000000000000".to_string(), // 0.5 BNB
            confirmations: "15".to_string(),
            is_error: "0".to_string(),
            txreceipt_status: "1".to_string(),
            block_number: String::new(),
            time_stamp: String::new(),
            hash: String::new(),
            nonce: String::new(),
            block_hash: String::new(),
            transaction_index: String::new(),
            from: String::new(),
            to: String::new(),
            gas: String::new(),
            gas_price: String::new(),
            input: String::new(),
            contract_address: String::new(),
            cumulative_gas_used: String::new(),
            gas_used: String::new(),
            method_id: String::new(),
            function_name: String::new(),
        };

        assert_eq!(tx.value_bnb(), Decimal::new(5, 1)); // 0.5
        assert_eq!(tx.confirmations_u64(), 15);
        assert!(tx.is_successful());
    }
}
