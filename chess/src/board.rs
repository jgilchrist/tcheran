use crate::{
    direction::Direction,
    move_tables, movegen,
    moves::Move,
    piece::{Piece, PieceKind},
    player::Player,
    square::Square,
    squares::{self, all::*, Squares},
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

    // Does not consider move legality. Just concerned with the implementation details
    // of removing a piece from one square and putting it on another one (and dealing with
    // any interesting consequences (e.g. en-passant/castling))
    //
    // PERF: There's likely many more efficient ways to do this.
    // A good target for optimisation once things are working.
    //
    // TODO: Return info about the move (was it a capture?)
    #[allow(clippy::result_unit_err)]
    pub fn make_move(&self, mv: &Move) -> Result<(Self, ()), ()> {
        let moved_piece = self.piece_at(mv.src).ok_or(())?;

        let remove_src_mask = Squares::all_except(mv.src);
        let remove_from_dst_mask = Squares::all_except(mv.dst);

        let add_piece_to_dst_mask = |piece: &Piece| {
            if *piece == moved_piece {
                Squares::from_square(mv.dst)
            } else {
                Squares::none()
            }
        };

        let mask_squares = |squares: Squares, piece: &Piece| {
            let mut new_squares = squares
                // Remove the piece that is being moved
                // Currently unconditional as it's the same as removing for all pieces
                & remove_src_mask
                // Remove any piece currently occupying the destination square
                & remove_from_dst_mask
                // Add the piece that is being moved to the destination square
                | add_piece_to_dst_mask(piece);

            if let Some(promoted_to) = mv.promotion {
                // The promoted pawn has turned into another piece
                let remove_promoted_pawn_mask = Squares::all_except(mv.dst);

                let add_promoted_piece_mask =
                    if *piece == Piece::new(moved_piece.player, promoted_to.piece()) {
                        Squares::from_square(mv.dst)
                    } else {
                        Squares::none()
                    };

                // Place that piece on the board
                new_squares &= remove_promoted_pawn_mask;
                new_squares |= add_promoted_piece_mask;
            }

            // PERF: Here, we figure out if the move was en-passant. It may be more performant to
            // tell this function that the move was en-passant, but it loses the cleanliness of
            // just telling the board the start and end destination for the piece.
            //
            // PERF: We only need to check mv.is_diagonal() if we moved from the rank where
            // en-passant can happen which is likely a much cheaper check (just bitwise and).
            //
            // If we just moved a pawn diagonally, we need to double check whether it was en-passant,
            // in which case we need to remove the captured pawn.
            if moved_piece.kind == PieceKind::Pawn {
                let pawn_attacks = move_tables::pawn_attacks(mv.src, moved_piece.player);

                if pawn_attacks.contains(mv.dst) {
                    let opponent_pieces = self.player_pieces(moved_piece.player.other()).all();

                    // Definitely en-passant, as we made a capture but there was no piece on that square.
                    if !opponent_pieces.contains(mv.dst) {
                        // Get the square that we need to remove the pawn from.
                        let inverse_pawn_move_direction = match moved_piece.player {
                            Player::White => Direction::South,
                            Player::Black => Direction::North,
                        };

                        let capture_square =
                            mv.dst.in_direction(&inverse_pawn_move_direction).unwrap();

                        let remove_captured_pawn_mask = Squares::all_except(capture_square);
                        new_squares &= remove_captured_pawn_mask;
                    }
                }
            }

            let king_start_square = squares::king_start(moved_piece.player);

            // PERF: Here, we figure out if the move was castling. It may be more performant to
            // tell this function that the move was castling, but it loses the cleanliness of
            // just telling the board the start and end destination for the piece.

            // If we just moved a king from its start square, we may have castled.
            if moved_piece.kind == PieceKind::King && mv.src == king_start_square {
                let kingside_square = squares::kingside_castle_dest(moved_piece.player);
                let queenside_square = squares::queenside_castle_dest(moved_piece.player);

                // We're castling!
                if mv.dst == kingside_square || mv.dst == queenside_square {
                    let is_kingside = mv.dst == kingside_square;

                    let rook_remove_mask = Squares::all_except(if is_kingside {
                        squares::kingside_rook_start(moved_piece.player)
                    } else {
                        squares::queenside_rook_start(moved_piece.player)
                    });

                    let rook_add_mask = if is_kingside {
                        match moved_piece.player {
                            Player::White => F1,
                            Player::Black => F8,
                        }
                    } else {
                        match moved_piece.player {
                            Player::White => D1,
                            Player::Black => D8,
                        }
                    };

                    if *piece == Piece::new(moved_piece.player, PieceKind::Rook) {
                        new_squares &= rook_remove_mask;
                        new_squares |= rook_add_mask;
                    }
                }
            }

            new_squares
        };

        let new_board = Self {
            white_pieces: PlayerPieces {
                pawns: mask_squares(self.white_pieces.pawns, &Piece::WHITE_PAWN),
                knights: mask_squares(self.white_pieces.knights, &Piece::WHITE_KNIGHT),
                bishops: mask_squares(self.white_pieces.bishops, &Piece::WHITE_BISHOP),
                rooks: mask_squares(self.white_pieces.rooks, &Piece::WHITE_ROOK),
                queens: mask_squares(self.white_pieces.queens, &Piece::WHITE_QUEEN),
                king: mask_squares(self.white_pieces.king, &Piece::WHITE_KING),
            },
            black_pieces: PlayerPieces {
                pawns: mask_squares(self.black_pieces.pawns, &Piece::BLACK_PAWN),
                knights: mask_squares(self.black_pieces.knights, &Piece::BLACK_KNIGHT),
                bishops: mask_squares(self.black_pieces.bishops, &Piece::BLACK_BISHOP),
                rooks: mask_squares(self.black_pieces.rooks, &Piece::BLACK_ROOK),
                queens: mask_squares(self.black_pieces.queens, &Piece::BLACK_QUEEN),
                king: mask_squares(self.black_pieces.king, &Piece::BLACK_KING),
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
