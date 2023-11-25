use chess::piece::Piece;
use chess::player::Player;
use chess::square::Square;
use chess::{
    game::Game,
    piece::PieceKind,
    square::{File, Rank},
};

use super::Eval;

type PieceValueTableDefinition = [[i16; File::N]; Rank::N];
type PieceValueTable = [i16; Square::N];
type PieceValueTables = [[PieceValueTable; PieceKind::N]; Player::N];

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
        [ -20, -10, -10,  -5,  -5, -10, -10, -20],
        [ -10,   0,   0,   0,   0,   0,   0, -10],
        [ -10,   0,   5,   5,   5,   5,   0, -10],
        [  -5,   0,   5,   5,   5,   5,   0,  -5],
        [   0,   0,   5,   5,   5,   5,   0,  -5],
        [ -10,   5,   5,   5,   5,   5,   0, -10],
        [ -10,   0,   5,   0,   0,   0,   0, -10],
        [ -20, -10, -10,  -5,  -5, -10, -10, -20],
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
    pub static mut TABLES: PieceValueTables = [[[0; Square::N]; PieceKind::N]; Player::N];
    
    pub fn negate(t: PieceValueTable) -> PieceValueTable {
        let mut new_table: PieceValueTable = [0; Square::N];

        for i in 0..Square::N {
            new_table[i] = -t[i];
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

    unsafe {
        TABLES[Player::Black.array_idx()][PieceKind::Pawn.array_idx()] = negate(flatten(PAWNS));
        TABLES[Player::Black.array_idx()][PieceKind::Knight.array_idx()] = negate(flatten(KNIGHTS));
        TABLES[Player::Black.array_idx()][PieceKind::Bishop.array_idx()] = negate(flatten(BISHOPS));
        TABLES[Player::Black.array_idx()][PieceKind::Rook.array_idx()] = negate(flatten(ROOKS));
        TABLES[Player::Black.array_idx()][PieceKind::Queen.array_idx()] = negate(flatten(QUEENS));
        TABLES[Player::Black.array_idx()][PieceKind::King.array_idx()] = negate(flatten(KINGS));

        TABLES[Player::White.array_idx()][PieceKind::Pawn.array_idx()] = flatten(flip(PAWNS));
        TABLES[Player::White.array_idx()][PieceKind::Knight.array_idx()] = flatten(flip(KNIGHTS));
        TABLES[Player::White.array_idx()][PieceKind::Bishop.array_idx()] = flatten(flip(BISHOPS));
        TABLES[Player::White.array_idx()][PieceKind::Rook.array_idx()] = flatten(flip(ROOKS));
        TABLES[Player::White.array_idx()][PieceKind::Queen.array_idx()] = flatten(flip(QUEENS));
        TABLES[Player::White.array_idx()][PieceKind::King.array_idx()] = flatten(flip(KINGS));
    }
}

pub fn piece_square_tables(game: &Game) -> Eval {
    let mut eval = Eval(0);

    for idx in 0..Square::N {
        let maybe_piece = game.board.pieces[idx];

        if let Some(piece) = maybe_piece {
            eval += piece_contribution(Square::from_array_index(idx), piece);
        }
    }

    eval
}

pub fn piece_square_tables_white(game: &Game) -> Eval {
    let mut eval = Eval(0);

    for (idx, maybe_piece) in game.board.pieces.iter().enumerate() {
        if let Some(piece) = maybe_piece {
            if piece.player == Player::White {
                eval += piece_contribution(Square::from_array_index(idx), *piece);
            }
        }
    }

    eval
}

pub fn piece_square_tables_black(game: &Game) -> Eval {
    let mut eval = Eval(0);

    for (idx, maybe_piece) in game.board.pieces.iter().enumerate() {
        if let Some(piece) = maybe_piece {
            if piece.player == Player::Black {
                eval += piece_contribution(Square::from_array_index(idx), *piece);
            }
        }
    }

    eval
}

#[inline]
pub fn piece_contribution(square: Square, piece: Piece) -> Eval {
    // Safe as idx is guaranteed to be in bounds - we have length 64 arrays and are
    // generating idx from Square
    Eval(unsafe {
        tables::TABLES[piece.player.array_idx()][piece.kind.array_idx()][square.array_idx()]
    })
}
