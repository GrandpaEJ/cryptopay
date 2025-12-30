//! Payment models and types

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Payment currency type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Currency {
    /// Native ETH
    ETH,
    /// ERC20 token
    ERC20 {
        /// Token contract address
        contract_address: String,
        /// Token decimals
        decimals: u8,
    },
}

impl Currency {
    /// Create an ERC20 currency
    pub fn erc20(contract_address: impl Into<String>, decimals: u8) -> Self {
        Self::ERC20 {
            contract_address: contract_address.into(),
            decimals,
        }
    }

    /// Common stablecoins on Ethereum
    pub fn usdt() -> Self {
        // Ethereum USDT contract
        Self::ERC20 {
            contract_address: "0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string(),
            decimals: 6, // USDT has 6 decimals on Ethereum
        }
    }

    pub fn usdc() -> Self {
        // Ethereum USDC contract
        Self::ERC20 {
            contract_address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
            decimals: 6, // USDC has 6 decimals
        }
    }

    pub fn dai() -> Self {
        // Ethereum DAI contract
        Self::ERC20 {
            contract_address: "0x6B175474E89094C44Da98b954EedeAC495271d0F".to_string(),
            decimals: 18,
        }
    }
}

/// Payment request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    /// Payment amount (in token/ETH units, not wei)
    pub amount: Decimal,

    /// Currency type
    pub currency: Currency,

    /// Recipient address (where payment should be sent)
    pub recipient_address: String,

    /// Required number of confirmations
    pub required_confirmations: u64,

    /// Optional timeout in seconds (payment expires if not confirmed)
    pub timeout_seconds: Option<u64>,
}

impl PaymentRequest {
    /// Create a new ETH payment request
    pub fn eth(
        amount: Decimal,
        recipient_address: impl Into<String>,
        required_confirmations: u64,
    ) -> Self {
        Self {
            amount,
            currency: Currency::ETH,
            recipient_address: recipient_address.into(),
            required_confirmations,
            timeout_seconds: None,
        }
    }

    /// Create a new ERC20 token payment request
    pub fn token(
        amount: Decimal,
        contract_address: impl Into<String>,
        decimals: u8,
        recipient_address: impl Into<String>,
        required_confirmations: u64,
    ) -> Self {
        Self {
            amount,
            currency: Currency::erc20(contract_address, decimals),
            recipient_address: recipient_address.into(),
            required_confirmations,
            timeout_seconds: None,
        }
    }

    /// Set timeout for the payment
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = Some(timeout_seconds);
        self
    }

    /// Check if payment has expired
    pub fn is_expired(&self, created_at: DateTime<Utc>) -> bool {
        if let Some(timeout) = self.timeout_seconds {
            let elapsed = Utc::now().signed_duration_since(created_at);
            elapsed.num_seconds() as u64 >= timeout
        } else {
            false
        }
    }
}

/// Payment status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PaymentStatus {
    /// Payment is pending (no transaction found yet)
    Pending,

    /// Transaction detected but not yet confirmed
    Detected {
        /// Number of confirmations
        confirmations: u64,
        /// Transaction hash
        tx_hash: String,
    },

    /// Payment confirmed
    Confirmed {
        /// Transaction hash
        tx_hash: String,
        /// Final number of confirmations
        confirmations: u64,
    },

    /// Payment failed
    Failed {
        /// Failure reason
        reason: String,
    },

    /// Payment expired (timeout reached)
    Expired,
}

impl PaymentStatus {
    /// Check if payment is finalized (confirmed, failed, or expired)
    pub fn is_finalized(&self) -> bool {
        matches!(
            self,
            PaymentStatus::Confirmed { .. }
                | PaymentStatus::Failed { .. }
                | PaymentStatus::Expired
        )
    }

    /// Check if payment is successful
    pub fn is_successful(&self) -> bool {
        matches!(self, PaymentStatus::Confirmed { .. })
    }
}

/// Complete payment record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    /// Unique payment ID
    pub id: Uuid,

    /// Payment request details
    pub request: PaymentRequest,

    /// Current status
    pub status: PaymentStatus,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,

    /// Optional metadata (for user's custom data)
    #[serde(default)]
    pub metadata: serde_json::Value,
}

impl Payment {
    /// Create a new payment
    pub fn new(request: PaymentRequest) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            request,
            status: PaymentStatus::Pending,
            created_at: now,
            updated_at: now,
            metadata: serde_json::Value::Null,
        }
    }

    /// Update payment status
    pub fn update_status(&mut self, status: PaymentStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    /// Check if payment has expired
    pub fn is_expired(&self) -> bool {
        self.request.is_expired(self.created_at)
    }

    /// Add custom metadata
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_eth_payment_request() {
        let request = PaymentRequest::eth(
            Decimal::from_str("0.1").unwrap(),
            "0x1234567890123456789012345678901234567890",
            12,
        );

        assert_eq!(request.currency, Currency::ETH);
        assert_eq!(request.required_confirmations, 12);
    }

    #[test]
    fn test_token_payment_request() {
        let request = PaymentRequest::token(
            Decimal::from(100),
            "0xcontract",
            18,
            "0x1234567890123456789012345678901234567890",
            6,
        );

        match request.currency {
            Currency::ERC20 {
                ref contract_address,
                decimals,
            } => {
                assert_eq!(contract_address, "0xcontract");
                assert_eq!(decimals, 18);
            }
            _ => panic!("Expected ERC20 currency"),
        }
    }

    #[test]
    fn test_payment_creation() {
        let request = PaymentRequest::eth(Decimal::from(1), "0xrecipient", 12);
        let payment = Payment::new(request);

        assert_eq!(payment.status, PaymentStatus::Pending);
        assert!(!payment.is_expired());
    }

    #[test]
    fn test_payment_status_finalized() {
        let status = PaymentStatus::Pending;
        assert!(!status.is_finalized());

        let status = PaymentStatus::Confirmed {
            tx_hash: "0xhash".to_string(),
            confirmations: 15,
        };
        assert!(status.is_finalized());
        assert!(status.is_successful());
    }
}
