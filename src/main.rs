use color_eyre::Result;

mod chess;
mod engine;

#[cfg(test)]
mod tests;

use crate::engine::uci::UciInputMode;
use engine::uci;
use engine::util::log::log;

pub const ENGINE_NAME: &str = "Tcheran";

pub fn engine_version() -> &'static str {
    // If we can't determine the version from git tags, fall back to the version
    // specified in the Cargo manifest
    let version = git_version::git_version!(cargo_prefix = "cargo:v");

    // If we used Cargo's manifest, adjust the format so it matches the Git version tag format
    if version.starts_with("cargo:") {
        return version
            .strip_prefix("cargo:")
            .unwrap()
            .strip_suffix(".0")
            .unwrap();
    }

    // Otherwise, if we got a version from Git, we can use it directly
    version
}

pub fn init() {
    chess::init();
    engine::init();
}

fn main() -> Result<()> {
    color_eyre::install()?;

    std::panic::set_hook(Box::new(|info| {
        println!("{info}");
        log(format!("{info:?}"));
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
            std::process::exit(1);
        }
    };

    init();

    uci::uci(uci_input_mode)
}
