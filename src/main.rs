mod chess;
mod engine;

#[cfg(test)]
mod tests;

use crate::engine::uci::UciInputMode;
use engine::uci;
use engine::util::log;
use std::process::ExitCode;

pub const ENGINE_NAME: &str = "Tcheran";

pub fn engine_version() -> String {
    let cargo_version = env!("CARGO_PKG_VERSION");
    let version = cargo_version.strip_suffix(".0").unwrap();
    let dev_suffix = if cfg!(feature = "release") { "" } else { "-dev" };

    format!("v{version}{dev_suffix}")
}

pub fn init() {
    chess::init();
    engine::init();
}

fn main() -> ExitCode {
    std::panic::set_hook(Box::new(|info| {
        println!("{info}");
        log::crashlog(format!("{info:?}"));
    }));

    let args = std::env::args().collect::<Vec<_>>();
    let uci_input_mode = match args.len() {
        1 => UciInputMode::Stdin,
        2 => {
            let commands = args[1]
                .replace("\\n", "\n")
                .lines()
                .map(ToString::to_string)
                .collect::<Vec<_>>();

            UciInputMode::Commands(commands)
        }
        _ => {
            let binary_name = args[0].clone();
            eprintln!("usage:");
            eprintln!("  {binary_name}                  - run in UCI mode");
            eprintln!(
                "  {binary_name} \"<uci commands>\" - run specific UCI commands and then exit"
            );

            return ExitCode::FAILURE;
        }
    };

    init();

    let result = uci::uci(uci_input_mode);

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
