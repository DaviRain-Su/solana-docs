use anyhow::anyhow;
use solana_cli_config::{Config, CONFIG_FILE};
use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::Sol;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::read_keypair_file;

fn main() -> anyhow::Result<()> {
    let config_file = CONFIG_FILE
        .as_ref()
        .ok_or_else(|| anyhow!("unable to get config file path"))?;
    let cli_config = Config::load(&config_file)?;
    println!("Config: {:#?}", cli_config);

    let keypair = read_keypair_file(&cli_config.keypair_path);
    let key_pair = match keypair {
        Ok(keypair) => {
            println!("Read keypair: {:?}", keypair.pubkey());
            keypair
        }
        Err(e) => {
            eprintln!("Unable to read keypair: {:?}", e);
            return Err(anyhow!("Unable to read keypair"));
        }
    };

    let rpc_client = RpcClient::new(cli_config.json_rpc_url.to_string());

    let balance = rpc_client.get_balance(&key_pair.pubkey())?;
    let balance = Sol(balance);
    println!("Balance: {:.9} SOL", balance);

    Ok(())
}
