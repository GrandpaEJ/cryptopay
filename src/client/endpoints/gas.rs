//! Gas-related API endpoints

use crate::client::types::GasOracle;
use crate::client::BscScanClient;
use crate::error::Result;
use rust_decimal::Decimal;

/// Gas speed options
#[derive(Debug, Clone, Copy)]
pub enum GasSpeed {
    Safe,
    Propose,
    Fast,
}

/// Gas endpoints
pub trait GasEndpoints {
    /// Get gas oracle data
    async fn get_gas_oracle(&self) -> Result<GasOracle>;

    /// Get estimated gas price for a given speed
    async fn estimate_gas_price(&self, speed: GasSpeed) -> Result<Decimal>;
}

impl GasEndpoints for BscScanClient {
    async fn get_gas_oracle(&self) -> Result<GasOracle> {
        let params: [(&str, &str); 0] = [];
        self.request("gastracker", "gasoracle", &params).await
    }

    async fn estimate_gas_price(&self, speed: GasSpeed) -> Result<Decimal> {
        let oracle = self.get_gas_oracle().await?;

        Ok(match speed {
            GasSpeed::Safe => oracle.safe_gwei(),
            GasSpeed::Propose => oracle.propose_gwei(),
            GasSpeed::Fast => oracle.fast_gwei(),
        })
    }
}
