use crate::chess::bitboard::{bitboards, Bitboard};
use crate::chess::piece::PieceKind;
use crate::chess::square::Square;
use crate::engine::eval::{Parameters, PhasedEval};

pub fn print_param(f: &mut std::fmt::Formatter<'_>, p: PhasedEval) -> std::fmt::Result {
    let (mg, eg) = (p.midgame().0, p.endgame().0);
    write!(f, "s({mg: >5}, {eg: >5})")
}

pub fn print_array(
    f: &mut std::fmt::Formatter<'_>,
    ps: &[PhasedEval],
    name: &str,
) -> std::fmt::Result {
    let size = ps.len();
    writeln!(f, "pub const {name}: [PhasedEval; {size}] = [")?;

    for param in ps {
        write!(f, "    ")?;
        print_param(f, *param)?;
        writeln!(f, ",")?;
    }

    writeln!(f, "];\n")?;

    Ok(())
}

pub fn print_pst(
    f: &mut std::fmt::Formatter<'_>,
    pst: &[PhasedEval],
    name: &str,
) -> std::fmt::Result {
    assert_eq!(pst.len(), Square::N);

    writeln!(f, "pub const {name}: PieceSquareTableDefinition = [")?;

    for rank in (0..8).rev() {
        write!(f, "    [")?;

        for file in 0..8 {
            let idx = Square::from_idxs(file, rank).array_idx();
            print_param(f, pst[idx])?;

            if file != 7 {
                write!(f, ", ")?;
            }
        }

        writeln!(f, "],")?;
    }

    writeln!(f, "];\n")?;

    Ok(())
}

pub fn print_single(
    f: &mut std::fmt::Formatter<'_>,
    p: &[PhasedEval],
    name: &str,
) -> std::fmt::Result {
    assert_eq!(p.len(), 1);

    write!(f, "pub const {name}: PhasedEval = ")?;
    print_param(f, p[0])?;
    writeln!(f, ";\n")?;

    Ok(())
}

impl Parameters {
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

    pub fn rebalance(&mut self) {
        Self::rebalance_pst(
            &mut self.pawn_pst,
            &mut self.material,
            PieceKind::Pawn,
            bitboards::RANK_1 | bitboards::RANK_8,
        );
        Self::rebalance_pst(
            &mut self.knight_pst,
            &mut self.material,
            PieceKind::Knight,
            Bitboard::EMPTY,
        );
        Self::rebalance_pst(
            &mut self.bishop_pst,
            &mut self.material,
            PieceKind::Bishop,
            Bitboard::EMPTY,
        );
        Self::rebalance_pst(
            &mut self.rook_pst,
            &mut self.material,
            PieceKind::Rook,
            Bitboard::EMPTY,
        );
        Self::rebalance_pst(
            &mut self.queen_pst,
            &mut self.material,
            PieceKind::Queen,
            Bitboard::EMPTY,
        );
    }
}
