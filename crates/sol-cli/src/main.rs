use clap::Parser;

mod command;
use command::Command;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let command = Command::parse();

    command.run()?;

    Ok(())
}
