#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::wildcard_imports,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
    clippy::inline_always,
    clippy::missing_const_for_fn
)]

use anyhow::Result;
use engine::util::log::log;

mod cli {
    use anyhow::Result;
    use chess::game::Game;
    use clap::{Parser, Subcommand, ValueEnum};
    use engine::{
        eval,
        strategy::KnownStrategy,
        uci::{self},
    };

    use chess::perft;

    #[derive(ValueEnum, Clone)]
    pub enum Strategy {
        Main,
        Random,
        TopEval,
    }

    #[derive(Parser)]
    pub struct Cli {
        #[command(subcommand)]
        pub command: Option<Commands>,
    }

    #[derive(Subcommand)]
    pub enum Commands {
        /// Run the engine using the UCI protocol
        Uci {
            #[arg(value_enum)]
            strategy: Strategy,
        },

        /// Run a perft test
        Perft { depth: u8, fen: Option<String> },

        /// Run a perft test for root moves
        PerftDiv {
            depth: u8,
            fen: String,
            moves: Option<String>,
        },

        /// Return the engine's evaluation of a position
        Eval { fen: String },
    }

    pub fn parse_cli() -> Cli {
        Cli::parse()
    }

    pub fn run(cmd: Commands) -> Result<()> {
        match cmd {
            Commands::Uci { strategy } => {
                let known_strategy = match strategy {
                    Strategy::Main => KnownStrategy::Main,
                    Strategy::Random => KnownStrategy::Random,
                    Strategy::TopEval => KnownStrategy::TopEval,
                };

                let strategy = known_strategy.create();
                uci::uci(strategy)
            }
            Commands::Perft { depth, fen } => {
                let game = fen.map_or_else(Game::default, |fen| Game::from_fen(&fen).unwrap());
                let result = perft::perft(depth, &game);
                println!("{result}");
                Ok(())
            }
            Commands::PerftDiv { depth, fen, moves } => {
                let mut game = Game::from_fen(&fen).unwrap();
                let moves = moves.map(|m| uci::parser::uci_moves(&m).unwrap().1);

                if let Some(ref moves) = moves {
                    for mv in moves {
                        game = game.make_move(mv).unwrap();
                    }
                }

                let depth_modifier: u8 = moves
                    .map(|m| m.len())
                    .unwrap_or_default()
                    .try_into()
                    .unwrap();

                let result = perft::perft_div(depth - depth_modifier, &game);

                for (mv, number_for_mv) in result {
                    println!("{mv:?}: {number_for_mv}");
                }

                Ok(())
            }
            Commands::Eval { fen } => {
                let game = Game::from_fen(&fen).unwrap();
                let eval_components = eval::eval_components(&game);

                println!("Eval: {}", eval_components.eval);
                println!("Components:");
                println!("  Material: {}", eval_components.material);
                println!(
                    "  Piece square tables: {}",
                    eval_components.piece_square_tables
                );
                println!("    White: {}", eval_components.piece_square_tables_white);
                println!("    Black: {}", eval_components.piece_square_tables_black);
                Ok(())
            }
        }
    }
}

fn main() -> Result<()> {
    std::panic::set_hook(Box::new(|info| {
        println!("{info}");
        log(format!("{info:?}"));
    }));

    engine::init();

    let args = cli::parse_cli();
    cli::run(args.command.unwrap_or(cli::Commands::Uci {
        strategy: cli::Strategy::Main,
    }))
}
