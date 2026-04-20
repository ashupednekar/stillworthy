use crate::prelude::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cmd {
    #[command(subcommand)]
    command: Option<SubCmd>,
}

#[derive(Subcommand)]
enum SubCmd {
    Enable,
}

pub async fn run() -> Result<()> {
    let args = Cmd::parse();
    match args.command {
        Some(SubCmd::Enable) => {
            execblck::guard::notify_exec().unwrap();
        }
        None => {
            tracing::error!("no subcommand passed");
        }
    }
    Ok(())
}
