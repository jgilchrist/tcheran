use clap::{Parser, Subcommand};

use crate::RunMode;

#[derive(Parser)]
#[clap()]
struct Cli {
    #[clap(subcommand)]
    command: Commands
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

    return match &args.command {
        Commands::Uci {} => {
            RunMode::Uci
        },
        Commands::PrintBoard {} => {
            RunMode::PrintBoard
        }
    }
}
