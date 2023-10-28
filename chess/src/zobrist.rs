use crate::game::{CastleRightsSide, Game};
use crate::piece::{Piece, PieceKind};
use crate::player::Player;
use crate::square::Square;
use crate::squares::Squares;
use rand::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ZobristHash(pub u64);

impl ZobristHash {
    #[must_use]
    pub fn uninit() -> Self {
        Self(0)
    }

    pub fn toggle_piece_on_square(&mut self, square: Square, piece: Piece) {
        self.0 ^= piece_on_square(piece.player, piece.kind, square);
    }

    pub fn toggle_castle_rights(&mut self, player: Player, side: CastleRightsSide) {
        self.0 ^= castle_rights(player, side);
    }

    pub fn set_en_passant(&mut self, previous_square: Option<Square>, square: Option<Square>) {
        self.0 ^= en_passant(previous_square);
        self.0 ^= en_passant(square);
    }

    pub fn toggle_side_to_play(&mut self) {
        self.0 ^= side_to_play();
    }
}

type ZobristComponent = u64;

mod components {
    use super::*;
    use crate::piece::PieceKind;
    use crate::player::Player;
    use crate::squares::Squares;

    pub static mut PIECE_SQUARE: [[[ZobristComponent; PieceKind::N]; Squares::N]; Player::N] =
        [[[0; PieceKind::N]; Squares::N]; Player::N];

    pub static mut CASTLING: [[ZobristComponent; CastleRightsSide::N]; Player::N] =
        [[0; CastleRightsSide::N]; Player::N];

    pub static mut EN_PASSANT_SQUARE: [ZobristComponent; Squares::N] = [0; Squares::N];

    pub static mut NO_EN_PASSANT_SQUARE: ZobristComponent = 0;

    pub static mut SIDE_TO_PLAY: ZobristComponent = 0;
}

pub fn init() {
    // TODO: Figure out exactly which generator to use
    // TODO: Seed this with a fixed seed
    let mut random = thread_rng();

    for player in 0..Player::N {
        for square in 0..Squares::N {
            for piece in 0..PieceKind::N {
                unsafe {
                    components::PIECE_SQUARE[player][square][piece] = random.next_u64();
                }
            }
        }

        for castle_rights in 0..CastleRightsSide::N {
            unsafe {
                components::CASTLING[player][castle_rights] = random.next_u64();
            }
        }
    }

    for square in 0..Squares::N {
        unsafe {
            components::EN_PASSANT_SQUARE[square] = random.next_u64();
        }
    }

    unsafe {
        components::NO_EN_PASSANT_SQUARE = random.next_u64();
    }

    unsafe {
        components::SIDE_TO_PLAY = random.next_u64();
    }
}

#[must_use]
pub fn hash(game: &Game) -> ZobristHash {
    let mut hash = 0u64;

    // Add piece components to hash
    // White
    for s in game.board.white_pieces.pawns {
        hash ^= piece_on_square(Player::White, PieceKind::Pawn, s);
    }

    for s in game.board.white_pieces.knights {
        hash ^= piece_on_square(Player::White, PieceKind::Knight, s);
    }

    for s in game.board.white_pieces.bishops {
        hash ^= piece_on_square(Player::White, PieceKind::Bishop, s);
    }

    for s in game.board.white_pieces.rooks {
        hash ^= piece_on_square(Player::White, PieceKind::Rook, s);
    }

    for s in game.board.white_pieces.queens {
        hash ^= piece_on_square(Player::White, PieceKind::Queen, s);
    }

    for s in game.board.white_pieces.king {
        hash ^= piece_on_square(Player::White, PieceKind::King, s);
    }

    // Black
    for s in game.board.black_pieces.pawns {
        hash ^= piece_on_square(Player::Black, PieceKind::Pawn, s);
    }

    for s in game.board.black_pieces.knights {
        hash ^= piece_on_square(Player::Black, PieceKind::Knight, s);
    }

    for s in game.board.black_pieces.bishops {
        hash ^= piece_on_square(Player::Black, PieceKind::Bishop, s);
    }

    for s in game.board.black_pieces.rooks {
        hash ^= piece_on_square(Player::Black, PieceKind::Rook, s);
    }

    for s in game.board.black_pieces.queens {
        hash ^= piece_on_square(Player::Black, PieceKind::Queen, s);
    }

    for s in game.board.black_pieces.king {
        hash ^= piece_on_square(Player::Black, PieceKind::King, s);
    }

    // Castle rights
    // White
    if game.white_castle_rights.king_side {
        hash ^= castle_rights(Player::White, CastleRightsSide::Kingside);
    }

    if game.white_castle_rights.queen_side {
        hash ^= castle_rights(Player::White, CastleRightsSide::Queenside);
    }

    // Black
    if game.black_castle_rights.king_side {
        hash ^= castle_rights(Player::Black, CastleRightsSide::Kingside);
    }

    if game.black_castle_rights.queen_side {
        hash ^= castle_rights(Player::Black, CastleRightsSide::Queenside);
    }

    // En passant
    hash ^= en_passant(game.en_passant_target);

    // Side to play
    if game.player == Player::Black {
        hash ^= side_to_play();
    }

    ZobristHash(hash)
}

fn piece_on_square(player: Player, piece: PieceKind, square: Square) -> ZobristComponent {
    unsafe { components::PIECE_SQUARE[player.array_idx()][square.array_idx()][piece.array_idx()] }
}

fn castle_rights(player: Player, side: CastleRightsSide) -> ZobristComponent {
    unsafe { components::CASTLING[player.array_idx()][side.array_idx()] }
}

fn en_passant(square: Option<Square>) -> ZobristComponent {
    match square {
        Some(s) => unsafe { components::EN_PASSANT_SQUARE[s.array_idx()] },
        None => unsafe { components::NO_EN_PASSANT_SQUARE },
    }
}

fn side_to_play() -> ZobristComponent {
    unsafe { components::SIDE_TO_PLAY }
}
