use color_eyre::Result;
use engine::util::log::log;

mod cli {
    use clap::{Parser, Subcommand, ValueEnum};
    use color_eyre::Result;
    use engine::{
        strategy::KnownStrategy,
        uci::{self},
    };

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
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

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
