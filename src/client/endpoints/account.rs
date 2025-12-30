//! Account-related API endpoints

use crate::client::types::{Balance, InternalTransaction, Transaction};
use crate::client::BscScanClient;
use crate::error::Result;

/// Account endpoints
pub trait AccountEndpoints {
    /// Get BNB balance for an address
    ///
    /// # Example
    /// ```no_run
    /// # use cryptopay::*;
    /// # async fn example() -> Result<()> {
    /// let client = BscScanClient::new("api-key")?;
    /// let balance = client.get_balance("0x...").await?;
    /// println!("Balance: {} BNB", balance.bnb());
    /// # Ok(())
    /// # }
    /// ```
    async fn get_balance(&self, address: &str) -> Result<Balance>;

    /// Get list of transactions for an address
    ///
    /// # Parameters
    /// - `address`: The address to get transactions for
    /// - `start_block`: Starting block number (0 for all)
    /// - `end_block`: Ending block number (99999999 for latest)
    /// - `page`: Page number (1-indexed)
    /// - `offset`: Number of transactions per page (max 10000)
    /// - `sort`: "asc" or "desc"
    async fn get_transactions(
        &self,
        address: &str,
        start_block: u64,
        end_block: u64,
        page: u32,
        offset: u32,
        sort: &str,
    ) -> Result<Vec<Transaction>>;

    /// Get list of internal transactions for an address
    async fn get_internal_transactions(
        &self,
        address: &str,
        start_block: u64,
        end_block: u64,
        page: u32,
        offset: u32,
        sort: &str,
    ) -> Result<Vec<InternalTransaction>>;
}

impl AccountEndpoints for BscScanClient {
    async fn get_balance(&self, address: &str) -> Result<Balance> {
        let params = [("address", address), ("tag", "latest")];

        // BscScan returns balance as a simple string, wrap it
        let balance_str: String = self.request_simple("account", "balance", &params).await?;

        Ok(Balance { wei: balance_str })
    }

    async fn get_transactions(
        &self,
        address: &str,
        start_block: u64,
        end_block: u64,
        page: u32,
        offset: u32,
        sort: &str,
    ) -> Result<Vec<Transaction>> {
        let params = [
            ("address", address),
            ("startblock", &start_block.to_string()),
            ("endblock", &end_block.to_string()),
            ("page", &page.to_string()),
            ("offset", &offset.to_string()),
            ("sort", sort),
        ];

        self.request("account", "txlist", &params).await
    }

    async fn get_internal_transactions(
        &self,
        address: &str,
        start_block: u64,
        end_block: u64,
        page: u32,
        offset: u32,
        sort: &str,
    ) -> Result<Vec<InternalTransaction>> {
        let params = [
            ("address", address),
            ("startblock", &start_block.to_string()),
            ("endblock", &end_block.to_string()),
            ("page", &page.to_string()),
            ("offset", &offset.to_string()),
            ("sort", sort),
        ];

        self.request("account", "txlistinternal", &params).await
    }
}
