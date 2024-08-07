use crate::chess::player::Player;
use crate::chess::square::Rank;
use crate::engine::eval::{IncrementalEvalFields, PhasedEval};

const DOUBLED_PAWN_MALUS: PhasedEval = PhasedEval::new(-20, -20);

fn doubled_pawn_malus(incremental_eval_fields: &IncrementalEvalFields) -> PhasedEval {
    let mut eval = PhasedEval::ZERO;

    for _ in
        incremental_eval_fields.doubled_pawn_files[Player::White.array_idx()] & Rank::R1.bitboard()
    {
        eval += DOUBLED_PAWN_MALUS;
    }

    for _ in
        incremental_eval_fields.doubled_pawn_files[Player::White.array_idx()] & Rank::R1.bitboard()
    {
        eval -= DOUBLED_PAWN_MALUS;
    }

    eval
}

pub fn eval(incremental_eval_fields: &IncrementalEvalFields) -> PhasedEval {
    doubled_pawn_malus(incremental_eval_fields)
}
