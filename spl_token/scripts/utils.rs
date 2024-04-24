use dotenv;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

pub fn main() {}

#[derive(Deserialize, Serialize)]
pub struct ChainConfig {
    pub solana_mainnet: String,
    pub solana_devnet: String,
}

pub fn get_rpc(env: String) -> RpcClient {
    let path = std::env::current_dir().unwrap().join("chains.json");

    let chain_config =
        serde_json::from_str::<ChainConfig>(&std::fs::read_to_string(path).unwrap()).unwrap();

    match env.as_str() {
        "solana_devnet" => {
            return RpcClient::new(chain_config.solana_devnet);
        }
        "solana_mainnet" => {
            return RpcClient::new(chain_config.solana_mainnet);
        }
        _ => panic!(
            "Invalid solana network! Supported networks are: solana_devnet and solana_mainnet"
        ),
    }
}

pub fn get_authority() -> Keypair {
    dotenv::dotenv().ok();

    let raw_keypair = std::env::var("AUTHORITY").expect("Missing AUTHORITY env variable!");

    Keypair::from_base58_string(&raw_keypair)
}
