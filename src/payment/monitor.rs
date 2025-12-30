//! Payment monitoring with callbacks

use crate::client::BscScanClient;
use crate::error::Result;
use crate::payment::models::{PaymentRequest, PaymentStatus};
use crate::payment::verification::{PaymentVerifier, VerificationResult};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// Payment monitor with background polling
pub struct PaymentMonitor {
    verifier: PaymentVerifier,
    poll_interval: Duration,
}

impl PaymentMonitor {
    /// Create a new payment monitor
    pub fn new(client: BscScanClient, poll_interval: Duration) -> Self {
        Self {
            verifier: PaymentVerifier::new(client),
            poll_interval,
        }
    }

    /// Create a builder for PaymentMonitor
    pub fn builder() -> PaymentMonitorBuilder {
        PaymentMonitorBuilder::default()
    }

    /// Start monitoring a payment with a callback
    ///
    /// This will poll the blockchain at regular intervals and call the callback
    /// with status updates until the payment is finalized.
    ///
    /// # Example
    /// ```no_run
    /// # use cryptopay::*;
    /// # use std::time::Duration;
    /// # async fn example() -> Result<()> {
    /// let client = BscScanClient::new("api-key")?;
    /// let monitor = PaymentMonitor::new(client, Duration::from_secs(10));
    ///
    /// let payment_request = PaymentRequest::bnb(
    ///     rust_decimal::Decimal::new(1, 1), // 0.1 BNB
    ///     "0x...",
    ///     12,
    /// );
    ///
    /// monitor.start_monitoring(payment_request, |status| {
    ///     println!("Payment status: {:?}", status);
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start_monitoring<F>(&self, request: PaymentRequest, callback: F) -> Result<()>
    where
        F: Fn(PaymentStatus) + Send + Sync,
    {
        let callback = Arc::new(callback);
        let mut last_status: Option<PaymentStatus> = None;

        loop {
            // Check payment status
            let result = self.verifier.verify_payment(&request).await?;

            let current_status = match result {
                VerificationResult::NotFound => PaymentStatus::Pending,
                VerificationResult::Pending {
                    tx_hash,
                    confirmations,
                } => PaymentStatus::Detected {
                    tx_hash,
                    confirmations,
                },
                VerificationResult::Confirmed {
                    tx_hash,
                    confirmations,
                } => PaymentStatus::Confirmed {
                    tx_hash,
                    confirmations,
                },
                VerificationResult::Failed { reason } => PaymentStatus::Failed { reason },
            };

            // Call callback if status changed
            if last_status.as_ref() != Some(&current_status) {
                callback(current_status.clone());
                last_status = Some(current_status.clone());
            }

            // Break if finalized
            if current_status.is_finalized() {
                break;
            }

            // Check for timeout
            // Note: In real usage, you'd want to track creation time
            // For now, we rely on the user to handle timeouts externally

            // Wait before next poll
            sleep(self.poll_interval).await;
        }

        Ok(())
    }

    /// Check payment status once (no monitoring)
    pub async fn check_payment_status(&self, request: &PaymentRequest) -> Result<PaymentStatus> {
        let result = self.verifier.verify_payment(request).await?;

        Ok(match result {
            VerificationResult::NotFound => PaymentStatus::Pending,
            VerificationResult::Pending {
                tx_hash,
                confirmations,
            } => PaymentStatus::Detected {
                tx_hash,
                confirmations,
            },
            VerificationResult::Confirmed {
                tx_hash,
                confirmations,
            } => PaymentStatus::Confirmed {
                tx_hash,
                confirmations,
            },
            VerificationResult::Failed { reason } => PaymentStatus::Failed { reason },
        })
    }
}

/// Builder for PaymentMonitor
#[derive(Default)]
pub struct PaymentMonitorBuilder {
    client: Option<BscScanClient>,
    poll_interval: Option<Duration>,
}

impl PaymentMonitorBuilder {
    /// Set the BscScan client
    pub fn client(mut self, client: BscScanClient) -> Self {
        self.client = Some(client);
        self
    }

    /// Set the poll interval
    pub fn poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = Some(interval);
        self
    }

    /// Build the PaymentMonitor
    pub fn build(self) -> PaymentMonitor {
        PaymentMonitor::new(
            self.client.expect("BscScanClient is required"),
            self.poll_interval.unwrap_or(Duration::from_secs(10)),
        )
    }
}
