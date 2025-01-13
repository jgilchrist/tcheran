use crate::engine::uci;
use crate::engine::uci::UciInputMode;
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Uci,

    Tune {
        file: PathBuf,

        #[clap(default_value_t = 5000)]
        epochs: usize,
    },
}

pub fn uci_command() -> ExitCode {
    let result = uci::uci(UciInputMode::Stdin);

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(feature = "tuner")]
pub fn tune_command(file: &Path, epochs: usize) -> ExitCode {
    crate::utils::tuner::tune(file, epochs);
    ExitCode::SUCCESS
}

#[cfg(not(feature = "tuner"))]
pub fn tune_command(_file: &Path, _epochs: usize) -> ExitCode {
    eprintln!("Tuning requires the 'tuner' feature to be enabled");
    ExitCode::FAILURE
}

pub fn run() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Some(c) => match c {
            Command::Uci => uci_command(),
            Command::Tune { file, epochs } => tune_command(&file, epochs),
        },
        _ => uci_command(),
    }
}
