use crate::{
    attacks::{self, generate_all_attacks},
    bitboard::{self, Bitboard},
    direction::Direction,
    moves::Move,
    piece::{Piece, PieceKind},
    player::Player,
    square::{
        squares::{self, *},
        Square,
    },
};

#[derive(Clone, Copy)]
pub struct Board {
    pub white_pieces: PlayerPieces,
    pub black_pieces: PlayerPieces,
}

// Many engines store these in an array (or 2D array) by piece & player.
// This avoids this approach for the initial implementation for simplicity.
#[derive(Clone, Copy)]
pub struct PlayerPieces {
    pub pawns: Bitboard,
    pub knights: Bitboard,
    pub bishops: Bitboard,
    pub rooks: Bitboard,
    pub queen: Bitboard,
    pub king: Bitboard,
}

impl PlayerPieces {
    pub(crate) fn all(&self) -> Bitboard {
        self.pawns | self.knights | self.bishops | self.rooks | self.queen | self.king
    }
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

    pub fn from_array(pieces: [Option<Piece>; 64]) -> Board {
        let mut white_pawns_bitboard = Bitboard::empty();
        let mut white_knights_bitboard = Bitboard::empty();
        let mut white_bishops_bitboard = Bitboard::empty();
        let mut white_rooks_bitboard = Bitboard::empty();
        let mut white_queen_bitboard = Bitboard::empty();
        let mut white_king_bitboard = Bitboard::empty();

        let mut black_pawns_bitboard = Bitboard::empty();
        let mut black_knights_bitboard = Bitboard::empty();
        let mut black_bishops_bitboard = Bitboard::empty();
        let mut black_rooks_bitboard = Bitboard::empty();
        let mut black_queen_bitboard = Bitboard::empty();
        let mut black_king_bitboard = Bitboard::empty();

        for (i, maybe_piece) in pieces.iter().enumerate() {
            if let Some(p) = maybe_piece {
                let bitboard = Bitboard::new(1 << i);

                match *p {
                    Piece::WHITE_PAWN => white_pawns_bitboard |= bitboard,
                    Piece::WHITE_KNIGHT => white_knights_bitboard |= bitboard,
                    Piece::WHITE_BISHOP => white_bishops_bitboard |= bitboard,
                    Piece::WHITE_ROOK => white_rooks_bitboard |= bitboard,
                    Piece::WHITE_QUEEN => white_queen_bitboard |= bitboard,
                    Piece::WHITE_KING => white_king_bitboard |= bitboard,

                    Piece::BLACK_PAWN => black_pawns_bitboard |= bitboard,
                    Piece::BLACK_KNIGHT => black_knights_bitboard |= bitboard,
                    Piece::BLACK_BISHOP => black_bishops_bitboard |= bitboard,
                    Piece::BLACK_ROOK => black_rooks_bitboard |= bitboard,
                    Piece::BLACK_QUEEN => black_queen_bitboard |= bitboard,
                    Piece::BLACK_KING => black_king_bitboard |= bitboard,
                }
            }
        }

        Board {
            white_pieces: PlayerPieces {
                pawns: white_pawns_bitboard,
                knights: white_knights_bitboard,
                bishops: white_bishops_bitboard,
                rooks: white_rooks_bitboard,
                queen: white_queen_bitboard,
                king: white_king_bitboard,
            },
            black_pieces: PlayerPieces {
                pawns: black_pawns_bitboard,
                knights: black_knights_bitboard,
                bishops: black_bishops_bitboard,
                rooks: black_rooks_bitboard,
                queen: black_queen_bitboard,
                king: black_king_bitboard,
            },
        }
    }

