use chess::board::Board;
use chess::piece::Piece;
use chess::player::Player;
use chess::square::Square;
use chess::{
    piece::PieceKind,
    square::{File, Rank},
};

use super::WhiteEval;

type PieceValueTableDefinition = [[i16; File::N]; Rank::N];
type PieceValueTable = [i16; Square::N];
type PieceValueTables = [[PieceValueTable; PieceKind::N]; Player::N];

fn midgame_piece_value(kind: PieceKind) -> i16 {
    match kind {
        PieceKind::Pawn => 82,
        PieceKind::Knight => 337,
        PieceKind::Bishop => 365,
        PieceKind::Rook => 477,
        PieceKind::Queen => 1025,
        PieceKind::King => 0,
    }
}

fn endgame_piece_value(kind: PieceKind) -> i16 {
    match kind {
        PieceKind::Pawn => 94,
        PieceKind::Knight => 281,
        PieceKind::Bishop => 297,
        PieceKind::Rook => 512,
        PieceKind::Queen => 936,
        PieceKind::King => 0,
    }
}

const PHASE_COUNT_MAX: i64 = 24;

pub fn piece_phase_value_contribution(kind: PieceKind) -> i16 {
    match kind {
        PieceKind::Pawn | PieceKind::King => 0,
        PieceKind::Knight | PieceKind::Bishop => 1,
        PieceKind::Rook => 2,
        PieceKind::Queen => 4,
    }
}

pub fn phase_value(board: &Board) -> i16 {
    let mut v = 0;

    for idx in 0..Square::N {
        let maybe_piece = board.pieces[idx];

        if let Some(piece) = maybe_piece {
            v += piece_phase_value_contribution(piece.kind);
        }
    }

    v
}

pub(super) fn tapered_eval(
    phase_value: i16,
    midgame_eval: WhiteEval,
    endgame_eval: WhiteEval,
) -> WhiteEval {
    // Switch to 64 bit calculations to avoid overflow
    let phase_value = i64::from(phase_value);

    let midgame_phase_value = phase_value.min(PHASE_COUNT_MAX);
    let endgame_phase_value = PHASE_COUNT_MAX - phase_value;

    let midgame_eval = i64::from(midgame_eval.0);
    let endgame_eval = i64::from(endgame_eval.0);

    let eval = (midgame_eval * midgame_phase_value + endgame_eval * endgame_phase_value) / 24;
    WhiteEval(i16::try_from(eval).unwrap())
}

#[rustfmt::skip]
mod tables {
    use super::*;

    // Piece square tables from https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function

    pub const PAWNS_MIDGAME: PieceValueTableDefinition = [
        [    0,    0,    0,    0,    0,    0,    0,    0],
        [   98,  134,   61,   95,   68,  126,   34,  -11],
        [   -6,    7,   26,   31,   65,   56,   25,  -20],
        [  -14,   13,    6,   21,   23,   12,   17,  -23],
        [  -27,   -2,   -5,   12,   17,    6,   10,  -25],
        [  -26,   -4,   -4,  -10,    3,    3,   33,  -12],
        [  -35,   -1,  -20,  -23,  -15,   24,   38,  -22],
        [    0,    0,    0,    0,    0,    0,    0,    0],
    ];

    pub const PAWNS_ENDGAME: PieceValueTableDefinition = [
        [    0,    0,    0,    0,    0,    0,    0,    0],
        [  178,  173,  158,  134,  147,  132,  165, 187],
        [   94,  100,   85,   67,   56,   53,   82,   84],
        [   32,   24,   13,    5,   -2,    4,   17,   17],
        [   13,    9,   -3,   -7,   -7,   -8,    3,   -1],
        [    4,    7,   -6,    1,    0,   -5,   -1,   -8],
        [   13,    8,    8,   10,   13,    0,    2,   -7],
        [    0,    0,    0,    0,    0,    0,    0,    0],
    ];

    pub const KNIGHTS_MIDGAME: PieceValueTableDefinition = [
        [ -167,  -89,  -34,  -49,   61,  -97,  -15, -107],
        [  -73,  -41,   72,   36,   23,   62,    7,  -17],
        [  -47,   60,   37,   65,   84,  129,   73,   44],
        [   -9,   17,   19,   53,   37,   69,   18,   22],
        [  -13,    4,   16,   13,   28,   19,   21,   -8],
        [  -23,   -9,   12,   10,   19,   17,   25,  -16],
        [  -29,  -53,  -12,   -3,   -1,   18,  -14,  -19],
        [ -105,  -21,  -58,  -33,  -17,  -28,  -19,  -23],
    ];

