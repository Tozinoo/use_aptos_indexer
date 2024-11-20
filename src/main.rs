use anyhow::Result;
use aptos_sdk::rest_client::{AptosBaseUrl, ClientBuilder};
use aptos_sdk::types::account_address::AccountAddress;
use aptos_sdk::types::network_address::NetworkAddress;
use clap::{Parser, ValueEnum};
use url::Url;
use dotenv::dotenv;
use std::env;


#[derive(Debug, Parser)]
struct Args {
    #[clap(long)]
    network: Network,
}

#[derive(Debug, Clone, ValueEnum)]
enum Network {
    Mainnet,
    Devnet,
    Testnet,
}

impl Network {
    fn to_aptos_base_url(&self) -> AptosBaseUrl {
        match self {
            Network::Mainnet => AptosBaseUrl::Mainnet,
            Network::Devnet => AptosBaseUrl::Devnet,
            Network::Testnet => AptosBaseUrl::Testnet,
        }
    }
    fn get_api_key(&self) -> String {
        match self {
            Network::Mainnet => env::var("MAINNET_API_KEY").expect("MAINNET_API_KEY must be set"),
            Network::Devnet => env::var("DEVNET_API_KEY").expect("DEVNET_API_KEY must be set"),
            Network::Testnet => env::var("TESTNET_API_KEY").expect("TESTNET_API_KEY must be set"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let args = Args::parse();

    let aptos_base_url = args.network.to_aptos_base_url();
    let api_key = args.network.get_api_key();

    let node_api_client = ClientBuilder::new(aptos_base_url)
        .api_key(&api_key)?
        .build();

    let account = node_api_client
        .get_account(AccountAddress::from_str_strict("0x1")?)
        .await?;
    println!("Account: {:#?}", account);

    Ok(())
}