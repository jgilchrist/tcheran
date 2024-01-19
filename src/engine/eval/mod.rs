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

pub fn init() {
    piece_square_tables::init();
}

#[derive(Debug, Clone)]
pub struct IncrementalEvalFields {
    pub phase_value: i16,

    pub midgame_pst_eval: WhiteEval,
    pub endgame_pst_eval: WhiteEval,
}

impl IncrementalEvalFields {
    pub fn set_at(&mut self, sq: Square, piece: Piece) {
        let phase_value_diff = tapered_eval::piece_phase_value_contribution(piece.kind);
        self.phase_value += phase_value_diff;

        let (mg, eg) = piece_square_tables::piece_contributions(sq, piece);
        self.midgame_pst_eval += mg;
        self.endgame_pst_eval += eg;
    }

    pub fn remove_at(&mut self, sq: Square, piece: Piece) {
        let phase_value_diff = tapered_eval::piece_phase_value_contribution(piece.kind);
        self.phase_value -= phase_value_diff;

        let (mg, eg) = piece_square_tables::piece_contributions(sq, piece);
        self.midgame_pst_eval -= mg;
        self.endgame_pst_eval -= eg;
    }
}

impl IncrementalEvalFields {
    pub fn init(board: &Board) -> Self {
        let phase_value = tapered_eval::phase_value(board);
        let (midgame_pst_eval, endgame_pst_eval) = piece_square_tables::phase_evals(board);

        Self {
            phase_value,

            midgame_pst_eval,
            endgame_pst_eval,
        }
    }
}

pub fn eval(game: &Game) -> Eval {
    let absolute_eval = absolute_eval(game);
    Eval::from_white_eval(absolute_eval, game.player)
}

pub fn absolute_eval(game: &Game) -> WhiteEval {
    let midgame_eval = game.incremental_eval.midgame_pst_eval;
    let endgame_eval = game.incremental_eval.endgame_pst_eval;

    tapered_eval::taper(
        game.incremental_eval.phase_value,
        midgame_eval,
        endgame_eval,
    )
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

    let (midgame_pst, endgame_pst) = piece_square_tables::phase_evals(&game.board);
    let phase_value = tapered_eval::phase_value(&game.board);

    let pst_eval = tapered_eval::taper(phase_value, midgame_pst, endgame_pst);

    EvalComponents {
        eval,
        piece_square_midgame: midgame_pst,
        piece_square_endgame: endgame_pst,
        phase_value,
        piece_square_tables: pst_eval,
    }
}
