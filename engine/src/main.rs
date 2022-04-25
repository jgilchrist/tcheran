use anyhow::Result;
use chess::game::Game;
use engine::strategy::{self, KnownStrategy, Strategy};
use engine::uci;

mod cli {
    use super::RunMode;
    use chess::game::Game;
    use clap::{Parser, Subcommand};
    use engine::{strategy::KnownStrategy, uci};

    #[derive(clap::ValueEnum, Clone)]
    enum Strategy {
        Random,
        TopEval,
        OutOfProcess,
    }

    #[derive(Parser)]
    #[clap()]
    struct Cli {
        #[clap(subcommand)]
        command: Option<Commands>,
    }

    #[derive(Subcommand)]
    enum Commands {
        /// Run the engine using the UCI protocol
        Uci {
            #[clap(value_enum)]
            strategy: Strategy,
        },

        /// Run in out-of-process engine mode
        OutOfProcess {},

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
                Commands::Uci { strategy } => {
                    let known_strategy = match strategy {
                        Strategy::Random => KnownStrategy::Random,
                        Strategy::TopEval => KnownStrategy::TopEval,
                        Strategy::OutOfProcess => KnownStrategy::OutOfProcess,
                    };

                    RunMode::Uci(known_strategy.create())
                }
                Commands::OutOfProcess {} => RunMode::OutOfProcessEngine,
                Commands::Perft { depth, fen } => {
                    let fen = fen.clone().unwrap_or_else(|| chess::fen::START_POS.to_string());
                    RunMode::Perft(*depth, Game::from_fen(&fen).unwrap())
                }
                Commands::PerftDiv { depth, fen, moves } => {
                    let mut game = Game::from_fen(fen).unwrap();
                    let (_, moves) = uci::parser::maybe_uci_moves(moves).unwrap();

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
}

pub enum RunMode {
    Uci(Box<dyn Strategy>),
    OutOfProcessEngine,
    Perft(u8, Game),
    PerftDiv(u8, Game),
}

impl Default for RunMode {
    fn default() -> Self {
        RunMode::Uci(KnownStrategy::OutOfProcess.create())
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

        println!("{:?} {}", mv, number_for_mv);
        all += number_for_mv
    }

    println!();
    println!("{}", all);
}

fn main() -> Result<()> {
    std::panic::set_hook(Box::new(|info| {
        chess::debug::log("crash", format!("{:?}", info))
    }));

    let run_mode = cli::parse_cli();

    match run_mode {
        RunMode::Uci(strategy) => uci::uci(strategy),
        RunMode::OutOfProcessEngine => {
            engine::strategy::run_out_of_process_engine(strategy::KnownStrategy::Main.create())
        }
        RunMode::Perft(depth, game) => {
            println!("{}", perft(depth, &game));
            Ok(())
        }
        RunMode::PerftDiv(depth, game) => {
            perft_div(depth, &game);
            Ok(())
        }
    }
}
