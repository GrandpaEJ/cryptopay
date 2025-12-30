//! BEP20 token payment verification example

use cryptopay::{BscScanClient, Currency, PaymentRequest, PaymentVerifier, VerificationResult};
use rust_decimal::Decimal;
use std::str::FromStr;

// BSC Mainnet USDT contract address
const USDT_CONTRACT: &str = "0x55d398326f99059fF775485246999027B3197955";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = std::env::var("BSCSCAN_API_KEY")
        .expect("BSCSCAN_API_KEY environment variable not set");

    // Create BscScan client
    let client = BscScanClient::new(api_key)?;
    let verifier = PaymentVerifier::new(client);

    // Create a payment request for 100 USDT
    let payment_request = PaymentRequest {
        amount: Decimal::from_str("100.0")?,
        currency: Currency::BEP20 {
            contract_address: USDT_CONTRACT.to_string(),
            decimals: 18,
        },
        recipient_address: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb0".to_string(),
        required_confirmations: 6, // Fewer confirmations for tokens
        timeout_seconds: Some(3600), // 1 hour
    };

    println!("Checking for USDT payment to {}", payment_request.recipient_address);
    println!("Expected amount: {} USDT", payment_request.amount);
    println!("Token contract: {}", USDT_CONTRACT);
    println!("Required confirmations: {}", payment_request.required_confirmations);
    println!();

    // Verify the payment
    match verifier.verify_payment(&payment_request).await? {
        VerificationResult::Confirmed {
            tx_hash,
            confirmations,
        } => {
            println!("✓ USDT payment confirmed!");
            println!("  Transaction: {}", tx_hash);
            println!("  Confirmations: {}", confirmations);
        }
        VerificationResult::Pending {
            tx_hash,
            confirmations,
        } => {
            println!("⏳ USDT payment detected but pending confirmations");
            println!("  Transaction: {}", tx_hash);
            println!("  Confirmations: {}/{}", confirmations, payment_request.required_confirmations);
        }
        VerificationResult::NotFound => {
            println!("✗ No matching USDT payment found");
        }
        VerificationResult::Failed { reason } => {
            println!("✗ USDT payment verification failed: {}", reason);
        }
    }

    // Demonstrate using predefined currency helpers
    let usdc_payment = PaymentRequest {
        amount: Decimal::from_str("50.0")?,
        currency: Currency::usdc(), // Use predefined USDC
        recipient_address: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb0".to_string(),
        required_confirmations: 6,
        timeout_seconds: Some(3600),
    };

    println!("\nYou can also use predefined currencies:");
    println!("- Currency::usdt()");
    println!("- Currency::usdc()");
    println!("- Currency::busd()");

    Ok(())
}
