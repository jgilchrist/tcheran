use crate::{
    bitboard::{self, Bitboard},
    square::Square,
    Piece, Player,
};

pub struct Board {
    pub player: Player,
    pub white_pieces: PlayerPieces,
    pub black_pieces: PlayerPieces,
}

// Many engines store these in an array (or 2D array) by piece & player.
// This avoids this approach for the initial implementation for simplicity.
pub struct PlayerPieces {
    pub pawns: Bitboard,
    pub knights: Bitboard,
    pub bishops: Bitboard,
    pub rooks: Bitboard,
    pub queen: Bitboard,
    pub king: Bitboard,
}

impl Board {
    pub fn start() -> Board {
        Board {
            player: Player::White,
            white_pieces: PlayerPieces {
                pawns: bitboard::known::INIT_WHITE_PAWNS,
                knights: bitboard::known::INIT_WHITE_KNIGHTS,
                bishops: bitboard::known::INIT_WHITE_BISHOPS,
                rooks: bitboard::known::INIT_WHITE_ROOKS,
                queen: bitboard::known::INIT_WHITE_QUEEN,
                king: bitboard::known::INIT_WHITE_KING,
            },
            black_pieces: PlayerPieces {
                pawns: bitboard::known::INIT_BLACK_PAWNS,
                knights: bitboard::known::INIT_BLACK_KNIGHTS,
                bishops: bitboard::known::INIT_BLACK_BISHOPS,
                rooks: bitboard::known::INIT_BLACK_ROOKS,
                queen: bitboard::known::INIT_BLACK_QUEEN,
                king: bitboard::known::INIT_BLACK_KING,
            },
        }
    }

    fn current_player_pieces(&self) -> &PlayerPieces {
        self.player_pieces(&self.player)
    }

    fn player_pieces(&self, player: &Player) -> &PlayerPieces {
        match player {
            Player::White => &self.white_pieces,
            Player::Black => &self.black_pieces,
        }
    }

    fn player_piece_at(&self, player: &Player, square: &Square) -> Option<Piece> {
        let player_pieces = self.player_pieces(player);

        if player_pieces.pawns.has_square(square) {
            Some(Piece::Pawn)
        } else if player_pieces.knights.has_square(square) {
            Some(Piece::Knight)
        } else if player_pieces.bishops.has_square(square) {
            Some(Piece::Bishop)
        } else if player_pieces.rooks.has_square(square) {
            Some(Piece::Rook)
        } else if player_pieces.queen.has_square(square) {
            Some(Piece::Queen)
        } else if player_pieces.king.has_square(square) {
            Some(Piece::King)
        } else {
            None
        }
    }

    fn piece_at(&self, square: &Square) -> Option<(Player, Piece)> {
        if let Some(white_piece) = self.player_piece_at(&Player::White, square) {
            return Some((Player::White, white_piece));
        }

        if let Some(black_piece) = self.player_piece_at(&Player::Black, square) {
            return Some((Player::Black, black_piece));
        }

        None
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{}\n",
            (0..8)
                .rev()
                .into_iter()
                .map(|rank| {
                    (0..8)
                        .into_iter()
                        .map(|file| match self.piece_at(&Square::from_idx(file, rank)) {
                            Some((player, piece)) => match piece {
                                Piece::Pawn => match player {
                                    Player::White => "♟",
                                    Player::Black => "♙",
                                },
                                Piece::Knight => match player {
                                    Player::White => "♞",
                                    Player::Black => "♘",
                                },
                                Piece::Bishop => match player {
                                    Player::White => "♝",
                                    Player::Black => "♗",
                                },
                                Piece::Rook => match player {
                                    Player::White => "♜",
                                    Player::Black => "♖",
                                },
                                Piece::Queen => match player {
                                    Player::White => "♛",
                                    Player::Black => "♕",
                                },
                                Piece::King => match player {
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
