use crate::engine::uci;
use crate::engine::uci::UciInputMode;
use crate::utils;
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

pub fn tune_command(file: &Path, epochs: usize) -> ExitCode {
    utils::tuner::tune(file, epochs);
    ExitCode::SUCCESS
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
