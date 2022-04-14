#![allow(dead_code)]
#![allow(unused_variables)]

mod cli;

use anyhow::Result;
use chess::{bitboard::Bitboard, square::Square};
use engine::uci;

pub enum RunMode {
    Uci,
    PrintBoard,
    Devel,
}

impl Default for RunMode {
    fn default() -> Self {
        RunMode::Devel
    }
}

fn print_board() {
    dbg!(chess::board::Board::start());
}

fn devel() {
    let bitboard = Bitboard::from_square(&Square::E1);
}

fn main() -> Result<()> {
    let run_mode = cli::parse_cli();

    match run_mode {
        RunMode::Uci => uci::uci(),
        RunMode::PrintBoard => {
            print_board();
            Ok(())
        }
        RunMode::Devel => {
            devel();
            Ok(())
        }
    }
}
