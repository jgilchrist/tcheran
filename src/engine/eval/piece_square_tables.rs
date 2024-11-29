use super::PhasedEval;
use crate::chess::board::Board;
use crate::chess::piece::Piece;
use crate::chess::player::{ByPlayer, Player};
use crate::chess::square::Square;
use crate::chess::{
    piece::PieceKind,
    square::{File, Rank},
};
use crate::engine::eval::phased_eval::s;

type PieceSquareTableDefinition = [[PhasedEval; File::N]; Rank::N];
type PieceSquareTable = [PhasedEval; Square::N];
type PieceValueTables = [[PieceSquareTable; PieceKind::N]; Player::N];

const PIECE_VALUES: [PhasedEval; PieceKind::N] = [
    s(82, 94),
    s(337, 281),
    s(365, 297),
    s(477, 512),
    s(1025, 936),
    s(0, 0),
];

#[rustfmt::skip]
mod tables {
    use super::*;

    // Piece square tables from https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function

    pub const PAWNS: PieceSquareTableDefinition = [
        [s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0)],
        [s(   98,   178), s(  134,   173), s(   61,   158), s(   95,   134), s(   68,   147), s(  126,   132), s(   34,   165), s(  -11,   187)],
        [s(   -6,    94), s(    7,   100), s(   26,    85), s(   31,    67), s(   65,    56), s(   56,    53), s(   25,    82), s(  -20,    84)],
        [s(  -14,    32), s(   13,    24), s(    6,    13), s(   21,     5), s(   23,    -2), s(   12,     4), s(   17,    17), s(  -23,    17)],
        [s(  -27,    13), s(   -2,     9), s(   -5,    -3), s(   12,    -7), s(   17,    -7), s(    6,    -8), s(   10,     3), s(  -25,    -1)],
        [s(  -26,     4), s(   -4,     7), s(   -4,    -6), s(  -10,     1), s(    3,     0), s(    3,    -5), s(   33,    -1), s(  -12,    -8)],
        [s(  -35,    13), s(   -1,     8), s(  -20,     8), s(  -23,    10), s(  -15,    13), s(   24,     0), s(   38,     2), s(  -22,    -7)],
        [s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0), s(    0,     0)],
    ];

    pub const KNIGHTS: PieceSquareTableDefinition = [
        [s( -167,   -58), s(  -89,   -38), s(  -34,   -13), s(  -49,   -28), s(   61,   -31), s(  -97,   -27), s(  -15,   -63), s( -107,   -99)],
        [s(  -73,   -25), s(  -41,    -8), s(   72,   -25), s(   36,    -2), s(   23,    -9), s(   62,   -25), s(    7,   -24), s(  -17,   -52)],
        [s(  -47,   -24), s(   60,   -20), s(   37,    10), s(   65,     9), s(   84,    -1), s(  129,    -9), s(   73,   -19), s(   44,   -41)],
        [s(   -9,   -17), s(   17,     3), s(   19,    22), s(   53,    22), s(   37,    22), s(   69,    11), s(   18,     8), s(   22,   -18)],
        [s(  -13,   -18), s(    4,    -6), s(   16,    16), s(   13,    25), s(   28,    16), s(   19,    17), s(   21,     4), s(   -8,   -18)],
        [s(  -23,   -23), s(   -9,    -3), s(   12,    -1), s(   10,    15), s(   19,    10), s(   17,    -3), s(   25,   -20), s(  -16,   -22)],
        [s(  -29,   -42), s(  -53,   -20), s(  -12,   -10), s(   -3,    -5), s(   -1,    -2), s(   18,   -20), s(  -14,   -23), s(  -19,   -44)],
        [s( -105,   -29), s(  -21,   -51), s(  -58,   -23), s(  -33,   -15), s(  -17,   -22), s(  -28,   -18), s(  -19,   -50), s(  -23,   -64)],
    ];

    pub const BISHOPS: PieceSquareTableDefinition = [
        [s(  -29,   -14), s(    4,   -21), s(  -82,   -11), s(  -37,    -8), s(  -25,    -7), s(  -42,    -9), s(    7,   -17), s(   -8,   -24)],
        [s(  -26,    -8), s(   16,    -4), s(  -18,     7), s(  -13,   -12), s(   30,    -3), s(   59,   -13), s(   18,    -4), s(  -47,   -14)],
        [s(  -16,     2), s(   37,    -8), s(   43,     0), s(   40,    -1), s(   35,    -2), s(   50,     6), s(   37,     0), s(   -2,     4)],
        [s(   -4,    -3), s(    5,     9), s(   19,    12), s(   50,     9), s(   37,    14), s(   37,    10), s(    7,     3), s(   -2,     2)],
        [s(   -6,    -6), s(   13,     3), s(   13,    13), s(   26,    19), s(   34,     7), s(   12,    10), s(   10,    -3), s(    4,    -9)],
        [s(    0,   -12), s(   15,    -3), s(   15,     8), s(   15,    10), s(   14,    13), s(   27,     3), s(   18,    -7), s(   10,   -15)],
        [s(    4,   -14), s(   15,   -18), s(   16,    -7), s(    0,    -1), s(    7,     4), s(   21,    -9), s(   33,   -15), s(    1,   -27)],
        [s(  -33,   -23), s(   -3,    -9), s(  -14,   -23), s(  -21,    -5), s(  -13,    -9), s(  -12,   -16), s(  -39,    -5), s(  -21,   -17)],
    ];

    pub const ROOKS: PieceSquareTableDefinition = [
        [s(   32,    13), s(   42,    10), s(   32,    18), s(   51,    15), s(   63,    12), s(    9,    12), s(   31,     8), s(   43,     5)],
        [s(   27,    11), s(   32,    13), s(   58,    13), s(   62,    11), s(   80,    -3), s(   67,     3), s(   26,     8), s(   44,     3)],
        [s(   -5,     7), s(   19,     7), s(   26,     7), s(   36,     5), s(   17,     4), s(   45,    -3), s(   61,    -5), s(   16,    -3)],
        [s(  -24,     4), s(  -11,     3), s(    7,    13), s(   26,     1), s(   24,     2), s(   35,     1), s(   -8,    -1), s(  -20,     2)],
        [s(  -36,     3), s(  -26,     5), s(  -12,     8), s(   -1,     4), s(    9,    -5), s(   -7,    -6), s(    6,    -8), s(  -23,   -11)],
        [s(  -45,    -4), s(  -25,     0), s(  -16,    -5), s(  -17,    -1), s(    3,    -7), s(    0,   -12), s(   -5,    -8), s(  -33,   -16)],
        [s(  -44,    -6), s(  -16,    -6), s(  -20,     0), s(   -9,     2), s(   -1,    -9), s(   11,    -9), s(   -6,   -11), s(  -71,    -3)],
        [s(  -19,    -9), s(  -13,     2), s(    1,     3), s(   17,    -1), s(   16,    -5), s(    7,   -13), s(  -37,     4), s(  -26,   -20)],
    ];

    pub const QUEENS: PieceSquareTableDefinition = [
        [s(  -28,    -9), s(    0,    22), s(   29,    22), s(   12,    27), s(   59,    27), s(   44,    19), s(   43,    10), s(   45,    20)],
        [s(  -24,   -17), s(  -39,    20), s(   -5,    32), s(    1,    41), s(  -16,    58), s(   57,    25), s(   28,    30), s(   54,     0)],
        [s(  -13,   -20), s(  -17,     6), s(    7,     9), s(    8,    49), s(   29,    47), s(   56,    35), s(   47,    19), s(   57,     9)],
        [s(  -27,     3), s(  -27,    22), s(  -16,    24), s(  -16,    45), s(   -1,    57), s(   17,    40), s(   -2,    57), s(    1,    36)],
        [s(   -9,   -18), s(  -26,    28), s(   -9,    19), s(  -10,    47), s(   -2,    31), s(   -4,    34), s(    3,    39), s(   -3,    23)],
        [s(  -14,   -16), s(    2,   -27), s(  -11,    15), s(   -2,     6), s(   -5,     9), s(    2,    17), s(   14,    10), s(    5,     5)],
        [s(  -35,   -22), s(   -8,   -23), s(   11,   -30), s(    2,   -16), s(    8,   -16), s(   15,   -23), s(   -3,   -36), s(    1,   -32)],
        [s(   -1,   -33), s(  -18,   -28), s(   -9,   -22), s(   10,   -43), s(  -15,    -5), s(  -25,   -32), s(  -31,   -20), s(  -50,   -41)],
    ];

    pub const KING: PieceSquareTableDefinition = [
        [s(  -65,   -74), s(   23,   -35), s(   16,   -18), s(  -15,   -18), s(  -56,   -11), s(  -34,    15), s(    2,     4), s(   13,   -17)],
        [s(   29,   -12), s(   -1,    17), s(  -20,    14), s(   -7,    17), s(   -8,    17), s(   -4,    38), s(  -38,    23), s(  -29,    11)],
        [s(   -9,    10), s(   24,    17), s(    2,    23), s(  -16,    15), s(  -20,    20), s(    6,    45), s(   22,    44), s(  -22,    13)],
        [s(  -17,    -8), s(  -20,    22), s(  -12,    24), s(  -27,    27), s(  -30,    26), s(  -25,    33), s(  -14,    26), s(  -36,     3)],
        [s(  -49,   -18), s(   -1,    -4), s(  -27,    21), s(  -39,    24), s(  -46,    27), s(  -44,    23), s(  -33,     9), s(  -51,   -11)],
        [s(  -14,   -19), s(  -14,    -3), s(  -22,    11), s(  -46,    21), s(  -44,    23), s(  -30,    16), s(  -15,     7), s(  -27,    -9)],
        [s(    1,   -27), s(    7,   -11), s(   -8,     4), s(  -64,    13), s(  -43,    14), s(  -16,     4), s(    9,    -5), s(    8,   -17)],
        [s(  -15,   -53), s(   36,   -34), s(   12,   -21), s(  -54,   -11), s(    8,   -28), s(  -28,   -14), s(   24,   -24), s(   14,   -43)],
    ];

    // These need to be initialised when we start up, since they can
    // be derived from the white tables.
    pub static mut TABLES: PieceValueTables = [[[PhasedEval::ZERO; Square::N]; PieceKind::N]; Player::N];

    pub fn negate(t: PieceSquareTable) -> PieceSquareTable {
        let mut new_table: PieceSquareTable = [PhasedEval::ZERO; Square::N];

        for i in 0..Square::N {
            new_table[i] = -t[i];
        }

        new_table
    }
    
    pub fn add_material(t: PieceSquareTable, p: PieceKind) -> PieceSquareTable {
        let mut new_table: PieceSquareTable = [PhasedEval::ZERO; Square::N];
        let material_value = tables::PIECE_VALUES[p.array_idx()];

        for i in 0..Square::N {
            new_table[i] = t[i] + material_value;
        }

        new_table
    }

    pub fn flip(t: PieceSquareTableDefinition) -> PieceSquareTableDefinition {
        let mut new_table: PieceSquareTableDefinition = [[PhasedEval::ZERO; File::N]; Rank::N];

        for i in 0..Rank::N {
            new_table[i] = t[Rank::N - i - 1];
        }

        new_table
    }

    pub fn flatten(definition: PieceSquareTableDefinition) -> PieceSquareTable {
        let mut new_table: PieceSquareTable = [PhasedEval::ZERO; Square::N];

        for i in 0..Square::N {
            new_table[i] = definition[i / File::N][i % Rank::N];
        }

        new_table
    }
}

