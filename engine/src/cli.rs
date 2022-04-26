use chess::game::Game;
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

    /// Run a perft test
    Perft { depth: u8, fen: String },

    /// Run a perft test for root moves
    PerftDiv { depth: u8, fen: String },
}

pub fn parse_cli() -> RunMode {
    let args: Cli = Cli::parse();

    match &args.command {
        Some(cmd) => match cmd {
            Commands::Uci {} => RunMode::Uci,
            Commands::PrintBoard {} => RunMode::PrintBoard,
            Commands::Perft { depth, fen } => RunMode::Perft(*depth, Game::from_fen(fen).unwrap()),
            Commands::PerftDiv { depth, fen } => {
                RunMode::PerftDiv(*depth, Game::from_fen(fen).unwrap())
            }
        },
        None => RunMode::default(),
    }
}
