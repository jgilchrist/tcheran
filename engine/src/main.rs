use color_eyre::Result;
use engine::util::log::log;

mod cli {
    use clap::{Parser, Subcommand};
    use color_eyre::Result;
    use engine::uci::{self};

    #[derive(Parser)]
    pub struct Cli {
        #[command(subcommand)]
        pub command: Option<Commands>,
    }

    #[derive(Subcommand)]
    pub enum Commands {
        /// Run the engine using the UCI protocol
        Uci,
    }

    pub fn parse_cli() -> Cli {
        Cli::parse()
    }

    pub fn run(cmd: Commands) -> Result<()> {
        match cmd {
            Commands::Uci => uci::uci(),
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
    cli::run(args.command.unwrap_or(cli::Commands::Uci))
}
