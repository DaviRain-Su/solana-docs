use std::str::FromStr;

use anyhow::anyhow;
use clap::Parser;
use solana_cli_config::{Config, CONFIG_FILE};
use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::Sol;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::read_keypair_file;

#[derive(Parser, Debug)]
pub struct Balance {
    /// 指定读取的账户地址
    #[arg(short, long)]
    pub address: Option<String>,
}

impl Balance {
    pub fn run(&self) -> anyhow::Result<()> {
        let config_file = CONFIG_FILE
            .as_ref()
            .ok_or_else(|| anyhow!("unable to get config file path"))?;
        let cli_config = Config::load(&config_file)?;
        log::info!("Config: {:#?}", cli_config);
        let address = if let Some(address) = &self.address {
            Pubkey::from_str(address)?
        } else {
            let keypair = read_keypair_file(&cli_config.keypair_path);
            let key_pair = match keypair {
                Ok(keypair) => {
                    log::info!("Read keypair: {:?}", keypair.pubkey());
                    keypair
                }
                Err(e) => {
                    eprintln!("Unable to read keypair: {:?}", e);
                    return Err(anyhow!("Unable to read keypair"));
                }
            };
            key_pair.pubkey()
        };

        let rpc_client = RpcClient::new(cli_config.json_rpc_url.to_string());

        let balance = rpc_client.get_balance(&address)?;
        let balance = Sol(balance);
        println!("地址 {} 有 {:.9} SOL", address, balance);
        Ok(())
    }
}
