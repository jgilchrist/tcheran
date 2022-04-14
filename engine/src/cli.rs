use clap::{Parser, Subcommand};

use crate::RunMode;

#[derive(Parser)]
#[clap()]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the engine using the UCI protocol
    Uci {},

    /// Print the starting chessboard
    PrintBoard {},
}

pub fn parse_cli() -> RunMode {
    let args: Cli = Cli::parse();

    match &args.command {
        Some(cmd) => match cmd {
            Commands::Uci {} => RunMode::Uci,
            Commands::PrintBoard {} => RunMode::PrintBoard,
        },
        None => RunMode::default(),
    }
}
