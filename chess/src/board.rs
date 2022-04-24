use crate::{
    bitboard::{self, Bitboard},
    piece::{self, Piece, PieceKind},
    player::Player,
    r#move::Move,
    square::Square,
};

pub struct Board {
    white_pieces: PlayerPieces,
    black_pieces: PlayerPieces,
}

// Many engines store these in an array (or 2D array) by piece & player.
// This avoids this approach for the initial implementation for simplicity.
#[derive(Debug)]
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

    fn player_pieces(&self, player: &Player) -> &PlayerPieces {
        match player {
            Player::White => &self.white_pieces,
            Player::Black => &self.black_pieces,
        }
    }

    pub fn player_piece_at(&self, player: &Player, square: &Square) -> Option<PieceKind> {
        let player_pieces = self.player_pieces(player);

        if player_pieces.pawns.has_square(square) {
            Some(PieceKind::Pawn)
        } else if player_pieces.knights.has_square(square) {
            Some(PieceKind::Knight)
        } else if player_pieces.bishops.has_square(square) {
            Some(PieceKind::Bishop)
        } else if player_pieces.rooks.has_square(square) {
            Some(PieceKind::Rook)
        } else if player_pieces.queen.has_square(square) {
            Some(PieceKind::Queen)
        } else if player_pieces.king.has_square(square) {
            Some(PieceKind::King)
        } else {
            None
        }
    }

    pub fn piece_at(&self, square: &Square) -> Option<Piece> {
        if let Some(white_piece_kind) = self.player_piece_at(&Player::White, square) {
            return Some(Piece::white(white_piece_kind));
        }

        if let Some(black_piece_kind) = self.player_piece_at(&Player::Black, square) {
            return Some(Piece::black(black_piece_kind));
        }

        None
    }

    // Does not consider move legality. Just concerned with the implementation details
    // of removing a piece from one square and putting it on another one (and dealing with
    // any interesting consequences (e.g. en-passant/castling))
    //
    // PERF: There's likely many more efficient ways to do this.
    // A good target for optimisation once things are working.
    //
    // TODO: Return info about the move (was it a capture?)
    #[allow(clippy::result_unit_err)]
    pub fn make_move(&self, r#move: &Move) -> Result<(Board, ()), ()> {
        let piece_to_move = self.piece_at(&r#move.src).ok_or(())?;

        let remove_src_mask = Bitboard::except_square(&r#move.src);
        let remove_from_dst_mask = Bitboard::except_square(&r#move.dst);

        let add_piece_to_dst_mask = |piece: &Piece| {
            if *piece == piece_to_move {
                Bitboard::from_square(&r#move.dst)
            } else {
                Bitboard::empty()
            }
        };

        let mask_bitboard = |bitboard: Bitboard, piece: &Piece| {
            let mut new_bitboard = bitboard
                // Remove the piece that is being moved from its bitboard
                // Currently unconditional as it's the same as removing from every bitboard
                & remove_src_mask
                // Remove any piece currently occupying the destination square
                & remove_from_dst_mask
                // Add the piece that is being moved to the destination square
                | add_piece_to_dst_mask(piece);

            if let Some(promoted_to) = r#move.promotion {
                // The promoted pawn has turned into another piece
                let remove_promoted_pawn_mask = Bitboard::except_square(&r#move.dst);

                let add_promoted_piece_mask =
                    if *piece == Piece::new(piece_to_move.player, promoted_to.piece()) {
                        Bitboard::from_square(&r#move.dst)
                    } else {
                        Bitboard::empty()
                    };

                // Place that piece on the board
                new_bitboard = new_bitboard & remove_promoted_pawn_mask | add_promoted_piece_mask;
            }

            new_bitboard
        };

        let new_board = Board {
            white_pieces: PlayerPieces {
                pawns: mask_bitboard(self.white_pieces.pawns, &piece::known::WHITE_PAWN),
                knights: mask_bitboard(self.white_pieces.knights, &piece::known::WHITE_KNIGHT),
                bishops: mask_bitboard(self.white_pieces.bishops, &piece::known::WHITE_BISHOP),
                rooks: mask_bitboard(self.white_pieces.rooks, &piece::known::WHITE_ROOK),
                queen: mask_bitboard(self.white_pieces.queen, &piece::known::WHITE_QUEEN),
                king: mask_bitboard(self.white_pieces.king, &piece::known::WHITE_KING),
            },
            black_pieces: PlayerPieces {
                pawns: mask_bitboard(self.black_pieces.pawns, &piece::known::BLACK_PAWN),
                knights: mask_bitboard(self.black_pieces.knights, &piece::known::BLACK_KNIGHT),
                bishops: mask_bitboard(self.black_pieces.bishops, &piece::known::BLACK_BISHOP),
                rooks: mask_bitboard(self.black_pieces.rooks, &piece::known::BLACK_ROOK),
                queen: mask_bitboard(self.black_pieces.queen, &piece::known::BLACK_QUEEN),
                king: mask_bitboard(self.black_pieces.king, &piece::known::BLACK_KING),
            },
        };

        // TODO: Castling
        // TODO: En-passant

        Ok((new_board, ()))
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
                        .map(|file| match self.piece_at(&Square::from_idxs(file, rank)) {
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
