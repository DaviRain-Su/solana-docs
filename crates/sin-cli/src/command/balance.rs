use std::str::FromStr;

use anyhow::anyhow;
use clap::Parser;
use solana_cli_config::{Config, CONFIG_FILE};
use solana_client::rpc_client::RpcClient;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::native_token::{sol_to_lamports, Sol};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::read_keypair_file;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;

#[derive(Parser, Debug)]
pub enum SolNative {
    #[command(name = "balance", about = "获取账户余额")]
    SolNative(SolNativeBalance),
    #[command(name = "transfer", about = "转账")]
    SolNativeTransfer(SolNativeTransfer),
}

impl SolNative {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            SolNative::SolNative(balance) => balance.run(),
            SolNative::SolNativeTransfer(transfer) => transfer.run(),
        }
    }
}

#[derive(Parser, Debug)]
pub struct SolNativeBalance {
    /// 指定读取的账户地址
    #[arg(short, long)]
    pub address: Option<String>,
}

impl SolNativeBalance {
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

#[derive(Parser, Debug)]
pub struct SolNativeTransfer {
    /// 指定接收的账户地址
    #[arg(short, long)]
    pub to: Pubkey,
    #[arg(short, long)]
    /// 指定转账的数量
    pub amount: f64,
}

impl SolNativeTransfer {
    pub fn run(&self) -> anyhow::Result<()> {
        let config_file = CONFIG_FILE
            .as_ref()
            .ok_or_else(|| anyhow!("unable to get config file path"))?;
        let cli_config = Config::load(&config_file)?;
        log::info!("Config: {:#?}", cli_config);
        let rpc_client = RpcClient::new(cli_config.json_rpc_url.to_string());
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

        let recent_blockhash = rpc_client.get_latest_blockhash()?;
        log::info!("Recent blockhash: {:?}", recent_blockhash);
        let amount = sol_to_lamports(self.amount);
        log::info!("Amount: {}", amount);

        // let SetComputeUnitPrice
        let set_compute_unit_price_instruction =
            ComputeBudgetInstruction::set_compute_unit_price(100000);
        // let SetComputeUnitLimit
        let set_compute_unit_limit_instruction =
            ComputeBudgetInstruction::set_compute_unit_limit(500);
        let mut transaction = Transaction::new_with_payer(
            &[
                set_compute_unit_price_instruction,
                set_compute_unit_limit_instruction,
                system_instruction::transfer(&key_pair.pubkey(), &self.to, amount),
            ],
            Some(&key_pair.pubkey()),
        );
        log::info!("Transaction: {:#?}", transaction);
        let blockhash = rpc_client.get_latest_blockhash()?;
        log::info!("Blockhash: {:?}", blockhash);
        transaction.sign(&[&key_pair], blockhash);
        let signature = rpc_client.send_and_confirm_transaction(&transaction)?;
        println!("https://solscan.io/tx/{}", signature);

        Ok(())
    }
}
