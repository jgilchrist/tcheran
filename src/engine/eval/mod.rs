pub mod piece_square_tables;
mod player_eval;
mod white_eval;

pub use player_eval::Eval;
pub use white_eval::WhiteEval;

use crate::chess::game::Game;
use crate::chess::piece::Piece;
use crate::chess::square::Square;
use crate::engine::game::EngineGame;

pub fn init() {
    piece_square_tables::init();
}

#[derive(Debug, Clone)]
pub struct IncrementalEvalFields {
    pub midgame_eval: WhiteEval,
    pub endgame_eval: WhiteEval,
    pub phase_value: i16,
}

impl IncrementalEvalFields {
    pub fn set_at(&mut self, sq: Square, piece: Piece) {
        let (mg, eg) = piece_square_tables::piece_contributions(sq, piece);
        let phase_value_diff = piece_square_tables::piece_phase_value_contribution(piece.kind);

        self.midgame_eval += mg;
        self.endgame_eval += eg;
        self.phase_value += phase_value_diff;
    }

    pub fn remove_at(&mut self, sq: Square, piece: Piece) {
        let (mg, eg) = piece_square_tables::piece_contributions(sq, piece);
        let phase_value_diff = piece_square_tables::piece_phase_value_contribution(piece.kind);

        self.midgame_eval -= mg;
        self.endgame_eval -= eg;
        self.phase_value -= phase_value_diff;
    }
}

impl IncrementalEvalFields {
    pub fn init(game: &Game) -> Self {
        let (midgame_eval, endgame_eval) = piece_square_tables::phase_evals(&game.board);
        let phase_value = piece_square_tables::phase_value(&game.board);

        Self {
            midgame_eval,
            endgame_eval,
            phase_value,
        }
    }
}

pub fn eval(game: &EngineGame) -> Eval {
    let absolute_eval = absolute_eval(game);
    Eval::from_white_eval(absolute_eval, game.game.player)
}

pub fn absolute_eval(game: &EngineGame) -> WhiteEval {
    piece_square_tables::tapered_eval(
        game.incremental_eval.phase_value,
        game.incremental_eval.midgame_eval,
        game.incremental_eval.endgame_eval,
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

pub fn eval_components(game: &EngineGame) -> EvalComponents {
    let eval = absolute_eval(game);

    let (midgame_pst, endgame_pst) = piece_square_tables::phase_evals(&game.game.board);
    let phase_value = piece_square_tables::phase_value(&game.game.board);

    let pst_eval = piece_square_tables::tapered_eval(phase_value, midgame_pst, endgame_pst);

    EvalComponents {
        eval,
        piece_square_midgame: midgame_pst,
        piece_square_endgame: endgame_pst,
        phase_value,
        piece_square_tables: pst_eval,
    }
}
