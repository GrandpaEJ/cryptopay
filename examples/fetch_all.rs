use cryptopay::client::{AccountEndpoints, TransactionEndpoints, TokenEndpoints, GasEndpoints};
use cryptopay::{EtherscanClient, ClientConfig};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key = env::var("ETHERSCAN_API_KEY")?;
    
    // Use the builder to ensure V2 defaults
    let config = ClientConfig::builder().api_key(api_key).build()?;
    let client = EtherscanClient::with_config(config)?;

    let address = "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb0";
    let usdt = "0xdAC17F958D2ee523a2206206994597C13D831ec7";

    println!("--- Account Balance ---");
    let balance = client.get_balance(address).await?;
    println!("Balance: {:#?}", balance);

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    println!("\n--- Transaction List (Last 1) ---");
    let txs = client.get_transactions(address, 0, 99999999, 1, 1, "desc").await?;
    if let Some(tx) = txs.first() {
        println!("{:#?}", tx);
        
        println!("\n--- Single Transaction ---");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let tx_details = client.get_transaction(&tx.hash).await?;
        println!("{:#?}", tx_details);

        println!("\n--- Transaction Receipt ---");
        // Use a known recent tx or the one we just found
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let receipt = client.get_transaction_receipt(&tx.hash).await?;
        println!("{:#?}", receipt);
    }

    println!("\n--- Token Balance (USDT) ---");
    let token_bal = client.get_token_balance(address, usdt).await?;
    println!("{:#?}", token_bal);

    println!("\n--- Token Transfers (Last 1) ---");
    let transfers = client.get_token_transfers(address, None, 0, 99999999, 1, 1, "desc").await?;
    if let Some(tf) = transfers.first() {
        println!("{:#?}", tf);
    }

    println!("\n--- Gas Oracle ---");
    let gas = client.get_gas_oracle().await?;
    println!("{:#?}", gas);

    Ok(())
}
