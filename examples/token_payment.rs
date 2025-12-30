//! ERC20 token payment verification example

use cryptopay::{EtherscanClient, Currency, PaymentRequest, PaymentVerifier, VerificationResult};
use rust_decimal::Decimal;
use std::str::FromStr;

// Ethereum Mainnet USDT contract address
const USDT_CONTRACT: &str = "0xdAC17F958D2ee523a2206206994597C13D831ec7";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    // Get API key from environment
    let api_key = std::env::var("ETHERSCAN_API_KEY")
        .expect("ETHERSCAN_API_KEY environment variable not set");

    // Create Etherscan client
    let client = EtherscanClient::new(api_key)?;
    let verifier = PaymentVerifier::new(client);

    // Create a payment request for 100 USDT
    let payment_request = PaymentRequest {
        amount: Decimal::from_str("100.0")?,
        currency: Currency::ERC20 {
            contract_address: USDT_CONTRACT.to_string(),
            decimals: 6, // USDT has 6 decimals on Ethereum
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
    let _usdc_payment = PaymentRequest {
        amount: Decimal::from_str("50.0")?,
        currency: Currency::usdc(), // Use predefined USDC
        recipient_address: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb0".to_string(),
        required_confirmations: 6,
        timeout_seconds: Some(3600),
    };

    println!("\nYou can also use predefined currencies:");
    println!("- Currency::usdt()");
    println!("- Currency::usdc()");
    println!("- Currency::dai()");

    Ok(())
}
