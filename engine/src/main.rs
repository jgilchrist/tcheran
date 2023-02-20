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
use engine::log::log;

mod cli {
    use std::{io::BufWriter, net::TcpStream};

    use anyhow::Result;
    use chess::game::Game;
    use clap::{Parser, Subcommand, ValueEnum};
    use engine::{
        strategy::KnownStrategy,
        uci::{
            self,
            comms::{LocalComms, RemoteComms},
        },
    };

    use engine::perft;

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

            #[arg(long)]
            remote: bool,
        },

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
            Commands::Uci { strategy, remote } => {
                let known_strategy = match strategy {
                    Strategy::Main => KnownStrategy::Main,
                    Strategy::Random => KnownStrategy::Random,
                    Strategy::TopEval => KnownStrategy::TopEval,
                };

                let strategy = known_strategy.create();

                // TODO: Move this code into comms
                if remote {
                    let stream = TcpStream::connect("localhost:3001").unwrap();
                    let writer = BufWriter::new(stream.try_clone().unwrap());
                    let mut comms = RemoteComms { stream, writer };
                    uci::uci(&mut comms, strategy)
                } else {
                    let mut comms = LocalComms {};
                    uci::uci(&mut comms, strategy)
                }
            }
            Commands::Perft { depth, fen } => {
                let game = fen.map_or_else(Game::default, |fen| Game::from_fen(&fen).unwrap());
                let result = perft::perft(depth, &game);
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

                perft::perft_div(depth, &game);
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

    let args = cli::parse_cli();
    cli::run(args.command.unwrap_or(cli::Commands::Uci {
        strategy: cli::Strategy::Main,
        remote: false,
    }))
}
