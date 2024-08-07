use crate::chess::{
    movegen,
    piece::{Piece, PieceKind},
    player::Player,
    square::Square,
};

use crate::chess::bitboard::Bitboard;

#[derive(Clone)]
pub struct Board {
    pieces: [Bitboard; PieceKind::N],
    colors: Bitboard,
    squares: [Option<Piece>; Square::N],
}

impl Board {
    #[inline(always)]
    pub fn pieces(&self, player: Player) -> &PlayerPieces {
        &self.pieces[player.array_idx()]
    }

    #[inline(always)]
    pub fn white_pieces(&self) -> &PlayerPieces {
        self.pieces(Player::White)
    }

    #[inline(always)]
    pub fn black_pieces(&self) -> &PlayerPieces {
        self.pieces(Player::Black)
    }

    #[inline(always)]
    pub fn occupancy(&self) -> Bitboard {
        self.white_pieces().all() | self.black_pieces().all()
    }

    #[inline(always)]
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        // We know array_idx can only return up to Square::N - 1
        unsafe { *self.squares.get_unchecked(square.array_idx()) }
    }

    #[inline(always)]
    pub fn remove_at(&mut self, square: Square) -> bool {
        let Some(piece) = self.piece_at(square) else {
            return false;
        };

        self.pieces[piece.player.array_idx()].0[piece.kind.array_idx()].unset_inplace(square);
        self.squares[square.array_idx()] = None;
        true
    }

    #[inline(always)]
    pub fn set_at(&mut self, square: Square, piece: Piece) {
        self.pieces[piece.player.array_idx()].0[piece.kind.array_idx()].set_inplace(square);
        self.squares[square.array_idx()] = Some(piece);
    }

    pub fn king_in_check(&self, player: Player) -> bool {
        let king = self.pieces(player).king().single();
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
    type Error = ();

    fn try_from(squares: [Option<Piece>; Square::N]) -> Result<Self, ()> {
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

        for (i, maybe_piece) in squares.iter().enumerate() {
            if let Some(p) = maybe_piece {
                let square = Square::from_index(i.try_into().unwrap()).bb();

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
            pieces: [
                PlayerPieces::new([
                    white_pawns,
                    white_knights,
                    white_bishops,
                    white_rooks,
                    white_queens,
                    white_king,
                ]),
                PlayerPieces([
                    black_pawns,
                    black_knights,
                    black_bishops,
                    black_rooks,
                    black_queens,
                    black_king,
                ]),
            ],
            squares,
        })
    }
}
