use engine::chess::bitboard::{Bitboard, bitboards};
use engine::chess::piece::PieceKind;
use engine::chess::square::Square;
use engine::engine::eval::{Parameters, PhasedEval};

pub trait RebalanceParameters {
    fn rebalance(&mut self);
}

fn rebalance_pst(
    pst: &mut [PhasedEval; Square::N],
    material: &mut [PhasedEval; PieceKind::N],
    piece: PieceKind,
    ignore_mask: Bitboard,
) {
    let mut midgame_sum = 0;
    let mut endgame_sum = 0;

    // We won't include some squares in the calculation - e.g. ranks where pawns can never be
    let squares = Bitboard::FULL & !ignore_mask;

    // First, calculate the average across all non-zero squares in the PST
    // Do our computations in 32 bits to avoid overflowing i16 for large PST values.
    for sq in squares {
        let v = pst[sq.array_idx()];
        midgame_sum += i32::from(v.midgame().0);
        endgame_sum += i32::from(v.endgame().0);
    }

    let mg_average = midgame_sum / i32::from(squares.count());
    let eg_average = endgame_sum / i32::from(squares.count());

    let average = PhasedEval::new(
        i16::try_from(mg_average).unwrap(),
        i16::try_from(eg_average).unwrap(),
    );
    material[piece.array_idx()] += average;

    for sq in squares {
        pst[sq.array_idx()] -= average;
    }
}

impl RebalanceParameters for Parameters {
    fn rebalance(&mut self) {
        rebalance_pst(
            &mut self.pawn_pst,
            &mut self.material,
            PieceKind::Pawn,
            bitboards::RANK_1 | bitboards::RANK_8,
        );
        rebalance_pst(
            &mut self.knight_pst,
            &mut self.material,
            PieceKind::Knight,
            Bitboard::EMPTY,
        );
        rebalance_pst(
            &mut self.bishop_pst,
            &mut self.material,
            PieceKind::Bishop,
            Bitboard::EMPTY,
        );
        rebalance_pst(
            &mut self.rook_pst,
            &mut self.material,
            PieceKind::Rook,
            Bitboard::EMPTY,
        );
        rebalance_pst(
            &mut self.queen_pst,
            &mut self.material,
            PieceKind::Queen,
            Bitboard::EMPTY,
        );
    }
}
