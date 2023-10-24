use chess::{
    game::Game,
    piece::PieceKind,
    square::{File, Rank},
    squares::Squares,
};
use chess::piece::Piece;
use chess::player::Player;

use super::Eval;

type PieceValueTableDefinition = [[i32; File::N]; Rank::N];
type PieceValueTable = [i32; Squares::N];
type PieceValueTables = [PieceValueTable; PieceKind::N];

#[rustfmt::skip]
mod tables {
    use super::*;

    // Piece square tables taken, for now, from https://www.chessprogramming.org/Simplified_Evaluation_Function

    pub const PAWNS: PieceValueTableDefinition = [
        [   0,   0,   0,   0,   0,   0,   0,   0],
        [  50,  50,  50,  50,  50,  50,  50,  50],
        [  10,  10,  20,  30,  30,  20,  10,  10],
        [   5,   5,  10,  25,  25,  10,   5,   5],
        [   0,   0,   0,  20,  20,   0,   0,   0],
        [   5,  -5, -10,   0,   0, -10,  -5,   5],
        [   5,  10,  10, -20, -20,  10,  10,   5],
        [   0,   0,   0,   0,   0,   0,   0,   0],
    ];

    pub const KNIGHTS: PieceValueTableDefinition = [
        [ -50, -40, -30, -30, -30, -30, -40, -50],
        [ -40, -20,   0,   0,   0,   0, -20, -40],
        [ -30,   0,  10,  15,  15,  10,   0, -30],
        [ -30,   5,  15,  20,  20,  15,   5, -30],
        [ -30,   0,  15,  20,  20,  15,   0, -30],
        [ -30,   5,  10,  15,  15,  10,   5, -30],
        [ -40, -20,   0,   5,   5,   0, -20, -40],
        [ -50, -40, -30, -30, -30, -30, -40, -50],
    ];

    pub const BISHOPS: PieceValueTableDefinition = [
        [ -20, -10, -10, -10, -10, -10, -10, -20],
        [ -10,   0,   0,   0,   0,   0,   0, -10],
        [ -10,   0,   5,  10,  10,   5,   0, -10],
        [ -10,   5,   5,  10,  10,   5,   5, -10],
        [ -10,   0,  10,  10,  10,  10,   0, -10],
        [ -10,  10,  10,  10,  10,  10,  10, -10],
        [ -10,   5,   0,   0,   0,   0,   5, -10],
        [ -20, -10, -10, -10, -10, -10, -10, -20],
    ];

    pub const ROOKS: PieceValueTableDefinition = [
        [   0,   0,   0,   0,   0,   0,   0,   0],
        [   5,  10,  10,  10,  10,  10,  10,   5],
        [  -5,   0,   0,   0,   0,   0,   0,  -5],
        [  -5,   0,   0,   0,   0,   0,   0,  -5],
        [  -5,   0,   0,   0,   0,   0,   0,  -5],
        [  -5,   0,   0,   0,   0,   0,   0,  -5],
        [  -5,   0,   0,   0,   0,   0,   0,  -5],
        [   0,   0,   0,   5,   5,   0,   0,   0],
    ];

    pub const QUEENS: PieceValueTableDefinition = [
        [ -30, -40, -40, -50, -50, -40, -40, -30],
        [ -30, -40, -40, -50, -50, -40, -40, -30],
        [ -30, -40, -40, -50, -50, -40, -40, -30],
        [ -30, -40, -40, -50, -50, -40, -40, -30],
        [ -20, -30, -30, -40, -40, -30, -30, -20],
        [ -10, -20, -20, -20, -20, -20, -20, -10],
        [  20,  20,   0,   0,   0,   0,  20,  20],
        [  20,  30,  10,   0,   0,  10,  30,  20],
    ];

