//! Transaction-related API endpoints

use crate::client::types::{Transaction, TransactionReceipt};
use crate::client::BscScanClient;
use crate::error::{Error, Result};

/// Transaction endpoints
pub trait TransactionEndpoints {
    /// Get transaction by hash
    async fn get_transaction(&self, tx_hash: &str) -> Result<Transaction>;

    /// Get transaction receipt
    async fn get_transaction_receipt(&self, tx_hash: &str) -> Result<TransactionReceipt>;

    /// Get number of confirmations for a transaction
    async fn get_confirmations(&self, tx_hash: &str) -> Result<u64>;

    /// Get current block number
    async fn get_block_number(&self) -> Result<u64>;
}

impl TransactionEndpoints for BscScanClient {
    async fn get_transaction(&self, tx_hash: &str) -> Result<Transaction> {
        let params = [("txhash", tx_hash)];

        self.request("proxy", "eth_getTransactionByHash", &params)
            .await
    }

    async fn get_transaction_receipt(&self, tx_hash: &str) -> Result<TransactionReceipt> {
        let params = [("txhash", tx_hash)];

        self.request("proxy", "eth_getTransactionReceipt", &params)
            .await
    }

    async fn get_confirmations(&self, tx_hash: &str) -> Result<u64> {
        // Get transaction to find its block number
        let tx = self.get_transaction(tx_hash).await?;
        let tx_block: u64 = tx
            .block_number
            .parse()
            .map_err(|_| Error::generic("Invalid block number in transaction"))?;

        // Get current block number
        let current_block = self.get_block_number().await?;

        // Calculate confirmations
        if current_block >= tx_block {
            Ok(current_block - tx_block + 1)
        } else {
            Ok(0)
        }
    }

    async fn get_block_number(&self) -> Result<u64> {
        let params: [(&str, &str); 0] = [];
        let block_hex: String = self
            .request_simple("proxy", "eth_blockNumber", &params)
            .await?;

        // Parse hex string (e.g., "0x1a2b3c")
        let block_num = u64::from_str_radix(block_hex.trim_start_matches("0x"), 16)
            .map_err(|_| Error::generic("Invalid block number format"))?;

        Ok(block_num)
    }
}
