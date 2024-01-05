use crate::chess::piece::{Piece, PieceKind};
use crate::chess::{game::Game, moves::Move};
use std::cmp::Ordering;

const PIECE_ORDER: [i16; PieceKind::N] = [0, 1, 2, 3, 4, 5];

pub fn order_moves(game: &Game, moves: &mut [Move], previous_best_move: Option<Move>) {
    moves.sort_unstable_by(|m1, m2| {
        // If there was some best move in this situation previously, always search it first
        if let Some(m) = previous_best_move {
            if m == *m1 {
                return Ordering::Less;
            }
        }

        let m1_victim = game.board.piece_at(m1.dst);
        let m2_victim = game.board.piece_at(m2.dst);

        match (m1_victim, m2_victim) {
            (None, None) => Ordering::Equal,
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (Some(p1), Some(p2)) => {
                let m1_attacker = game.board.piece_at(m1.src).unwrap();
                let m2_attacker = game.board.piece_at(m2.src).unwrap();

                mvv_lva_ordering(p1, m1_attacker, p2, m2_attacker)
            }
        }
    });
}

// When deciding which captures to explore first, the most likely captures to be valuable
// are those which capture a very valuable piece with a not-so-valuable piece. The best
// to check first is the most valuable victim (MVV) with the least valuable attacker (LVA).
fn mvv_lva_ordering(
    m1_victim: Piece,
    m1_attacker: Piece,
    m2_victim: Piece,
    m2_attacker: Piece,
) -> Ordering {
    match PIECE_ORDER[m1_victim.kind.array_idx()].cmp(&PIECE_ORDER[m2_victim.kind.array_idx()]) {
        Ordering::Less => Ordering::Greater,
        Ordering::Equal => PIECE_ORDER[m1_attacker.kind.array_idx()]
            .cmp(&PIECE_ORDER[m2_attacker.kind.array_idx()]),
        Ordering::Greater => Ordering::Less,
    }
}
