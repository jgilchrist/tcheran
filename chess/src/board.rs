use crate::{
    movegen,
    piece::{Piece, PieceKind},
    player::Player,
    square::Square,
    squares::{self, Squares},
};

use anyhow::Result;

#[derive(Clone, Copy)]
pub struct Board {
    pub white_pieces: PlayerPieces,
    pub black_pieces: PlayerPieces,
}

// Many engines store these in an array (or 2D array) by piece & player.
// This avoids this approach for the initial implementation for simplicity.
#[derive(Clone, Copy)]
pub struct PlayerPieces {
    pub pawns: Squares,
    pub knights: Squares,
    pub bishops: Squares,
    pub rooks: Squares,
    pub queens: Squares,
    pub king: Squares,
}

impl PlayerPieces {
    pub(crate) fn all(&self) -> Squares {
        self.pawns | self.knights | self.bishops | self.rooks | self.queens | self.king
    }
}

impl Board {
    #[must_use]
    pub const fn start() -> Self {
        Self {
            white_pieces: PlayerPieces {
                pawns: squares::INIT_WHITE_PAWNS,
                knights: squares::INIT_WHITE_KNIGHTS,
                bishops: squares::INIT_WHITE_BISHOPS,
                rooks: squares::INIT_WHITE_ROOKS,
                queens: Squares::from_square(squares::INIT_WHITE_QUEEN),
                king: Squares::from_square(squares::INIT_WHITE_KING),
            },
            black_pieces: PlayerPieces {
                pawns: squares::INIT_BLACK_PAWNS,
                knights: squares::INIT_BLACK_KNIGHTS,
                bishops: squares::INIT_BLACK_BISHOPS,
                rooks: squares::INIT_BLACK_ROOKS,
                queens: Squares::from_square(squares::INIT_BLACK_QUEEN),
                king: Squares::from_square(squares::INIT_BLACK_KING),
            },
        }
    }

    #[must_use]
    pub const fn player_pieces(&self, player: Player) -> &PlayerPieces {
        match player {
            Player::White => &self.white_pieces,
            Player::Black => &self.black_pieces,
        }
    }

    #[must_use]
    pub fn player_piece_at(&self, player: Player, square: Square) -> Option<PieceKind> {
        let player_pieces = self.player_pieces(player);

        if player_pieces.pawns.contains(square) {
            Some(PieceKind::Pawn)
        } else if player_pieces.knights.contains(square) {
            Some(PieceKind::Knight)
        } else if player_pieces.bishops.contains(square) {
            Some(PieceKind::Bishop)
        } else if player_pieces.rooks.contains(square) {
            Some(PieceKind::Rook)
        } else if player_pieces.queens.contains(square) {
            Some(PieceKind::Queen)
        } else if player_pieces.king.contains(square) {
            Some(PieceKind::King)
        } else {
            None
        }
    }

    #[must_use]
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        if let Some(white_piece_kind) = self.player_piece_at(Player::White, square) {
            return Some(Piece::white(white_piece_kind));
        }

        if let Some(black_piece_kind) = self.player_piece_at(Player::Black, square) {
            return Some(Piece::black(black_piece_kind));
        }

        None
    }

    #[must_use]
    pub fn king_in_check(&self, player: Player) -> bool {
        let enemy_attacks = movegen::generate_all_attacks(self, player.other());

        let king = self.player_pieces(player).king.single();

        enemy_attacks.contains(king)
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

impl TryFrom<[Option<Piece>; Squares::N]> for Board {
    type Error = anyhow::Error;

    fn try_from(pieces: [Option<Piece>; Squares::N]) -> Result<Self> {
        let mut white_pawns = Squares::none();
        let mut white_knights = Squares::none();
        let mut white_bishops = Squares::none();
        let mut white_rooks = Squares::none();
        let mut white_queens = Squares::none();
        let mut white_king = Squares::none();

        let mut black_pawns = Squares::none();
        let mut black_knights = Squares::none();
        let mut black_bishops = Squares::none();
        let mut black_rooks = Squares::none();
        let mut black_queens = Squares::none();
        let mut black_king = Squares::none();

        for (i, maybe_piece) in pieces.iter().enumerate() {
            if let Some(p) = maybe_piece {
                let square = Square::from_index(i.try_into()?);

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
            white_pieces: PlayerPieces {
                pawns: white_pawns,
                knights: white_knights,
                bishops: white_bishops,
                rooks: white_rooks,
                queens: white_queens,
                king: white_king,
            },
            black_pieces: PlayerPieces {
                pawns: black_pawns,
                knights: black_knights,
                bishops: black_bishops,
                rooks: black_rooks,
                queens: black_queens,
                king: black_king,
            },
        })
    }
}
