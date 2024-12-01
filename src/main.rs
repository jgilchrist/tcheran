mod chess;
mod engine;

#[cfg(not(feature = "release"))]
mod utils;

#[cfg(test)]
mod tests;

use engine::uci;
use engine::util::log;
use std::panic::PanicHookInfo;
use std::process::ExitCode;

pub const ENGINE_NAME: &str = "Tcheran";

#[cfg(all(feature = "default", feature = "release"))]
compile_error!("features \"default\" and \"release\" cannot be enabled simultaneously");

pub fn engine_version() -> String {
    let cargo_version = env!("CARGO_PKG_VERSION");
    let version = cargo_version.strip_suffix(".0").unwrap();
    let dev_suffix = if cfg!(feature = "release") {
        ""
    } else {
        "-dev"
    };

    format!("v{version}{dev_suffix}")
}

pub fn init() {
    chess::init();
    engine::init();
}

fn get_panic_message(info: &PanicHookInfo<'_>) -> String {
    if let Some(s) = info.payload().downcast_ref::<&str>() {
        format!("panic occurred: {s:?} {info:?}")
    } else if let Some(s) = info.payload().downcast_ref::<String>() {
        format!("panic occurred: {s:?} {info:?}")
    } else {
        format!("{info:?}")
    }
}

#[cfg(not(feature = "release"))]
fn run() -> ExitCode {
    use utils::cli;

    cli::run()
}

#[cfg(feature = "release")]
fn run() -> ExitCode {
    use crate::engine::uci::UciInputMode;

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

    let result = uci::uci(uci_input_mode);

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn main() -> ExitCode {
    std::panic::set_hook(Box::new(|info| {
        let panic_message = get_panic_message(info);

        println!("{panic_message}");
        log::crashlog(panic_message);
    }));

    init();
    run()
}
