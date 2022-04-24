mod cli;

use anyhow::Result;
use chess::{game::Game, r#move::Move, square::Square};
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

fn devel() {
    let game = Game::new();
    dbg!(&game);
    let game = game.make_move(&Move::new(Square::E2, Square::E4)).unwrap();
    dbg!(&game);
    let game = game.make_move(&Move::new(Square::E7, Square::E5)).unwrap();
    dbg!(&game);
    let game = game.make_move(&Move::new(Square::G1, Square::F3)).unwrap();
    dbg!(&game);
    let game = game.make_move(&Move::new(Square::G8, Square::F6)).unwrap();
    dbg!(&game);
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
