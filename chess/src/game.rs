use crate::board::PlayerPieces;
use crate::piece::Piece;
use crate::squares::all::*;
use crate::{
    board::Board,
    direction::Direction,
    fen, move_tables,
    movegen::{self, generate_moves},
    moves::{self, Move},
    piece::PieceKind,
    player::Player,
    square::Square,
    squares::{self, Squares},
};
use anyhow::Result;

#[derive(Debug)]
pub enum MoveError {
    InvalidMove,
}

#[derive(Copy, Clone, Debug)]
#[allow(unused)]
pub struct CastleRights {
    pub king_side: bool,
    pub queen_side: bool,
}

pub enum GameStatus {
    Won,
    Lost,
    Stalemate,
}

impl CastleRights {
    #[must_use]
    pub const fn can_castle(&self) -> bool {
        self.king_side || self.queen_side
    }

    #[must_use]
    pub const fn none() -> Self {
        Self {
            king_side: false,
            queen_side: false,
        }
    }

    #[must_use]
    pub const fn without_kingside(&self) -> Self {
        Self {
            king_side: false,
            queen_side: self.queen_side,
        }
    }

    #[must_use]
    pub const fn without_queenside(&self) -> Self {
        Self {
            king_side: self.king_side,
            queen_side: false,
        }
    }
}

