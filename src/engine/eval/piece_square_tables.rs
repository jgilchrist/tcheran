use crate::chess::board::Board;
use crate::chess::piece::Piece;
use crate::chess::player::Player;
use crate::chess::square::Square;
use crate::chess::{
    piece::PieceKind,
    square::{File, Rank},
};

use super::PhasedEval;

type PieceValueTableDefinition = [[i16; File::N]; Rank::N];
type PhasePieceValueTable = [i16; Square::N];
type PhasePieceValueTables = [[PhasePieceValueTable; PieceKind::N]; Player::N];

type PieceValueTable = [PhasedEval; Square::N];
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
    pub static mut TABLES: PieceValueTables = [[[PhasedEval::ZERO; Square::N]; PieceKind::N]; Player::N];

    pub fn negate(t: PhasePieceValueTable) -> PhasePieceValueTable {
        let mut new_table: PhasePieceValueTable = [0; Square::N];

        for i in 0..Square::N {
            new_table[i] = -t[i];
        }

        new_table
    }
    
    pub fn add(t: PhasePieceValueTable, v: i16) -> PhasePieceValueTable {
        let mut new_table: PhasePieceValueTable = [0; Square::N];

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

    pub fn flatten(definition: PieceValueTableDefinition) -> PhasePieceValueTable {
        let mut new_table: PhasePieceValueTable = [0; Square::N];

        for i in 0..Square::N {
            new_table[i] = definition[i / File::N][i % Rank::N];
        }

        new_table
    }
}

pub fn init() {
    use tables::*;

    fn white_pst(def: PieceValueTableDefinition, piece_value: i16) -> PhasePieceValueTable {
        add(flatten(flip(def)), piece_value)
    }

    fn black_pst(def: PieceValueTableDefinition, piece_value: i16) -> PhasePieceValueTable {
        negate(add(flatten(def), piece_value))
    }

    let mut midgame_tables: PhasePieceValueTables = [[[0; Square::N]; PieceKind::N]; Player::N];
    let mut endgame_tables: PhasePieceValueTables = [[[0; Square::N]; PieceKind::N]; Player::N];

    midgame_tables[Player::Black.array_idx()][PieceKind::Pawn.array_idx()] =
        black_pst(PAWNS_MIDGAME, midgame_piece_value(PieceKind::Pawn));
    midgame_tables[Player::Black.array_idx()][PieceKind::Knight.array_idx()] =
        black_pst(KNIGHTS_MIDGAME, midgame_piece_value(PieceKind::Knight));
    midgame_tables[Player::Black.array_idx()][PieceKind::Bishop.array_idx()] =
        black_pst(BISHOPS_MIDGAME, midgame_piece_value(PieceKind::Bishop));
    midgame_tables[Player::Black.array_idx()][PieceKind::Rook.array_idx()] =
        black_pst(ROOKS_MIDGAME, midgame_piece_value(PieceKind::Rook));
    midgame_tables[Player::Black.array_idx()][PieceKind::Queen.array_idx()] =
        black_pst(QUEENS_MIDGAME, midgame_piece_value(PieceKind::Queen));
    midgame_tables[Player::Black.array_idx()][PieceKind::King.array_idx()] =
        black_pst(KING_MIDGAME, midgame_piece_value(PieceKind::King));

    midgame_tables[Player::White.array_idx()][PieceKind::Pawn.array_idx()] =
        white_pst(PAWNS_MIDGAME, midgame_piece_value(PieceKind::Pawn));
    midgame_tables[Player::White.array_idx()][PieceKind::Knight.array_idx()] =
        white_pst(KNIGHTS_MIDGAME, midgame_piece_value(PieceKind::Knight));
    midgame_tables[Player::White.array_idx()][PieceKind::Bishop.array_idx()] =
        white_pst(BISHOPS_MIDGAME, midgame_piece_value(PieceKind::Bishop));
    midgame_tables[Player::White.array_idx()][PieceKind::Rook.array_idx()] =
        white_pst(ROOKS_MIDGAME, midgame_piece_value(PieceKind::Rook));
    midgame_tables[Player::White.array_idx()][PieceKind::Queen.array_idx()] =
        white_pst(QUEENS_MIDGAME, midgame_piece_value(PieceKind::Queen));
    midgame_tables[Player::White.array_idx()][PieceKind::King.array_idx()] =
        white_pst(KING_MIDGAME, midgame_piece_value(PieceKind::King));

    endgame_tables[Player::Black.array_idx()][PieceKind::Pawn.array_idx()] =
        black_pst(PAWNS_ENDGAME, endgame_piece_value(PieceKind::Pawn));
    endgame_tables[Player::Black.array_idx()][PieceKind::Knight.array_idx()] =
        black_pst(KNIGHTS_ENDGAME, endgame_piece_value(PieceKind::Knight));
    endgame_tables[Player::Black.array_idx()][PieceKind::Bishop.array_idx()] =
        black_pst(BISHOPS_ENDGAME, endgame_piece_value(PieceKind::Bishop));
    endgame_tables[Player::Black.array_idx()][PieceKind::Rook.array_idx()] =
        black_pst(ROOKS_ENDGAME, endgame_piece_value(PieceKind::Rook));
    endgame_tables[Player::Black.array_idx()][PieceKind::Queen.array_idx()] =
        black_pst(QUEENS_ENDGAME, endgame_piece_value(PieceKind::Queen));
    endgame_tables[Player::Black.array_idx()][PieceKind::King.array_idx()] =
        black_pst(KING_ENDGAME, endgame_piece_value(PieceKind::King));

    endgame_tables[Player::White.array_idx()][PieceKind::Pawn.array_idx()] =
        white_pst(PAWNS_ENDGAME, endgame_piece_value(PieceKind::Pawn));
    endgame_tables[Player::White.array_idx()][PieceKind::Knight.array_idx()] =
        white_pst(KNIGHTS_ENDGAME, endgame_piece_value(PieceKind::Knight));
    endgame_tables[Player::White.array_idx()][PieceKind::Bishop.array_idx()] =
        white_pst(BISHOPS_ENDGAME, endgame_piece_value(PieceKind::Bishop));
    endgame_tables[Player::White.array_idx()][PieceKind::Rook.array_idx()] =
        white_pst(ROOKS_ENDGAME, endgame_piece_value(PieceKind::Rook));
    endgame_tables[Player::White.array_idx()][PieceKind::Queen.array_idx()] =
        white_pst(QUEENS_ENDGAME, endgame_piece_value(PieceKind::Queen));
    endgame_tables[Player::White.array_idx()][PieceKind::King.array_idx()] =
        white_pst(KING_ENDGAME, endgame_piece_value(PieceKind::King));

    unsafe {
        for player in 0..Player::N {
            for piece in 0..PieceKind::N {
                for square in 0..Square::N {
                    TABLES[player][piece][square] = PhasedEval::new(
                        midgame_tables[player][piece][square],
                        endgame_tables[player][piece][square],
                    );
                }
            }
        }
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
