#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::wildcard_imports,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
    clippy::inline_always
)]

use anyhow::Result;
use chess::game::Game;
use engine::log::log;

mod cli {
    use crate::{perft, perft_div};

    use anyhow::Result;
    use chess::game::Game;
    use clap::{Parser, Subcommand, ValueEnum};
    use engine::{
        strategy::{self, KnownStrategy},
        uci,
    };

    #[derive(ValueEnum, Clone)]
    pub enum Strategy {
        Random,
        TopEval,
        OutOfProcess,
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

        /// Run in out-of-process engine mode
        OutOfProcess,

        /// Run a perft test
        Perft { depth: u8, fen: Option<String> },

        /// Run a perft test for root moves
        PerftDiv {
            depth: u8,
            fen: String,
            moves: String,
        },
    }

    pub fn parse_cli() -> Cli {
        Cli::parse()
    }

    pub fn run(cmd: Commands) -> Result<()> {
        match cmd {
            Commands::Uci { strategy } => {
                let known_strategy = match strategy {
                    Strategy::Random => KnownStrategy::Random,
                    Strategy::TopEval => KnownStrategy::TopEval,
                    Strategy::OutOfProcess => KnownStrategy::OutOfProcess,
                };

                let strategy = known_strategy.create();
                uci::uci(strategy)
            }
            Commands::OutOfProcess => {
                engine::strategy::run_out_of_process_engine(strategy::KnownStrategy::Main.create())
            }
            Commands::Perft { depth, fen } => {
                let game = fen.map_or_else(Game::default, |fen| Game::from_fen(&fen).unwrap());
                let result = perft(depth, &game);
                println!("{result}");
                Ok(())
            }
            Commands::PerftDiv { depth, fen, moves } => {
                let mut game = Game::from_fen(&fen).unwrap();
                let (_, moves) = uci::parser::maybe_uci_moves(&moves).unwrap();

                if let Some(moves) = moves {
                    for mv in moves {
                        game = game.make_move(&mv).unwrap();
                    }
                }

                perft_div(depth, &game);
                Ok(())
            }
        }
    }
}

fn perft(depth: u8, game: &Game) -> usize {
    if depth == 1 {
        return game.legal_moves().len();
    }

    game.legal_moves()
        .iter()
        .map(|m| perft(depth - 1, &game.make_move(m).unwrap()))
        .sum()
}

fn perft_div(depth: u8, game: &Game) {
    let root_moves = game.legal_moves();
    let mut all = 0;

    for mv in root_moves {
        let number_for_mv = perft(depth - 1, &game.make_move(&mv).unwrap());

        println!("{mv:?} {number_for_mv}");
        all += number_for_mv;
    }

    println!();
    println!("{all}");
}

fn main() -> Result<()> {
    std::panic::set_hook(Box::new(|info| {
        println!("{info}");
        log(format!("{info:?}"));
    }));

    let args = cli::parse_cli();
    cli::run(args.command.unwrap_or(cli::Commands::Uci {
        strategy: cli::Strategy::Random,
    }))
}
