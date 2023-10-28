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
    pub pieces: [Option<Piece>; Squares::N],
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
    pub fn start() -> Self {
        let mut start = Self {
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
            pieces: [None; Squares::N],
        };

        // TODO: Use a constant
        for pawn in start.white_pieces.pawns {
            start.pieces[pawn.array_idx()] = Some(Piece::new(Player::White, PieceKind::Pawn));
        }

        for knight in start.white_pieces.knights {
            start.pieces[knight.array_idx()] = Some(Piece::new(Player::White, PieceKind::Knight));
        }

        for bishop in start.white_pieces.bishops {
            start.pieces[bishop.array_idx()] = Some(Piece::new(Player::White, PieceKind::Bishop));
        }

        for rook in start.white_pieces.rooks {
            start.pieces[rook.array_idx()] = Some(Piece::new(Player::White, PieceKind::Rook));
        }

        for queen in start.white_pieces.queens {
            start.pieces[queen.array_idx()] = Some(Piece::new(Player::White, PieceKind::Queen));
        }

        for king in start.white_pieces.king {
            start.pieces[king.array_idx()] = Some(Piece::new(Player::White, PieceKind::King));
        }

        for pawn in start.black_pieces.pawns {
            start.pieces[pawn.array_idx()] = Some(Piece::new(Player::Black, PieceKind::Pawn));
        }

        for knight in start.black_pieces.knights {
            start.pieces[knight.array_idx()] = Some(Piece::new(Player::Black, PieceKind::Knight));
        }

        for bishop in start.black_pieces.bishops {
            start.pieces[bishop.array_idx()] = Some(Piece::new(Player::Black, PieceKind::Bishop));
        }

        for rook in start.black_pieces.rooks {
            start.pieces[rook.array_idx()] = Some(Piece::new(Player::Black, PieceKind::Rook));
        }

        for queen in start.black_pieces.queens {
            start.pieces[queen.array_idx()] = Some(Piece::new(Player::Black, PieceKind::Queen));
        }

        for king in start.black_pieces.king {
            start.pieces[king.array_idx()] = Some(Piece::new(Player::Black, PieceKind::King));
        }

        start
    }

    #[must_use]
    pub const fn player_pieces(&self, player: Player) -> &PlayerPieces {
        match player {
            Player::White => &self.white_pieces,
            Player::Black => &self.black_pieces,
        }
    }

    #[must_use]
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        // We know array_idx can only return up to Squares::N - 1
        unsafe { *self.pieces.get_unchecked(square.array_idx()) }
    }

    #[inline]
    fn player_pieces_for(&mut self, player: Player) -> &mut PlayerPieces {
        match player {
            Player::White => &mut self.white_pieces,
            Player::Black => &mut self.black_pieces,
        }
    }

    #[inline]
    fn squares_for_piece(&mut self, piece: Piece) -> &mut Squares {
        let player_pieces = self.player_pieces_for(piece.player);

        match piece.kind {
            PieceKind::Pawn => &mut player_pieces.pawns,
            PieceKind::Knight => &mut player_pieces.knights,
            PieceKind::Bishop => &mut player_pieces.bishops,
            PieceKind::Rook => &mut player_pieces.rooks,
            PieceKind::Queen => &mut player_pieces.queens,
            PieceKind::King => &mut player_pieces.king,
        }
    }

    #[inline]
    pub fn remove_at(&mut self, square: Square) -> bool {
        let Some(piece) = self.piece_at(square) else {
            return false;
        };

        self.squares_for_piece(piece).unset_inplace(square);
        self.pieces[square.array_idx()] = None;
        true
    }

    #[inline]
    pub fn set_at(&mut self, square: Square, piece: Piece) {
        self.squares_for_piece(piece).set_inplace(square);
        self.pieces[square.array_idx()] = Some(piece);
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
            pieces,
        })
    }
}
