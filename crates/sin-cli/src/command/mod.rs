use clap::Parser;

pub mod balance;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Command {
    #[command(subcommand, name = "sn", about = "操作solana令牌的指令")]
    SolNative(balance::SolNative),
}

impl Command {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            Command::SolNative(balance) => balance.run(),
        }
    }
}
