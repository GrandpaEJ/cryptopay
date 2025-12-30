//! Payment verification logic

use crate::client::endpoints::{AccountEndpoints, TokenEndpoints, TransactionEndpoints};
use crate::client::BscScanClient;
use crate::error::{Error, Result};
use crate::payment::models::{Currency, PaymentRequest};
use crate::payment::utils::{amount_sufficient, is_valid_address};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Payment verifier
pub struct PaymentVerifier {
    client: BscScanClient,
}

/// Verification result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VerificationResult {
    /// No matching transaction found
    NotFound,

    /// Transaction found but not yet confirmed
    Pending {
        /// Transaction hash
        tx_hash: String,
        /// Current confirmations
        confirmations: u64,
    },

    /// Payment confirmed
    Confirmed {
        /// Transaction hash
        tx_hash: String,
        /// Final confirmations
        confirmations: u64,
    },

    /// Payment failed verification
    Failed {
        /// Failure reason
        reason: String,
    },
}

impl PaymentVerifier {
    /// Create a new payment verifier
    pub fn new(client: BscScanClient) -> Self {
        Self { client }
    }

    /// Verify a payment request
    ///
    /// This checks if a matching transaction exists on the blockchain and
    /// verifies it meets all requirements (amount, recipient, confirmations).
    pub async fn verify_payment(&self, request: &PaymentRequest) -> Result<VerificationResult> {
        // Validate recipient address
        if !is_valid_address(&request.recipient_address) {
            return Err(Error::InvalidAddress(request.recipient_address.clone()));
        }

        // Find matching transaction based on currency type
        let matching_tx = match &request.currency {
            Currency::ETH => self.find_eth_transaction(request).await?,
            Currency::ERC20 {
                contract_address,
                decimals,
            } => {
                self.find_token_transaction(request, contract_address, *decimals)
                    .await?
            }
        };

        // If no matching transaction, return NotFound
        let (tx_hash, confirmations, actual_amount) = match matching_tx {
            Some(data) => data,
            None => return Ok(VerificationResult::NotFound),
        };

        // Check if amount matches (allow 99.9% minimum to account for dust/rounding)
        let min_percent = Decimal::from_str_radix("99.9", 10).unwrap();
        if !amount_sufficient(request.amount, actual_amount, min_percent) {
            return Ok(VerificationResult::Failed {
                reason: format!(
                    "Amount mismatch: expected {}, got {}",
                    request.amount, actual_amount
                ),
            });
        }

        // Check confirmations
        if confirmations >= request.required_confirmations {
            Ok(VerificationResult::Confirmed {
                tx_hash,
                confirmations,
            })
        } else {
            Ok(VerificationResult::Pending {
                tx_hash,
                confirmations,
            })
        }
    }

    /// Find matching ETH transaction
    async fn find_eth_transaction(
        &self,
        request: &PaymentRequest,
    ) -> Result<Option<(String, u64, Decimal)>> {
        // Get recent transactions to the recipient address
        let transactions = self
            .client
            .get_transactions(&request.recipient_address, 0, 99999999, 1, 100, "desc")
            .await?;

        // Find matching transaction
        for tx in transactions {
            // Skip failed transactions
            if !tx.is_successful() {
                continue;
            }

            let tx_value = tx.value_bnb();

            // Check if amount matches (within tolerance)
            if amount_sufficient(request.amount, tx_value, Decimal::new(999, 1)) {
                let confirmations = tx.confirmations_u64();
                return Ok(Some((tx.hash, confirmations, tx_value)));
            }
        }

        Ok(None)
    }

    /// Find matching ERC20 token transaction
    async fn find_token_transaction(
        &self,
        request: &PaymentRequest,
        contract_address: &str,
        _decimals: u8,
    ) -> Result<Option<(String, u64, Decimal)>> {
        // Get recent token transfers to the recipient address
        let transfers = self
            .client
            .get_token_transfers(
                &request.recipient_address,
                Some(contract_address),
                0,
                99999999,
                1,
                100,
                "desc",
            )
            .await?;

        // Find matching transfer
        for transfer in transfers {
            let tx_value = transfer.value_tokens();

            // Check if amount matches (within tolerance)
            if amount_sufficient(request.amount, tx_value, Decimal::new(999, 1)) {
                let confirmations = transfer.confirmations_u64();
                return Ok(Some((transfer.hash, confirmations, tx_value)));
            }
        }

        Ok(None)
    }

    /// Check confirmations for a specific transaction hash
    pub async fn check_confirmations(&self, tx_hash: &str) -> Result<u64> {
        self.client.get_confirmations(tx_hash).await
    }

    /// Find any matching transaction for a payment request
    ///
    /// Returns the transaction hash if found
    pub async fn find_matching_transaction(&self, request: &PaymentRequest) -> Result<Option<String>> {
        let result = self.verify_payment(request).await?;

        match result {
            VerificationResult::Confirmed { tx_hash, .. } => Ok(Some(tx_hash)),
            VerificationResult::Pending { tx_hash, .. } => Ok(Some(tx_hash)),
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_result() {
        let result = VerificationResult::Confirmed {
            tx_hash: "0x123".to_string(),
            confirmations: 15,
        };

        match result {
            VerificationResult::Confirmed {
                confirmations,
                ..
            } => {
                assert_eq!(confirmations, 15);
            }
            _ => panic!("Expected Confirmed"),
        }
    }
}
