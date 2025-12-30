#![allow(async_fn_in_trait)]

//! # CryptoPay - Etherscan Payment Gateway Library
//!
//! A comprehensive Rust library for integrating with Etherscan API to verify and monitor
//! cryptocurrency payments on Ethereum.
//!
//! ## Features
//!
//! - **Etherscan API Integration**: Full support for accounts, transactions, tokens, and gas tracking
//! - **Payment Verification**: Verify ETH and ERC20 token payments with confirmation tracking
//! - **Payment Monitoring**: Monitor pending payments with callbacks
//! - **Rate Limiting**: Built-in rate limiter respecting Etherscan's 5 req/s limit
//! - **Caching**: In-memory LRU cache to minimize API calls
//! - **Optional Storage**: PostgreSQL and SQLite storage implementations (feature-gated)
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use cryptopay::*;
//! use rust_decimal::Decimal;
//! use std::str::FromStr;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create Etherscan client
//!     let client = EtherscanClient::new("your-api-key")?;
//!     let verifier = PaymentVerifier::new(client);
//!     
//!     // Create payment request
//!     let payment = PaymentRequest {
//!         amount: Decimal::from_str("0.1").unwrap(),
//!         currency: Currency::ETH,
//!         recipient_address: "0x...".to_string(),
//!         required_confirmations: 12,
//!         timeout_seconds: Some(1800),
//!     };
//!     
//!     // Verify payment
//!     match verifier.verify_payment(&payment).await? {
//!         VerificationResult::Confirmed { tx_hash, .. } => {
//!             println!("Payment confirmed: {}", tx_hash);
//!         }
//!         VerificationResult::Pending { confirmations, .. } => {
//!             println!("Waiting for confirmations: {}/{}", 
//!                 confirmations, payment.required_confirmations);
//!         }
//!         _ => println!("No payment found"),
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod config;
pub mod error;
pub mod payment;

#[cfg(feature = "postgres-storage")]
pub mod storage;

// Re-export main types for convenience
pub use client::BscScanClient as EtherscanClient;
pub use client::BscScanClient; // Keep for backward compat
pub use config::ClientConfig;
pub use error::{Error, Result};
pub use payment::{
    Currency, Payment, PaymentMonitor, PaymentRequest, PaymentStatus, PaymentVerifier,
    VerificationResult,
};

#[cfg(feature = "postgres-storage")]
pub use storage::{PaymentStorage, PostgresStorage};

#[cfg(feature = "sqlite-storage")]
pub use storage::SqliteStorage;
