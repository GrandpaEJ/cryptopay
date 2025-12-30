//! Payment monitoring with callbacks example

use cryptopay::{BscScanClient, Currency, PaymentMonitor, PaymentRequest, PaymentStatus};
use rust_decimal::Decimal;
use std::str::FromStr;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = std::env::var("BSCSCAN_API_KEY")
        .expect("BSCSCAN_API_KEY environment variable not set");

    // Create BscScan client
    let client = BscScanClient::new(api_key)?;

    // Create payment monitor with 10-second polling interval
    let monitor = PaymentMonitor::builder()
        .client(client)
        .poll_interval(Duration::from_secs(10))
        .build();

    // Create a payment request
    let payment_request = PaymentRequest {
        amount: Decimal::from_str("0.1")?,
        currency: Currency::BNB,
        recipient_address: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb0".to_string(),
        required_confirmations: 12,
        timeout_seconds: Some(1800), // 30 minutes
    };

    println!("🔍 Monitoring payment...");
    println!("Send {} BNB to: {}", payment_request.amount, payment_request.recipient_address);
    println!("Polling every 10 seconds...");
    println!();

    // Start monitoring with callback
    monitor
        .start_monitoring(payment_request, |status| {
            match status {
                PaymentStatus::Pending => {
                    println!("⏳ Status: Waiting for payment...");
                }
                PaymentStatus::Detected { tx_hash, confirmations } => {
                    println!("📥 Payment detected!");
                    println!("   Transaction: {}", tx_hash);
                    println!("   Confirmations: {}", confirmations);
                }
                PaymentStatus::Confirmed { tx_hash, confirmations } => {
                    println!("✅ Payment confirmed!");
                    println!("   Transaction: {}", tx_hash);
                    println!("   Final confirmations: {}", confirmations);
                }
                PaymentStatus::Failed { reason } => {
                    println!("❌ Payment failed: {}", reason);
                }
                PaymentStatus::Expired => {
                    println!("⏰ Payment expired");
                }
            }
        })
        .await?;

    println!("\n✨ Monitoring complete!");

    Ok(())
}
