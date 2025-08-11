use super::PhasedEval;
use crate::chess::board::Board;
use crate::chess::piece::Piece;
use crate::chess::player::{ByPlayer, Player};
use crate::chess::square::Square;
use crate::chess::{
    piece::PieceKind,
    square::{File, Rank},
};
use crate::engine::eval::params::{
    BISHOPS, KING, KNIGHTS, PAWNS, PIECE_VALUES, PieceSquareTableDefinition, QUEENS, ROOKS,
};

pub type PieceSquareTable = [PhasedEval; Square::N];
type PieceValueTables = [[PieceSquareTable; PieceKind::N]; Player::N];

// These need to be initialised when we start up, since they can
// be derived from the white tables.
pub static mut TABLES: PieceValueTables =
    [[[PhasedEval::ZERO; Square::N]; PieceKind::N]; Player::N];

pub fn negate(t: PieceSquareTable) -> PieceSquareTable {
    let mut new_table: PieceSquareTable = [PhasedEval::ZERO; Square::N];

    for i in 0..Square::N {
        new_table[i] = -t[i];
    }

    new_table
}

pub fn add_material(t: PieceSquareTable, p: PieceKind) -> PieceSquareTable {
    let mut new_table: PieceSquareTable = [PhasedEval::ZERO; Square::N];
    let material_value = PIECE_VALUES[p.array_idx()];

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

#[rustfmt::skip]
pub fn init() {
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
    unsafe { TABLES[piece.player.array_idx()][piece.kind.array_idx()][square.array_idx()] }
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
