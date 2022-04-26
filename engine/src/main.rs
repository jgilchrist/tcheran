mod cli;

use anyhow::Result;
use chess::game::Game;
use engine::uci;

pub enum RunMode {
    Uci,
    PrintBoard,
    Perft(u8, Game),
    PerftDiv(u8, Game),
}

impl Default for RunMode {
    fn default() -> Self {
        RunMode::Uci
    }
}

fn print_board() {
    dbg!(chess::board::Board::start());
}

fn perft(depth: u8, game: &Game) -> usize {
    if depth == 0 {
        return 1;
    }

    game.legal_moves()
        .iter()
        .map(|m| perft(depth - 1, &game.make_move(m).unwrap()))
        .sum()
}

fn perft_div(depth: u8, game: &Game) {
    let root_moves = game.legal_moves();

    for mv in root_moves {
        println!(
            "{:?}: {}",
            mv,
            perft(depth - 1, &game.make_move(&mv).unwrap())
        )
    }
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
        RunMode::Perft(depth, game) => {
            println!("{}", perft(depth, &game));
            Ok(())
        }
        RunMode::PerftDiv(depth, game) => {
            perft_div(depth, &game);
            Ok(())
        }
    }
}
