mod san_parser;
mod san_writer;

const KINGSIDE_CASTLE: &str = "O-O";
const QUEENSIDE_CASTLE: &str = "O-O-O";
const CAPTURE: char = 'x';
const PROMOTION: char = '=';
const CHECK: char = '+';
const CHECKMATE: char = '#';

#[expect(unused, reason = "Unused")]
pub use san_parser::parse_move;

pub use san_writer::format_move;