    pub const KINGS: PieceValueTableDefinition = [
        [ -30, -40, -40, -50, -50, -40, -40, -30],
        [ -30, -40, -40, -50, -50, -40, -40, -30],
        [ -30, -40, -40, -50, -50, -40, -40, -30],
        [ -30, -40, -40, -50, -50, -40, -40, -30],
        [ -20, -30, -30, -40, -40, -30, -30, -20],
        [ -10, -20, -20, -20, -20, -20, -20, -10],
        [  20,  20,   0,   0,   0,   0,  20,  20],
        [  20,  30,  10,   0,   0,  10,  30,  20],
    ];

    // These need to be initialised when we start up, since they can
    // be derived from the white tables.
    pub static mut WHITE_TABLES: PieceValueTables = [[0; Squares::N]; PieceKind::N];
    pub static mut BLACK_TABLES: PieceValueTables = [[0; Squares::N]; PieceKind::N];

    pub fn flip(t: PieceValueTableDefinition) -> PieceValueTableDefinition {
        let mut new_table: PieceValueTableDefinition = Default::default();

        for i in 0..Rank::N {
            new_table[i] = t[Rank::N - i - 1];
        }

        new_table
    }

    pub fn flatten(definition: PieceValueTableDefinition) -> PieceValueTable {
        let mut new_table: PieceValueTable = [0; Squares::N];

        for i in 0..Squares::N {
            new_table[i] = definition[i / File::N][i % Rank::N];
        }

        new_table
    }
}

pub fn init() {
    use tables::*;

    unsafe {
        BLACK_TABLES[PieceKind::Pawn.array_idx()] = flatten(PAWNS);
        BLACK_TABLES[PieceKind::Knight.array_idx()] = flatten(KNIGHTS);
        BLACK_TABLES[PieceKind::Bishop.array_idx()] = flatten(BISHOPS);
        BLACK_TABLES[PieceKind::Rook.array_idx()] = flatten(ROOKS);
        BLACK_TABLES[PieceKind::Queen.array_idx()] = flatten(QUEENS);
        BLACK_TABLES[PieceKind::King.array_idx()] = flatten(KINGS);

        WHITE_TABLES[PieceKind::Pawn.array_idx()] = flatten(flip(PAWNS));
        WHITE_TABLES[PieceKind::Knight.array_idx()] = flatten(flip(KNIGHTS));
        WHITE_TABLES[PieceKind::Bishop.array_idx()] = flatten(flip(BISHOPS));
        WHITE_TABLES[PieceKind::Rook.array_idx()] = flatten(flip(ROOKS));
        WHITE_TABLES[PieceKind::Queen.array_idx()] = flatten(flip(QUEENS));
        WHITE_TABLES[PieceKind::King.array_idx()] = flatten(flip(KINGS));
    }
}

pub fn piece_square_tables(game: &Game) -> Eval {
    let mut eval = 0;

    for idx in 0..Squares::N {
        let maybe_piece = game.board.pieces[idx];

        if let Some(piece) = maybe_piece {
            eval += piece_contribution(idx, &piece);
        }
    }

    Eval(eval)
}

pub(crate) fn piece_square_tables_white(game: &Game) -> Eval {
    let mut eval = 0;

    for (idx, maybe_piece) in game.board.pieces.iter().enumerate() {
        if let Some(piece) = maybe_piece {
            if piece.player == Player::White {
                eval += piece_contribution(idx, piece);
            }
        }
    }

    Eval(eval)
}

pub(crate) fn piece_square_tables_black(game: &Game) -> Eval {
    let mut eval = 0;

    for (idx, maybe_piece) in game.board.pieces.iter().enumerate() {
        if let Some(piece) = maybe_piece {
            if piece.player == Player::Black {
                eval += piece_contribution(idx, piece);
            }
        }
    }

    Eval(eval)
}

#[inline]
fn piece_contribution(idx: usize, piece: &Piece) -> i32 {
    // Safe as idx is guaranteed to be in bounds - we have length 64 arrays and are
    // generating idx from Square
    unsafe {
        match piece.player {
            Player::White => tables::WHITE_TABLES[piece.kind.array_idx()][idx],
            Player::Black => -tables::BLACK_TABLES[piece.kind.array_idx()][idx],
        }
    }
}