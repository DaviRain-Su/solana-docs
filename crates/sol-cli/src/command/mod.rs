use clap::Parser;

pub mod balance;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Command {
    #[command(name = "balance")]
    Balance(balance::Balance),
}

impl Command {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            Command::Balance(balance) => balance.run(),
        }
    }
}