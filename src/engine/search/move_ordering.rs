use crate::chess::piece::PieceKind;
use crate::chess::square::Square;
use crate::chess::{game::Game, moves::Move};

// Sentinel values
const PREVIOUS_BEST_MOVE_SCORE: i32 = 200_000;
const CAPTURE_SCORE: i32 = 100_000;
const KILLER_MOVE_1_SCORE: i32 = 90001;
const KILLER_MOVE_2_SCORE: i32 = 90000;
const QUIET_SCORE: i32 = 0;

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
const PIECES: i32 = PieceKind::N as i32;

const MVV_ORDER: [i32; PieceKind::N] = [0, PIECES, PIECES * 2, PIECES * 3, PIECES * 4, PIECES * 5];
const LVA_ORDER: [i32; PieceKind::N] = [5, 4, 3, 2, 1, 0];

pub fn score_move(
    game: &Game,
    mv: Move,
    previous_best_move: Option<Move>,
    killer_moves: [Option<Move>; 2],
    history: &[[i32; Square::N]; Square::N],
) -> i32 {
    if previous_best_move == Some(mv) {
        return PREVIOUS_BEST_MOVE_SCORE;
    }

    if killer_moves[0] == Some(mv) {
        return KILLER_MOVE_1_SCORE;
    }

    if killer_moves[1] == Some(mv) {
        return KILLER_MOVE_2_SCORE;
    }

    let captured_piece = game.board.piece_at(mv.dst);

    if let Some(captured_piece) = captured_piece {
        let moved_piece = game.board.piece_at(mv.src).unwrap();

        let victim_score = MVV_ORDER[captured_piece.kind.array_idx()];
        let attacker_score = LVA_ORDER[moved_piece.kind.array_idx()];

        let mvv_lva = victim_score + attacker_score;

        return CAPTURE_SCORE + mvv_lva;
    }

    QUIET_SCORE + history[mv.src.array_idx()][mv.dst.array_idx()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::game::Game;
    use crate::chess::square::squares::all::*;

    struct ScoredMove {
        pub mv: Move,
        pub score: i32,
    }

    impl ScoredMove {
        pub fn new(mv: Move) -> Self {
            Self { mv, score: 0 }
        }
    }

    #[test]
    fn test_mvv_lva() {
        crate::init();

        let game = Game::from_fen("k3B3/8/n1q1R1r1/1P6/1NQn4/7P/2r5/5K2 w - - 0 1").unwrap();
        let mut moves: Vec<ScoredMove> = game
            .moves()
            .to_vec()
            .into_iter()
            .map(ScoredMove::new)
            .collect();

        for mv in &mut moves {
            mv.score = score_move(&game, mv.mv, None, [None; 2], &[[0; Square::N]; Square::N]);
        }

        moves.sort_unstable_by_key(|m| -m.score);

        assert_eq!(moves[0].mv, Move::new(B5, C6)); // PxQ
        assert_eq!(moves[1].mv, Move::new(B4, C6)); // NxQ
        assert_eq!(moves[2].mv, Move::new(E8, C6)); // BxQ
        assert_eq!(moves[3].mv, Move::new(E6, C6)); // RxQ
        assert_eq!(moves[4].mv, Move::new(C4, C6)); // QxQ
        assert_eq!(moves[5].mv, Move::new(B4, C2)); // NxR
        assert_eq!(moves[6].mv, Move::new(E8, G6)); // BxR
        assert_eq!(moves[7].mv, Move::new(E6, G6)); // RxR
        assert_eq!(moves[8].mv, Move::new(C4, C2)); // QxR
        assert_eq!(moves[9].mv, Move::new(B5, A6)); // PxN
        assert_eq!(moves[10].mv, Move::new(B4, A6)); // NxN
        assert_eq!(moves[11].mv, Move::new(C4, D4)); // QxN
    }
}
