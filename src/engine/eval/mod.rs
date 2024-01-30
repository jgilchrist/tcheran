pub mod piece_square_tables;
mod player_eval;
mod tapered_eval;
mod white_eval;

use crate::chess::board::Board;
pub use player_eval::Eval;
pub use white_eval::WhiteEval;

use crate::chess::game::Game;
use crate::chess::piece::Piece;
use crate::chess::square::Square;
pub use crate::engine::eval::tapered_eval::PhasedEval;

pub fn init() {
    piece_square_tables::init();
}

#[derive(Debug, Clone)]
pub struct IncrementalEvalFields {
    pub phase_value: i16,

    pub piece_square_tables: PhasedEval,
}

impl IncrementalEvalFields {
    pub fn set_at(&mut self, sq: Square, piece: Piece) {
        self.phase_value += tapered_eval::piece_phase_value_contribution(piece.kind);
        self.piece_square_tables += piece_square_tables::piece_contributions(sq, piece);
    }

    pub fn remove_at(&mut self, sq: Square, piece: Piece) {
        self.phase_value -= tapered_eval::piece_phase_value_contribution(piece.kind);
        self.piece_square_tables -= piece_square_tables::piece_contributions(sq, piece);
    }
}

impl IncrementalEvalFields {
    pub fn init(board: &Board) -> Self {
        let phase_value = tapered_eval::phase_value(board);
        let piece_square_tables = piece_square_tables::eval(board);

        Self {
            phase_value,

            piece_square_tables,
        }
    }
}

pub fn eval(game: &Game) -> Eval {
    let absolute_eval = absolute_eval(game);
    Eval::from_white_eval(absolute_eval, game.player)
}

pub fn absolute_eval(game: &Game) -> WhiteEval {
    let eval = game.incremental_eval.piece_square_tables;

    tapered_eval::taper(game.incremental_eval.phase_value, eval)
}

#[derive(Debug)]
pub struct EvalComponents {
    pub eval: WhiteEval,
    pub piece_square_midgame: WhiteEval,
    pub piece_square_endgame: WhiteEval,
    pub phase_value: i16,
    pub piece_square_tables: WhiteEval,
}

pub fn eval_components(game: &Game) -> EvalComponents {
    let eval = absolute_eval(game);
    let phase_value = tapered_eval::phase_value(&game.board);

    let piece_square_tables = piece_square_tables::eval(&game.board);
    let pst_eval = tapered_eval::taper(phase_value, piece_square_tables);

    EvalComponents {
        eval,
        piece_square_midgame: piece_square_tables.midgame(),
        piece_square_endgame: piece_square_tables.endgame(),
        phase_value,
        piece_square_tables: pst_eval,
    }
}
