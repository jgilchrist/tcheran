mod cli;

use anyhow::Result;
use engine::uci;

pub enum RunMode {
    Uci,
    PrintBoard,
}

impl Default for RunMode {
    fn default() -> Self {
        RunMode::Uci
    }
}

fn print_board() {
    dbg!(chess::board::Board::start());
}

fn main() -> Result<()> {
    std::panic::set_hook(Box::new(|info| {
        chess::debug::log("crash", format!("{:?}", info))
    }));

    let run_mode = cli::parse_cli();

    match run_mode {
        RunMode::Uci => uci::uci(),
        RunMode::PrintBoard => {
            print_board();
            Ok(())
        }
    }
}
