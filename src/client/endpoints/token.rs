//! Token-related API endpoints

use crate::client::types::{TokenBalance, TokenTransfer};
use crate::client::BscScanClient;
use crate::error::Result;

/// Token endpoints
pub trait TokenEndpoints {
    /// Get BEP20 token transfers for an address
    ///
    /// # Parameters
    /// - `address`: The address to get token transfers for
    /// - `contract_address`: Optional token contract address to filter by
    /// - `start_block`: Starting block number (0 for all)
    /// - `end_block`: Ending block number (99999999 for latest)
    /// - `page`: Page number (1-indexed)
    /// - `offset`: Number of transfers per page (max 10000)
    /// - `sort`: "asc" or "desc"
    async fn get_token_transfers(
        &self,
        address: &str,
        contract_address: Option<&str>,
        start_block: u64,
        end_block: u64,
        page: u32,
        offset: u32,
        sort: &str,
    ) -> Result<Vec<TokenTransfer>>;

    /// Get BEP20 token balance for an address
    async fn get_token_balance(&self, address: &str, contract_address: &str) -> Result<TokenBalance>;
}

impl TokenEndpoints for BscScanClient {
    async fn get_token_transfers(
        &self,
        address: &str,
        contract_address: Option<&str>,
        start_block: u64,
        end_block: u64,
        page: u32,
        offset: u32,
        sort: &str,
    ) -> Result<Vec<TokenTransfer>> {
        let mut params = vec![
            ("address", address.to_string()),
            ("startblock", start_block.to_string()),
            ("endblock", end_block.to_string()),
            ("page", page.to_string()),
            ("offset", offset.to_string()),
            ("sort", sort.to_string()),
        ];

        if let Some(contract) = contract_address {
            params.push(("contractaddress", contract.to_string()));
        }

        let params_ref: Vec<(&str, &str)> = params
            .iter()
            .map(|(k, v)| (k.as_ref(), v.as_ref()))
            .collect();

        self.request("account", "tokentx", &params_ref).await
    }

    async fn get_token_balance(&self, address: &str, contract_address: &str) -> Result<TokenBalance> {
        let params = [
            ("contractaddress", contract_address),
            ("address", address),
            ("tag", "latest"),
        ];

        let balance_str: String = self
            .request_simple("account", "tokenbalance", &params)
            .await?;

        // Note: We don't have token metadata from this endpoint
        // In real usage, you might want to call another endpoint or maintain a token list
        Ok(TokenBalance {
            contract_address: contract_address.to_string(),
            token_name: String::new(),
            token_symbol: String::new(),
            token_decimal: "18".to_string(), // Default to 18
            balance: balance_str,
        })
    }
}
