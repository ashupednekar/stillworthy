use clap::{Parser, Subcommand};
use crate::prelude::Result;

#[derive(Parser)]
struct Cmd {
    #[command(subcommand)]
    command: Option<SubCmd>,
}

#[derive(Subcommand)]
enum SubCmd {
    Enable,
}

pub async fn run() -> Result<()>{
    let args = Cmd::parse();
    match args.command {
        Some(SubCmd::Enable) => {}
        None => {
            tracing::error!("no subcommand passed");
        }
    }
    Ok(())
}
