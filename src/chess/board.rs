use crate::chess::{
    movegen,
    piece::{Piece, PieceKind},
    player::Player,
    square::Square,
};

use crate::chess::bitboard::Bitboard;
use color_eyre::Result;

#[derive(Clone)]
pub struct Board {
    pub white_pieces: PlayerPieces,
    pub black_pieces: PlayerPieces,
    pub pieces: [Option<Piece>; Square::N],
}

// Many engines store these in an array (or 2D array) by piece & player.
// This avoids this approach for the initial implementation for simplicity.
#[derive(Clone)]
pub struct PlayerPieces([Bitboard; PieceKind::N]);

impl PlayerPieces {
    pub fn new(pieces: [Bitboard; PieceKind::N]) -> Self {
        Self(pieces)
    }

    #[inline(always)]
    pub(crate) fn all(&self) -> Bitboard {
        self.pawns() | self.knights() | self.bishops() | self.rooks() | self.queens() | self.king()
    }

    #[inline(always)]
    pub fn pawns(&self) -> Bitboard {
        self.0[PieceKind::Pawn.array_idx()]
    }

    #[inline(always)]
    pub fn knights(&self) -> Bitboard {
        self.0[PieceKind::Knight.array_idx()]
    }

    #[inline(always)]
    pub fn bishops(&self) -> Bitboard {
        self.0[PieceKind::Bishop.array_idx()]
    }

    #[inline(always)]
    pub fn rooks(&self) -> Bitboard {
        self.0[PieceKind::Rook.array_idx()]
    }

    #[inline(always)]
    pub fn queens(&self) -> Bitboard {
        self.0[PieceKind::Queen.array_idx()]
    }

    #[inline(always)]
    pub fn king(&self) -> Bitboard {
        self.0[PieceKind::King.array_idx()]
    }
}

impl Board {
    #[inline(always)]
    pub const fn player_pieces(&self, player: Player) -> &PlayerPieces {
        match player {
            Player::White => &self.white_pieces,
            Player::Black => &self.black_pieces,
        }
    }

    #[inline(always)]
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        // We know array_idx can only return up to Square::N - 1
        unsafe { *self.pieces.get_unchecked(square.array_idx()) }
    }

    #[inline(always)]
    fn player_pieces_for(&mut self, player: Player) -> &mut PlayerPieces {
        match player {
            Player::White => &mut self.white_pieces,
            Player::Black => &mut self.black_pieces,
        }
    }

    #[inline(always)]
    fn squares_for_piece(&mut self, piece: Piece) -> &mut Bitboard {
        let player_pieces = self.player_pieces_for(piece.player);
        &mut player_pieces.0[piece.kind.array_idx()]
    }

    #[inline(always)]
    pub fn remove_at(&mut self, square: Square) -> bool {
        let Some(piece) = self.piece_at(square) else {
            return false;
        };

        self.squares_for_piece(piece).unset_inplace(square);
        self.pieces[square.array_idx()] = None;
        true
    }

    #[inline(always)]
    pub fn set_at(&mut self, square: Square, piece: Piece) {
        self.squares_for_piece(piece).set_inplace(square);
        self.pieces[square.array_idx()] = Some(piece);
    }

    pub fn king_in_check(&self, player: Player) -> bool {
        let king = self.player_pieces(player).king().single();
        let enemy_attackers = movegen::generate_attackers_of(self, player, king);
        enemy_attackers.any()
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{}\n",
            (0..8)
                .rev()
                .map(|rank| {
                    (0..8)
                        .map(|file| match self.piece_at(Square::from_idxs(file, rank)) {
                            Some(Piece { player, kind }) => match kind {
                                PieceKind::Pawn => match player {
                                    Player::White => "♟",
                                    Player::Black => "♙",
                                },
                                PieceKind::Knight => match player {
                                    Player::White => "♞",
                                    Player::Black => "♘",
                                },
                                PieceKind::Bishop => match player {
                                    Player::White => "♝",
                                    Player::Black => "♗",
                                },
                                PieceKind::Rook => match player {
                                    Player::White => "♜",
                                    Player::Black => "♖",
                                },
                                PieceKind::Queen => match player {
                                    Player::White => "♛",
                                    Player::Black => "♕",
                                },
                                PieceKind::King => match player {
                                    Player::White => "♚",
                                    Player::Black => "♔",
                                },
                            },
                            None => ".",
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl TryFrom<[Option<Piece>; Square::N]> for Board {
    type Error = color_eyre::eyre::Error;

    fn try_from(pieces: [Option<Piece>; Square::N]) -> Result<Self> {
        let mut white_pawns = Bitboard::EMPTY;
        let mut white_knights = Bitboard::EMPTY;
        let mut white_bishops = Bitboard::EMPTY;
        let mut white_rooks = Bitboard::EMPTY;
        let mut white_queens = Bitboard::EMPTY;
        let mut white_king = Bitboard::EMPTY;

        let mut black_pawns = Bitboard::EMPTY;
        let mut black_knights = Bitboard::EMPTY;
        let mut black_bishops = Bitboard::EMPTY;
        let mut black_rooks = Bitboard::EMPTY;
        let mut black_queens = Bitboard::EMPTY;
        let mut black_king = Bitboard::EMPTY;

        for (i, maybe_piece) in pieces.iter().enumerate() {
            if let Some(p) = maybe_piece {
                let square = Square::from_index(i.try_into()?).bb();

                match *p {
                    Piece::WHITE_PAWN => white_pawns |= square,
                    Piece::WHITE_KNIGHT => white_knights |= square,
                    Piece::WHITE_BISHOP => white_bishops |= square,
                    Piece::WHITE_ROOK => white_rooks |= square,
                    Piece::WHITE_QUEEN => white_queens |= square,
                    Piece::WHITE_KING => white_king |= square,

                    Piece::BLACK_PAWN => black_pawns |= square,
                    Piece::BLACK_KNIGHT => black_knights |= square,
                    Piece::BLACK_BISHOP => black_bishops |= square,
                    Piece::BLACK_ROOK => black_rooks |= square,
                    Piece::BLACK_QUEEN => black_queens |= square,
                    Piece::BLACK_KING => black_king |= square,
                }
            }
        }

        Ok(Self {
            white_pieces: PlayerPieces::new([
                white_pawns,
                white_knights,
                white_bishops,
                white_rooks,
                white_queens,
                white_king,
            ]),
            black_pieces: PlayerPieces([
                black_pawns,
                black_knights,
                black_bishops,
                black_rooks,
                black_queens,
                black_king,
            ]),
            pieces,
        })
    }
}
