use crate::engine::uci;
use crate::engine::uci::UciInputMode;
use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Uci,
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

pub fn run() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Some(c) => match c {
            Command::Uci => uci_command(),
        },
        _ => uci_command(),
    }
}
