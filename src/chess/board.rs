use crate::chess::{
    movegen,
    piece::{Piece, PieceKind},
    player::Player,
    square::Square,
};

use crate::chess::bitboard::Bitboard;
use crate::chess::player::ByPlayer;

#[derive(Clone)]
pub struct Board {
    pieces: [Bitboard; PieceKind::N],
    colors: ByPlayer<Bitboard>,
    squares: [Option<Piece>; Square::N],
}

impl Board {
    #[inline(always)]
    pub fn occupancy(&self) -> Bitboard {
        self.occupancy_for(Player::White) | self.occupancy_for(Player::Black)
    }

    #[inline(always)]
    pub fn occupancy_for(&self, player: Player) -> Bitboard {
        *self.colors.for_player(player)
    }

    #[inline(always)]
    pub fn pieces_of_kind(&self, kind: PieceKind, player: Player) -> Bitboard {
        self.pieces[kind.array_idx()] & self.occupancy_for(player)
    }

    pub fn pawns(&self, player: Player) -> Bitboard {
        self.all_pawns() & self.occupancy_for(player)
    }

    pub fn all_pawns(&self) -> Bitboard {
        self.pieces[PieceKind::Pawn.array_idx()]
    }

    pub fn knights(&self, player: Player) -> Bitboard {
        self.all_knights() & self.occupancy_for(player)
    }

    pub fn all_knights(&self) -> Bitboard {
        self.pieces[PieceKind::Knight.array_idx()]
    }

    pub fn bishops(&self, player: Player) -> Bitboard {
        self.all_bishops() & self.occupancy_for(player)
    }

    pub fn all_bishops(&self) -> Bitboard {
        self.pieces[PieceKind::Bishop.array_idx()]
    }

    pub fn rooks(&self, player: Player) -> Bitboard {
        self.all_rooks() & self.occupancy_for(player)
    }

    pub fn all_rooks(&self) -> Bitboard {
        self.pieces[PieceKind::Rook.array_idx()]
    }

    pub fn queens(&self, player: Player) -> Bitboard {
        self.all_queens() & self.occupancy_for(player)
    }

    pub fn all_queens(&self) -> Bitboard {
        self.pieces[PieceKind::Queen.array_idx()]
    }

    pub fn king(&self, player: Player) -> Bitboard {
        self.all_kings() & self.occupancy_for(player)
    }

    pub fn all_kings(&self) -> Bitboard {
        self.pieces[PieceKind::King.array_idx()]
    }

    pub fn diagonal_sliders(&self, player: Player) -> Bitboard {
        self.bishops(player) | self.queens(player)
    }

    pub fn all_diagonal_sliders(&self) -> Bitboard {
        self.all_bishops() | self.all_queens()
    }

    pub fn orthogonal_sliders(&self, player: Player) -> Bitboard {
        self.rooks(player) | self.queens(player)
    }

    pub fn all_orthogonal_sliders(&self) -> Bitboard {
        self.all_rooks() | self.all_queens()
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

        self.pieces[piece.kind.array_idx()] ^= square.bb();
        self.colors
            .for_player_mut(piece.player)
            .unset_inplace(square);
        self.squares[square.array_idx()] = None;
        true
    }

    #[inline(always)]
    pub fn set_at(&mut self, square: Square, piece: Piece) {
        self.pieces[piece.kind.array_idx()] |= square.bb();
        self.colors.for_player_mut(piece.player).set_inplace(square);
        self.squares[square.array_idx()] = Some(piece);
    }

    pub fn king_in_check(&self, player: Player) -> bool {
        let king = self.king(player).single();
        let enemy_attackers = movegen::generate_attackers_of(self, player, king);
        enemy_attackers.any()
    }

    #[allow(clippy::allow_attributes, reason = "Only used in non-release mode")]
    #[allow(unused, reason = "Only used in non-release mode")]
    pub fn flip_vertically(&self) -> Self {
        let [white_colors, black_colors] = self.colors.inner();
        let [pawns, knights, bishops, rooks, queens, king] = self.pieces;

        let squares = self.squares;
        let mut flipped_squares: [Option<Piece>; Square::N] = [None; Square::N];
        for rank in 0..8 {
            for file in 0..8 {
                flipped_squares[(8 - rank - 1) * 8 + file] = squares[rank * 8 + file];
            }
        }

        Self {
            colors: ByPlayer::new(
                white_colors.flip_vertically(),
                black_colors.flip_vertically(),
            ),
            pieces: [
                pawns.flip_vertically(),
                knights.flip_vertically(),
                bishops.flip_vertically(),
                rooks.flip_vertically(),
                queens.flip_vertically(),
                king.flip_vertically(),
            ],
            squares: flipped_squares,
        }
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

        let pawns = white_pawns | black_pawns;
        let knights = white_knights | black_knights;
        let bishops = white_bishops | black_bishops;
        let rooks = white_rooks | black_rooks;
        let queens = white_queens | black_queens;
        let kings = white_king | black_king;

        let white_occupancy =
            white_pawns | white_knights | white_bishops | white_rooks | white_queens | white_king;

        let black_occupancy =
            black_pawns | black_knights | black_bishops | black_rooks | black_queens | black_king;

        Ok(Self {
            pieces: [pawns, knights, bishops, rooks, queens, kings],
            colors: ByPlayer::new(white_occupancy, black_occupancy),
            squares,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::game::Game;

    #[test]
    fn test_flip_vertically() {
        let game =
            Game::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -")
                .unwrap();

        let our_flipped_board = game.board.flip_vertically();

        let flipped_kiwipete =
            Game::from_fen("R3K2R/PPPBBPPP/2N2Q1p/1p2P3/3PN3/bn2pnp1/p1ppqpb1/r3k2r w - - 0 1")
                .unwrap()
                .board;

        assert_eq!(
            our_flipped_board.colors.for_player(Player::White),
            flipped_kiwipete.colors.for_player(Player::White)
        );
        assert_eq!(
            our_flipped_board.colors.for_player(Player::Black),
            flipped_kiwipete.colors.for_player(Player::Black)
        );
        assert_eq!(our_flipped_board.pieces, flipped_kiwipete.pieces);

        for (i, &p) in our_flipped_board.squares.iter().enumerate() {
            assert_eq!(p, flipped_kiwipete.squares[i]);
        }
    }
}