#[rustfmt::skip]
pub fn init() {
    use tables::*;

    fn white_pst(def: PieceSquareTableDefinition, piece: PieceKind) -> PieceSquareTable {
        add_material(flatten(flip(def)), piece)
    }

    fn black_pst(def: PieceSquareTableDefinition, piece: PieceKind) -> PieceSquareTable {
        negate(add_material(flatten(def), piece))
    }

    unsafe {
        TABLES[Player::White.array_idx()][PieceKind::Pawn.array_idx()] = white_pst(PAWNS, PieceKind::Pawn);
        TABLES[Player::White.array_idx()][PieceKind::Knight.array_idx()] = white_pst(KNIGHTS, PieceKind::Knight);
        TABLES[Player::White.array_idx()][PieceKind::Bishop.array_idx()] = white_pst(BISHOPS, PieceKind::Bishop);
        TABLES[Player::White.array_idx()][PieceKind::Rook.array_idx()] = white_pst(ROOKS, PieceKind::Rook);
        TABLES[Player::White.array_idx()][PieceKind::Queen.array_idx()] = white_pst(QUEENS, PieceKind::Queen);
        TABLES[Player::White.array_idx()][PieceKind::King.array_idx()] = white_pst(KING, PieceKind::King);

        TABLES[Player::Black.array_idx()][PieceKind::Pawn.array_idx()] = black_pst(PAWNS, PieceKind::Pawn);
        TABLES[Player::Black.array_idx()][PieceKind::Knight.array_idx()] = black_pst(KNIGHTS, PieceKind::Knight);
        TABLES[Player::Black.array_idx()][PieceKind::Bishop.array_idx()] = black_pst(BISHOPS, PieceKind::Bishop);
        TABLES[Player::Black.array_idx()][PieceKind::Rook.array_idx()] = black_pst(ROOKS, PieceKind::Rook);
        TABLES[Player::Black.array_idx()][PieceKind::Queen.array_idx()] = black_pst(QUEENS, PieceKind::Queen);
        TABLES[Player::Black.array_idx()][PieceKind::King.array_idx()] = black_pst(KING, PieceKind::King);
    }
}

#[inline(always)]
pub fn piece_contributions(square: Square, piece: Piece) -> PhasedEval {
    // Safe as idx is guaranteed to be in bounds - we have length 64 arrays and are
    // generating idx from Square
    unsafe { tables::TABLES[piece.player.array_idx()][piece.kind.array_idx()][square.array_idx()] }
}

pub fn eval(board: &Board) -> PhasedEval {
    let mut eval = PhasedEval::ZERO;

    for idx in 0..Square::N {
        let square = Square::from_array_index(idx);

        let maybe_piece = board.piece_at(square);

        if let Some(piece) = maybe_piece {
            eval += piece_contributions(square, piece);
        }
    }

    eval
}

pub fn eval_by_player(board: &Board) -> ByPlayer<PhasedEval> {
    let mut white_eval = PhasedEval::ZERO;
    let mut black_eval = PhasedEval::ZERO;

    for idx in 0..Square::N {
        let square = Square::from_array_index(idx);

        let maybe_piece = board.piece_at(square);

        if let Some(piece) = maybe_piece {
            match piece.player {
                Player::White => white_eval += piece_contributions(square, piece),
                Player::Black => black_eval += piece_contributions(square, piece),
            }
        }
    }

    ByPlayer::new(white_eval, black_eval)
}
