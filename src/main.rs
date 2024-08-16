use clap::{Parser, Subcommand};
use commands::*;

mod commands;
mod utils;

#[macro_use]
mod macros;

/// Description of your CLI
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Args {
    #[clap(subcommand)]
    command: Commands,

    #[clap(long)]
    silent: bool,
}

commands_enum!(hello);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Args::parse();

    match Commands::exec(cli).await {
        Ok(_) => {}
        Err(e) => {
            // If the user cancels the operation, we want to exit successfully
            // This can happen if Ctrl+C is pressed during a prompt
            if e.root_cause().to_string() == inquire::InquireError::OperationInterrupted.to_string()
            {
                return Ok(());
            }

            eprintln!("{:?}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
