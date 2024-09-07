mod san_parser;
mod san_writer;

const KINGSIDE_CASTLE: &str = "O-O";
const QUEENSIDE_CASTLE: &str = "O-O-O";
const CAPTURE: char = 'x';
const PROMOTION: char = '=';
const CHECK: char = '+';

#[expect(unused)]
const CHECKMATE: char = '#';

#[expect(unused_imports)]
#[expect(unused)]
pub use san_parser::parse_move;

pub use san_writer::format_move;