    pub const KNIGHTS_ENDGAME: PieceValueTableDefinition = [
        [  -58,  -38,  -13,  -28,  -31,  -27,  -63,  -99],
        [  -25,   -8,  -25,   -2,   -9,  -25,  -24,  -52],
        [  -24,  -20,   10,    9,   -1,   -9,  -19,  -41],
        [  -17,    3,   22,   22,   22,   11,    8,  -18],
        [  -18,   -6,   16,   25,   16,   17,    4,  -18],
        [  -23,   -3,   -1,   15,   10,   -3,  -20,  -22],
        [  -42,  -20,  -10,   -5,   -2,  -20,  -23,  -44],
        [  -29,  -51,  -23,  -15,  -22,  -18,  -50,  -64],
    ];

    pub const BISHOPS_MIDGAME: PieceValueTableDefinition = [
        [  -29,    4,  -82,  -37,  -25,  -42,    7,   -8],
        [  -26,   16,  -18,  -13,   30,   59,   18,  -47],
        [  -16,   37,   43,   40,   35,   50,   37,   -2],
        [   -4,    5,   19,   50,   37,   37,    7,   -2],
        [   -6,   13,   13,   26,   34,   12,   10,    4],
        [    0,   15,   15,   15,   14,   27,   18,   10],
        [    4,   15,   16,    0,    7,   21,   33,    1],
        [  -33,   -3,  -14,  -21,  -13,  -12,  -39,  -21],
    ];

    pub const BISHOPS_ENDGAME: PieceValueTableDefinition = [
        [  -14,  -21,  -11,   -8,   -7,   -9,  -17,  -24],
        [   -8,   -4,    7,  -12,   -3,  -13,   -4,  -14],
        [    2,   -8,    0,   -1,   -2,    6,    0,    4],
        [   -3,    9,   12,    9,   14,   10,    3,    2],
        [   -6,    3,   13,   19,    7,   10,   -3,   -9],
        [  -12,   -3,    8,   10,   13,    3,   -7,  -15],
        [  -14,  -18,   -7,   -1,    4,   -9,  -15,  -27],
        [  -23,   -9,  -23,   -5,   -9,  -16,   -5,  -17],
    ];

    pub const ROOKS_MIDGAME: PieceValueTableDefinition = [
        [   32,   42,   32,   51,   63,    9,   31,   43],
        [   27,   32,   58,   62,   80,   67,   26,   44],
        [   -5,   19,   26,   36,   17,   45,   61,   16],
        [  -24,  -11,    7,   26,   24,   35,   -8,  -20],
        [  -36,  -26,  -12,   -1,    9,   -7,    6,  -23],
        [  -45,  -25,  -16,  -17,    3,    0,   -5,  -33],
        [  -44,  -16,  -20,   -9,   -1,   11,   -6,  -71],
        [  -19,  -13,    1,   17,   16,    7,  -37,  -26],
    ];

    pub const ROOKS_ENDGAME: PieceValueTableDefinition = [
        [   13,   10,   18,   15,   12,   12,    8,    5],
        [   11,   13,   13,   11,   -3,    3,    8,    3],
        [    7,    7,    7,    5,    4,   -3,   -5,   -3],
        [    4,    3,   13,    1,    2,    1,   -1,    2],
        [    3,    5,    8,    4,   -5,   -6,   -8,  -11],
        [   -4,    0,   -5,   -1,   -7,  -12,   -8,  -16],
        [   -6,   -6,    0,    2,   -9,   -9,  -11,   -3],
        [   -9,    2,    3,   -1,   -5,  -13,    4,  -20],
    ];

    pub const QUEENS_MIDGAME: PieceValueTableDefinition = [
        [  -28,    0,   29,   12,   59,   44,   43,   45],
        [  -24,  -39,   -5,    1,  -16,   57,   28,   54],
        [  -13,  -17,    7,    8,   29,   56,   47,   57],
        [  -27,  -27,  -16,  -16,   -1,   17,   -2,    1],
        [   -9,  -26,   -9,  -10,   -2,   -4,    3,   -3],
        [  -14,    2,  -11,   -2,   -5,    2,   14,    5],
        [  -35,   -8,   11,    2,    8,   15,   -3,    1],
        [   -1,  -18,   -9,   10,  -15,  -25,  -31,  -50],
    ];

    pub const QUEENS_ENDGAME: PieceValueTableDefinition = [
        [   -9,   22,   22,   27,   27,   19,   10,   20],
        [  -17,   20,   32,   41,   58,   25,   30,    0],
        [  -20,    6,    9,   49,   47,   35,   19,    9],
        [    3,   22,   24,   45,   57,   40,   57,   36],
        [  -18,   28,   19,   47,   31,   34,   39,   23],
        [  -16,  -27,   15,    6,    9,   17,   10,    5],
        [  -22,  -23,  -30,  -16,  -16,  -23,  -36,  -32],
        [  -33,  -28,  -22,  -43,   -5,  -32,  -20,  -41],
    ];

