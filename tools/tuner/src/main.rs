use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

mod parameters;
mod tuner;

#[derive(Parser)]
struct Cli {
    file: PathBuf,

    #[clap(default_value_t = 5000)]
    epochs: usize,
}

pub fn main() -> ExitCode {
    let cli = Cli::parse();

    engine::init();
    tuner::tune(&cli.file, cli.epochs);
    ExitCode::SUCCESS
}
