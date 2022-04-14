#![allow(dead_code)]
#![allow(unused_variables)]

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
    let run_mode = cli::parse_cli();

    match run_mode {
        RunMode::Uci => uci::uci(),
        RunMode::PrintBoard => {
            print_board();
            Ok(())
        }
    }
}