    pub const KING_MIDGAME: PieceValueTableDefinition = [
        [  -65,   23,   16,  -15,  -56,  -34,    2,   13],
        [   29,   -1,  -20,   -7,   -8,   -4,  -38,  -29],
        [   -9,   24,    2,  -16,  -20,    6,   22,  -22],
        [  -17,  -20,  -12,  -27,  -30,  -25,  -14,  -36],
        [  -49,   -1,  -27,  -39,  -46,  -44,  -33,  -51],
        [  -14,  -14,  -22,  -46,  -44,  -30,  -15,  -27],
        [    1,    7,   -8,  -64,  -43,  -16,    9,    8],
        [  -15,   36,   12,  -54,    8,  -28,   24,   14],
    ];

    pub const KING_ENDGAME: PieceValueTableDefinition = [
        [  -74,  -35,  -18,  -18,  -11,   15,    4,  -17],
        [  -12,   17,   14,   17,   17,   38,   23,   11],
        [   10,   17,   23,   15,   20,   45,   44,   13],
        [   -8,   22,   24,   27,   26,   33,   26,    3],
        [  -18,   -4,   21,   24,   27,   23,    9,  -11],
        [  -19,   -3,   11,   21,   23,   16,    7,   -9],
        [  -27,  -11,    4,   13,   14,    4,   -5,  -17],
        [  -53,  -34,  -21,  -11,  -28,  -14,  -24,  -43],
    ];

    // These need to be initialised when we start up, since they can
    // be derived from the white tables.
    pub static mut MIDGAME_TABLES: PieceValueTables = [[[0; Square::N]; PieceKind::N]; Player::N];
    pub static mut ENDGAME_TABLES: PieceValueTables = [[[0; Square::N]; PieceKind::N]; Player::N];

    pub fn negate(t: PieceValueTable) -> PieceValueTable {
        let mut new_table: PieceValueTable = [0; Square::N];

        for i in 0..Square::N {
            new_table[i] = -t[i];
        }

        new_table
    }
    
    pub fn add(t: PieceValueTable, v: i16) -> PieceValueTable {
        let mut new_table: PieceValueTable = [0; Square::N];

        for i in 0..Square::N {
            new_table[i] = t[i] + v;
        }

        new_table
    }

    pub fn flip(t: PieceValueTableDefinition) -> PieceValueTableDefinition {
        let mut new_table: PieceValueTableDefinition = Default::default();

        for i in 0..Rank::N {
            new_table[i] = t[Rank::N - i - 1];
        }

        new_table
    }

    pub fn flatten(definition: PieceValueTableDefinition) -> PieceValueTable {
        let mut new_table: PieceValueTable = [0; Square::N];

        for i in 0..Square::N {
            new_table[i] = definition[i / File::N][i % Rank::N];
        }

        new_table
    }
}

