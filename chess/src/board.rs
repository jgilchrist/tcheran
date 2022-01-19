use crate::{
    bitboard::{self, Bitboard},
    square::Square,
    Piece, Player,
};

pub struct Board {
    player: Player,
    white_pieces: PlayerPieces,
    black_pieces: PlayerPieces,
}

// Many engines store these in an array (or 2D array) by piece & player.
// This avoids this approach for the initial implementation for simplicity.
struct PlayerPieces {
    pawns: Bitboard,
    knights: Bitboard,
    bishops: Bitboard,
    rooks: Bitboard,
    queen: Bitboard,
    king: Bitboard,
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
                        .map(|file| {
                            match self.piece_at(&Square::from_idx(file, rank)) {
                                Some((player, piece)) => {
                                    let piece_char = match piece {
                                        Piece::Pawn => "p",
                                        Piece::Knight => "n",
                                        Piece::Bishop => "b",
                                        Piece::Rook => "r",
                                        Piece::Queen => "q",
                                        Piece::King => "k",
                                    };

                                    match player {
                                        Player::White => piece_char.to_string(),
                                        Player::Black => piece_char.to_uppercase(),
                                    }
                                },
                                None => ".".to_string()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
