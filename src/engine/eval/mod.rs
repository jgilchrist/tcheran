mod pawn_structure;
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
use crate::engine::transposition_table::TranspositionTable;

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

pub fn eval(game: &Game, pawn_structure_tt: &mut TranspositionTable<PhasedEval>) -> Eval {
    let absolute_eval = absolute_eval(game, pawn_structure_tt);
    Eval::from_white_eval(absolute_eval, game.player)
}

pub fn absolute_eval(
    game: &Game,
    pawn_structure_tt: &mut TranspositionTable<PhasedEval>,
) -> WhiteEval {
    let pawn_structure_eval = pawn_structure_tt.get(&game.pawn_zobrist);
    let pawn_structure_eval = if let Some(e) = pawn_structure_eval {
        *e
    } else {
        let e = pawn_structure::eval(&game.board);
        pawn_structure_tt.insert(&game.pawn_zobrist, e);
        e
    };

    let eval = game.incremental_eval.piece_square_tables + pawn_structure_eval;
    tapered_eval::taper(game.incremental_eval.phase_value, eval)
}

#[derive(Debug)]
pub struct EvalComponents {
    pub eval: WhiteEval,
    pub phase_value: i16,

    pub phased_piece_square: PhasedEval,
    pub piece_square: WhiteEval,
}

pub fn eval_components(
    game: &Game,
    pawn_structure_tt: &mut TranspositionTable<PhasedEval>,
) -> EvalComponents {
    let eval = absolute_eval(game, pawn_structure_tt);
    let phase_value = tapered_eval::phase_value(&game.board);

    let phased_piece_square = piece_square_tables::eval(&game.board);
    let piece_square = tapered_eval::taper(phase_value, phased_piece_square);

    EvalComponents {
        eval,
        phase_value,

        phased_piece_square,
        piece_square,
    }
}
