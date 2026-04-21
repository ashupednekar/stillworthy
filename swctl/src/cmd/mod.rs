use crate::prelude::Result;
use clap::{Parser, Subcommand};
use execblck::{Extention, Guard};

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
            let ext = Extention::new().unwrap();
            ext.notify().unwrap();
        }
        None => {
            tracing::error!("no subcommand passed");
        }
    }
    Ok(())
}