impl Default for CastleRights {
    fn default() -> Self {
        Self {
            king_side: true,
            queen_side: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub player: Player,
    pub board: Board,
    pub white_castle_rights: CastleRights,
    pub black_castle_rights: CastleRights,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u32,
    pub plies: u32,
}

impl Game {
    #[must_use]
    pub fn new() -> Self {
        Self {
            board: Board::start(),
            player: Player::White,
            white_castle_rights: CastleRights::default(),
            black_castle_rights: CastleRights::default(),
            en_passant_target: None,
            halfmove_clock: 0,
            plies: 0,
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self> {
        fen::parse(fen)
    }

    #[must_use]
    pub fn to_fen(&self) -> String {
        fen::write(self)
    }

    #[must_use]
    pub fn turn(&self) -> u32 {
        self.plies / 2 + 1
    }

    #[must_use]
    pub fn pseudo_legal_moves(&self) -> Vec<Move> {
        generate_moves(self)
    }

    #[must_use]
    pub fn legal_moves(&self) -> Vec<Move> {
        self.pseudo_legal_moves()
            .into_iter()
            .filter(|m| self.is_legal(m))
            .collect()
    }

    #[must_use]
    pub fn game_status(&self) -> Option<GameStatus> {
        if self.halfmove_clock >= 100 {
            return Some(GameStatus::Stalemate);
        }

        let legal_moves = self.legal_moves();
        if !legal_moves.is_empty() {
            return None;
        }

        if self.board.king_in_check(self.player.other()) {
            return Some(GameStatus::Won);
        }

        if self.board.king_in_check(self.player) {
            return Some(GameStatus::Lost);
        }

        Some(GameStatus::Stalemate)
    }

    fn is_legal(&self, mv: &Move) -> bool {
        let piece_to_move = self.board.player_piece_at(self.player, mv.src).unwrap();

        if piece_to_move == PieceKind::King {
            let enemy_attacks = movegen::generate_all_attacks(&self.board, self.player.other());

            let kingside_castle_move = moves::known::kingside_castle_move(self.player);
            let queenside_castle_move = moves::known::queenside_castle_move(self.player);

            if mv == kingside_castle_move || mv == queenside_castle_move {
                let king_start_square = squares::king_start(self.player);

                // If the king is in check, it cannot castle
                if enemy_attacks.contains(king_start_square) {
                    return false;
                }

                let kingside_required_not_attacked_squares =
                    squares::kingside_required_not_attacked_squares(self.player);

                // The king cannot castle if the intervening squares are under attack
                if mv == kingside_castle_move
                    && !(enemy_attacks & kingside_required_not_attacked_squares).is_empty()
                {
                    return false;
                }

                let queenside_required_not_attacked_squares =
                    squares::queenside_required_not_attacked_squares(self.player);

                if mv == queenside_castle_move
                    && !(enemy_attacks & queenside_required_not_attacked_squares).is_empty()
                {
                    return false;
                }
            }
        }

        !self.make_move(mv).unwrap().board.king_in_check(self.player)
    }

    #[allow(unused)]
    pub fn make_move(&self, mv: &Move) -> Result<Self, MoveError> {
        let from = mv.src;
        let to = mv.dst;

        let moved_piece = self.board.piece_at(mv.src).ok_or(MoveError::InvalidMove)?;

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
                    let opponent_pieces =
                        self.board.player_pieces(moved_piece.player.other()).all();

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

        let board = Board {
            white_pieces: PlayerPieces {
                pawns: mask_squares(self.board.white_pieces.pawns, &Piece::WHITE_PAWN),
                knights: mask_squares(self.board.white_pieces.knights, &Piece::WHITE_KNIGHT),
                bishops: mask_squares(self.board.white_pieces.bishops, &Piece::WHITE_BISHOP),
                rooks: mask_squares(self.board.white_pieces.rooks, &Piece::WHITE_ROOK),
                queens: mask_squares(self.board.white_pieces.queens, &Piece::WHITE_QUEEN),
                king: mask_squares(self.board.white_pieces.king, &Piece::WHITE_KING),
            },
            black_pieces: PlayerPieces {
                pawns: mask_squares(self.board.black_pieces.pawns, &Piece::BLACK_PAWN),
                knights: mask_squares(self.board.black_pieces.knights, &Piece::BLACK_KNIGHT),
                bishops: mask_squares(self.board.black_pieces.bishops, &Piece::BLACK_BISHOP),
                rooks: mask_squares(self.board.black_pieces.rooks, &Piece::BLACK_ROOK),
                queens: mask_squares(self.board.black_pieces.queens, &Piece::BLACK_QUEEN),
                king: mask_squares(self.board.black_pieces.king, &Piece::BLACK_KING),
            },
        };

        let piece_to_move = self
            .board
            .player_piece_at(self.player, from)
            .ok_or(MoveError::InvalidMove)?;

        let dst_square_occupation = self.board.piece_at(to);

        let captured_piece = match dst_square_occupation {
            Some(piece) => {
                if piece.player == self.player {
                    // Tried to capture own piece
                    return Err(MoveError::InvalidMove);
                }

                Ok(Some(piece))
            }
            None => Ok(None),
        }?;

        let pawn_move_direction = match self.player {
            Player::White => Direction::North,
            Player::Black => Direction::South,
        };

        let back_rank = match self.player {
            Player::White => squares::RANK_2,
            Player::Black => squares::RANK_7,
        };

        let en_passant_target = if piece_to_move == PieceKind::Pawn
            && back_rank.contains(from)
            && to
                == from
                    .in_direction(&pawn_move_direction)
                    .and_then(|s| s.in_direction(&pawn_move_direction))
                    .unwrap()
        {
            let to_square = Squares::from_square(to);
            let en_passant_attacker_squares = to_square.west() | to_square.east();
            let enemy_pawns = self.board.player_pieces(self.player.other()).pawns;
            let en_passant_can_happen = !(en_passant_attacker_squares & enemy_pawns).is_empty();

            if en_passant_can_happen {
                Some(from.in_direction(&pawn_move_direction).unwrap())
            } else {
                None
            }
        } else {
            None
        };

        // FIXME: We update the castle rights when any piece moves off of/onto
        // the appropriate squares. This makes for simpler code, but will end up
        // copying CastleRights more than we need to.
        let castle_rights = |player: Player, castle_rights: &CastleRights| {
            let our_king_start = squares::king_start(player);
            let our_kingside_rook = squares::kingside_rook_start(player);
            let our_queenside_rook = squares::queenside_rook_start(player);

            if self.player == player {
                match mv.src {
                    s if s == our_king_start => CastleRights::none(),
                    s if s == our_kingside_rook => castle_rights.without_kingside(),
                    s if s == our_queenside_rook => castle_rights.without_queenside(),
                    _ => *castle_rights,
                }
            } else {
                match mv.dst {
                    s if s == our_kingside_rook => castle_rights.without_kingside(),
                    s if s == our_queenside_rook => castle_rights.without_queenside(),
                    _ => *castle_rights,
                }
            }
        };

        let white_castle_rights = castle_rights(Player::White, &self.white_castle_rights);
        let black_castle_rights = castle_rights(Player::Black, &self.black_castle_rights);

        let should_reset_halfmove_clock =
            captured_piece.is_some() || piece_to_move == PieceKind::Pawn;
        let halfmove_clock = if should_reset_halfmove_clock {
            0
        } else {
            self.halfmove_clock
        };

        let plies = self.plies + 1;

        Ok(Self {
            board,
            player: self.player.other(),
            white_castle_rights,
            black_castle_rights,
            en_passant_target,
            halfmove_clock,
            plies,
        })
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
