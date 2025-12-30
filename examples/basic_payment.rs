//! Basic payment verification example

use cryptopay::{BscScanClient, Currency, PaymentRequest, PaymentVerifier, VerificationResult};
use rust_decimal::Decimal;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = std::env::var("BSCSCAN_API_KEY")
        .expect("BSCSCAN_API_KEY environment variable not set");

    // Create BscScan client
    let client = BscScanClient::new(api_key)?;
    let verifier = PaymentVerifier::new(client);

    // Create a payment request for 0.1 BNB
    let payment_request = PaymentRequest {
        amount: Decimal::from_str("0.1")?,
        currency: Currency::BNB,
        recipient_address: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb0".to_string(),
        required_confirmations: 12,
        timeout_seconds: Some(1800), // 30 minutes
    };

    println!("Checking for payment to {}", payment_request.recipient_address);
    println!("Expected amount: {} BNB", payment_request.amount);
    println!("Required confirmations: {}", payment_request.required_confirmations);
    println!();

    // Verify the payment
    match verifier.verify_payment(&payment_request).await? {
        VerificationResult::Confirmed {
            tx_hash,
            confirmations,
        } => {
            println!("✓ Payment confirmed!");
            println!("  Transaction: {}", tx_hash);
            println!("  Confirmations: {}", confirmations);
        }
        VerificationResult::Pending {
            tx_hash,
            confirmations,
        } => {
            println!("⏳ Payment detected but pending confirmations");
            println!("  Transaction: {}", tx_hash);
            println!("  Confirmations: {}/{}", confirmations, payment_request.required_confirmations);
        }
        VerificationResult::NotFound => {
            println!("✗ No matching payment found");
        }
        VerificationResult::Failed { reason } => {
            println!("✗ Payment verification failed: {}", reason);
        }
    }

    Ok(())
}
