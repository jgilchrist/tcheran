mod cli;

use engine::uci;

pub enum RunMode {
    Uci,
    PrintBoard,
}

fn print_board() {
    dbg!(chess::board::Board::start());
}

fn main() {
    let run_mode = cli::parse_cli();

    match run_mode {
        RunMode::Uci => {
            uci::uci()
        },
        RunMode::PrintBoard => {
            print_board()
        },
    }
}