pub fn init() {
    use tables::*;

    fn white_pst(def: PieceValueTableDefinition, piece_value: i16) -> PieceValueTable {
        add(flatten(flip(def)), piece_value)
    }

    fn black_pst(def: PieceValueTableDefinition, piece_value: i16) -> PieceValueTable {
        negate(add(flatten(def), piece_value))
    }

    unsafe {
        MIDGAME_TABLES[Player::Black.array_idx()][PieceKind::Pawn.array_idx()] =
            black_pst(PAWNS_MIDGAME, midgame_piece_value(PieceKind::Pawn));
        MIDGAME_TABLES[Player::Black.array_idx()][PieceKind::Knight.array_idx()] =
            black_pst(KNIGHTS_MIDGAME, midgame_piece_value(PieceKind::Knight));
        MIDGAME_TABLES[Player::Black.array_idx()][PieceKind::Bishop.array_idx()] =
            black_pst(BISHOPS_MIDGAME, midgame_piece_value(PieceKind::Bishop));
        MIDGAME_TABLES[Player::Black.array_idx()][PieceKind::Rook.array_idx()] =
            black_pst(ROOKS_MIDGAME, midgame_piece_value(PieceKind::Rook));
        MIDGAME_TABLES[Player::Black.array_idx()][PieceKind::Queen.array_idx()] =
            black_pst(QUEENS_MIDGAME, midgame_piece_value(PieceKind::Queen));
        MIDGAME_TABLES[Player::Black.array_idx()][PieceKind::King.array_idx()] =
            black_pst(KING_MIDGAME, midgame_piece_value(PieceKind::King));

        MIDGAME_TABLES[Player::White.array_idx()][PieceKind::Pawn.array_idx()] =
            white_pst(PAWNS_MIDGAME, midgame_piece_value(PieceKind::Pawn));
        MIDGAME_TABLES[Player::White.array_idx()][PieceKind::Knight.array_idx()] =
            white_pst(KNIGHTS_MIDGAME, midgame_piece_value(PieceKind::Knight));
        MIDGAME_TABLES[Player::White.array_idx()][PieceKind::Bishop.array_idx()] =
            white_pst(BISHOPS_MIDGAME, midgame_piece_value(PieceKind::Bishop));
        MIDGAME_TABLES[Player::White.array_idx()][PieceKind::Rook.array_idx()] =
            white_pst(ROOKS_MIDGAME, midgame_piece_value(PieceKind::Rook));
        MIDGAME_TABLES[Player::White.array_idx()][PieceKind::Queen.array_idx()] =
            white_pst(QUEENS_MIDGAME, midgame_piece_value(PieceKind::Queen));
        MIDGAME_TABLES[Player::White.array_idx()][PieceKind::King.array_idx()] =
            white_pst(KING_MIDGAME, midgame_piece_value(PieceKind::King));

        ENDGAME_TABLES[Player::Black.array_idx()][PieceKind::Pawn.array_idx()] =
            black_pst(PAWNS_ENDGAME, endgame_piece_value(PieceKind::Pawn));
        ENDGAME_TABLES[Player::Black.array_idx()][PieceKind::Knight.array_idx()] =
            black_pst(KNIGHTS_ENDGAME, endgame_piece_value(PieceKind::Knight));
        ENDGAME_TABLES[Player::Black.array_idx()][PieceKind::Bishop.array_idx()] =
            black_pst(BISHOPS_ENDGAME, endgame_piece_value(PieceKind::Bishop));
        ENDGAME_TABLES[Player::Black.array_idx()][PieceKind::Rook.array_idx()] =
            black_pst(ROOKS_ENDGAME, endgame_piece_value(PieceKind::Rook));
        ENDGAME_TABLES[Player::Black.array_idx()][PieceKind::Queen.array_idx()] =
            black_pst(QUEENS_ENDGAME, endgame_piece_value(PieceKind::Queen));
        ENDGAME_TABLES[Player::Black.array_idx()][PieceKind::King.array_idx()] =
            black_pst(KING_ENDGAME, endgame_piece_value(PieceKind::King));

        ENDGAME_TABLES[Player::White.array_idx()][PieceKind::Pawn.array_idx()] =
            white_pst(PAWNS_ENDGAME, endgame_piece_value(PieceKind::Pawn));
        ENDGAME_TABLES[Player::White.array_idx()][PieceKind::Knight.array_idx()] =
            white_pst(KNIGHTS_ENDGAME, endgame_piece_value(PieceKind::Knight));
        ENDGAME_TABLES[Player::White.array_idx()][PieceKind::Bishop.array_idx()] =
            white_pst(BISHOPS_ENDGAME, endgame_piece_value(PieceKind::Bishop));
        ENDGAME_TABLES[Player::White.array_idx()][PieceKind::Rook.array_idx()] =
            white_pst(ROOKS_ENDGAME, endgame_piece_value(PieceKind::Rook));
        ENDGAME_TABLES[Player::White.array_idx()][PieceKind::Queen.array_idx()] =
            white_pst(QUEENS_ENDGAME, endgame_piece_value(PieceKind::Queen));
        ENDGAME_TABLES[Player::White.array_idx()][PieceKind::King.array_idx()] =
            white_pst(KING_ENDGAME, endgame_piece_value(PieceKind::King));
    }
}

#[inline(always)]
pub fn piece_contributions(square: Square, piece: Piece) -> (WhiteEval, WhiteEval) {
    // Safe as idx is guaranteed to be in bounds - we have length 64 arrays and are
    // generating idx from Square

    let midgame_contribution = unsafe {
        tables::MIDGAME_TABLES[piece.player.array_idx()][piece.kind.array_idx()][square.array_idx()]
    };

    let endgame_contribution = unsafe {
        tables::ENDGAME_TABLES[piece.player.array_idx()][piece.kind.array_idx()][square.array_idx()]
    };

    (
        WhiteEval(midgame_contribution),
        WhiteEval(endgame_contribution),
    )
}

pub fn phase_evals(board: &Board) -> (WhiteEval, WhiteEval) {
    let mut midgame_eval = WhiteEval(0);
    let mut endgame_eval = WhiteEval(0);

    for idx in 0..Square::N {
        let maybe_piece = board.pieces[idx];

        if let Some(piece) = maybe_piece {
            let (midgame_contribution, endgame_contribution) =
                piece_contributions(Square::from_array_index(idx), piece);
            midgame_eval += midgame_contribution;
            endgame_eval += endgame_contribution;
        }
    }

    (midgame_eval, endgame_eval)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_calculation_does_not_overflow() {
        let midgame_eval = WhiteEval(2456);
        let endgame_eval = WhiteEval(2393);
        let phase_value = 9;

        let eval = tapered_eval(phase_value, midgame_eval, endgame_eval);
        assert!(eval > WhiteEval(0));
    }
}