    pub fn player_pieces(&self, player: &Player) -> &PlayerPieces {
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

    pub fn king_in_check(&self, player: &Player) -> bool {
        let enemy_attacks = generate_all_attacks(self, &player.other());

        let king = self.player_pieces(player).king.to_square_definite();

        enemy_attacks.has_square(&king)
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
    pub fn make_move(&self, mv: &Move) -> Result<(Board, ()), ()> {
        let moved_piece = self.piece_at(&mv.src).ok_or(())?;

        let remove_src_mask = Bitboard::except_square(&mv.src);
        let remove_from_dst_mask = Bitboard::except_square(&mv.dst);

        let add_piece_to_dst_mask = |piece: &Piece| {
            if *piece == moved_piece {
                mv.dst.bitboard()
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

            if let Some(promoted_to) = mv.promotion {
                // The promoted pawn has turned into another piece
                let remove_promoted_pawn_mask = Bitboard::except_square(&mv.dst);

                let add_promoted_piece_mask =
                    if *piece == Piece::new(moved_piece.player, promoted_to.piece()) {
                        mv.dst.bitboard()
                    } else {
                        Bitboard::empty()
                    };

                // Place that piece on the board
                new_bitboard &= remove_promoted_pawn_mask;
                new_bitboard |= add_promoted_piece_mask;
            }

            // PERF: Here, we figure out if the move was en-passant. It may be more performant to
            // tell this function that the move was en-passant, but it loses the cleanliness of
            // just telling the board the start and end destination for the piece.
            //
            // PERF: We only need to check mv.is_diagonal() if we moved from the rank where
            // en-passant can happen which is likely a much cheaper check (just bitwise and on the
            // bitboard and square).
            //
            // If we just moved a pawn diagonally, we need to double check whether it was en-passant,
            // in which case we need to remove the captured pawn.
            if moved_piece.kind == PieceKind::Pawn {
                let pawn_attacks = attacks::generate_pawn_attacks(&mv.src, &moved_piece.player);

                if pawn_attacks.has_square(&mv.dst) {
                    let opponent_pieces = self.player_pieces(&moved_piece.player.other()).all();

                    // Definitely en-passant, as we made a capture but there was no piece on that square.
                    if !opponent_pieces.has_square(&mv.dst) {
                        // Get the square that we need to remove the pawn from.
                        let inverse_pawn_move_direction = match moved_piece.player {
                            Player::White => Direction::South,
                            Player::Black => Direction::North,
                        };

                        let capture_square =
                            mv.dst.in_direction(&inverse_pawn_move_direction).unwrap();

                        let remove_captured_pawn_mask = Bitboard::except_square(&capture_square);
                        new_bitboard &= remove_captured_pawn_mask;
                    }
                }
            }

            let king_start_square = *squares::king_start(&moved_piece.player);

            // PERF: Here, we figure out if the move was castling. It may be more performant to
            // tell this function that the move was castling, but it loses the cleanliness of
            // just telling the board the start and end destination for the piece.

            // If we just moved a king from its start square, we may have castled.
            if moved_piece.kind == PieceKind::King && mv.src == king_start_square {
                let kingside_square = *squares::kingside_castle_dest(&moved_piece.player);
                let queenside_square = *squares::queenside_castle_dest(&moved_piece.player);

                // We're castling!
                if mv.dst == kingside_square || mv.dst == queenside_square {
                    let is_kingside = mv.dst == kingside_square;

                    let rook_remove_mask = Bitboard::except_square(match is_kingside {
                        true => squares::kingside_rook_start(&moved_piece.player),
                        false => squares::queenside_rook_start(&moved_piece.player),
                    });

                    let rook_add_mask = match is_kingside {
                        true => match moved_piece.player {
                            Player::White => F1,
                            Player::Black => F8,
                        },
                        false => match moved_piece.player {
                            Player::White => D1,
                            Player::Black => D8,
                        },
                    }
                    .bitboard();

                    if *piece == Piece::new(moved_piece.player, PieceKind::Rook) {
                        new_bitboard &= rook_remove_mask;
                        new_bitboard |= rook_add_mask;
                    }
                }
            }

            new_bitboard
        };

        let new_board = Board {
            white_pieces: PlayerPieces {
                pawns: mask_bitboard(self.white_pieces.pawns, &Piece::WHITE_PAWN),
                knights: mask_bitboard(self.white_pieces.knights, &Piece::WHITE_KNIGHT),
                bishops: mask_bitboard(self.white_pieces.bishops, &Piece::WHITE_BISHOP),
                rooks: mask_bitboard(self.white_pieces.rooks, &Piece::WHITE_ROOK),
                queen: mask_bitboard(self.white_pieces.queen, &Piece::WHITE_QUEEN),
                king: mask_bitboard(self.white_pieces.king, &Piece::WHITE_KING),
            },
            black_pieces: PlayerPieces {
                pawns: mask_bitboard(self.black_pieces.pawns, &Piece::BLACK_PAWN),
                knights: mask_bitboard(self.black_pieces.knights, &Piece::BLACK_KNIGHT),
                bishops: mask_bitboard(self.black_pieces.bishops, &Piece::BLACK_BISHOP),
                rooks: mask_bitboard(self.black_pieces.rooks, &Piece::BLACK_ROOK),
                queen: mask_bitboard(self.black_pieces.queen, &Piece::BLACK_QUEEN),
                king: mask_bitboard(self.black_pieces.king, &Piece::BLACK_KING),
            },
        };

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
                .map(|rank| {
                    (0..8)
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
