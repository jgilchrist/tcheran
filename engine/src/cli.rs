use chess::game::Game;
use clap::{Parser, Subcommand};
use engine::uci::parser;

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
    Perft { depth: u8, fen: Option<String> },

    /// Run a perft test for root moves
    PerftDiv {
        depth: u8,
        fen: String,
        moves: String,
    },
}

pub fn parse_cli() -> RunMode {
    let args: Cli = Cli::parse();

    match &args.command {
        Some(cmd) => match cmd {
            Commands::Uci {} => RunMode::Uci,
            Commands::PrintBoard {} => RunMode::PrintBoard,
            Commands::Perft { depth, fen } => {
                let default_fen =
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
                let fen = fen.as_ref().unwrap_or(&default_fen);
                RunMode::Perft(*depth, Game::from_fen(fen).unwrap())
            }
            Commands::PerftDiv { depth, fen, moves } => {
                let mut game = Game::from_fen(fen).unwrap();
                let (_, moves) = nom::combinator::opt(parser::uci_moves)(moves).unwrap();

                if let Some(moves) = moves {
                    for mv in moves {
                        game = game.make_move(&mv).unwrap();
                    }
                }

                RunMode::PerftDiv(*depth, game)
            }
        },
        None => RunMode::default(),
    }
}
