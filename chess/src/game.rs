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

    pub fn remove_kingside_rights(&mut self) {
        self.king_side = false;
    }

    #[must_use]
    pub const fn without_queenside(&self) -> Self {
        Self {
            king_side: self.king_side,
            queen_side: false,
        }
    }

    pub fn remove_queenside_rights(&mut self) {
        self.queen_side = false;
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

    pub fn make_move(&self, mv: &Move) -> Result<Self, MoveError> {
        let from = mv.src;
        let to = mv.dst;

        let moved_piece = self.board.piece_at(from).ok_or(MoveError::InvalidMove)?;
        let maybe_captured_piece = self.board.piece_at(to);

        let mut board = self.board;

        board.remove_at(from);

        if maybe_captured_piece.is_some() {
            board.remove_at(to);
        }

        if let Some(promoted_to) = mv.promotion {
            board.set_at(to, Piece::new(moved_piece.player, promoted_to.piece()));
        } else {
            board.set_at(to, moved_piece);
        }

        // If we just moved a pawn diagonally, we need to double check whether it was en-passant,
        // in which case we need to remove the captured pawn.
        //
        // PERF: It may be more performant to
        // tell this function that the move was en-passant, but it loses the cleanliness of
        // just telling the board the start and end destination for the piece.
        //
        // PERF: We only need to check mv.is_diagonal() if we moved from the rank where
        // en-passant can happen which is likely a much cheaper check (just bitwise and).
        if moved_piece.kind == PieceKind::Pawn {
            let pawn_attacks = move_tables::pawn_attacks(from, moved_piece.player);

            if pawn_attacks.contains(to) {
                let opponent_pieces = self.board.player_pieces(moved_piece.player.other()).all();

                // Definitely en-passant, as we made a capture but there was no piece on that square.
                if !opponent_pieces.contains(to) {
                    // Get the square that we need to remove the pawn from.
                    let inverse_pawn_move_direction = match moved_piece.player {
                        Player::White => Direction::South,
                        Player::Black => Direction::North,
                    };

                    let capture_square = to.in_direction(&inverse_pawn_move_direction).unwrap();
                    board.remove_at(capture_square);
                }
            }
        }

        let pawn_move_direction = match self.player {
            Player::White => Direction::North,
            Player::Black => Direction::South,
        };

        let back_rank = match self.player {
            Player::White => squares::RANK_2,
            Player::Black => squares::RANK_7,
        };

        let double_push_rank = match self.player {
            Player::White => squares::RANK_4,
            Player::Black => squares::RANK_5,
        };

        let en_passant_target = if moved_piece.kind == PieceKind::Pawn
            && back_rank.contains(from)
            && double_push_rank.contains(to)
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

        // If we just moved a king from its start square, we may have castled.
        //
        // PERF: Here, we figure out if the move was castling. It may be more performant to
        // tell this function that the move was castling, but it loses the cleanliness of
        // just telling the board the start and end destination for the piece.
        if moved_piece.kind == PieceKind::King && from == squares::king_start(moved_piece.player) {
            // We're castling!
            if to == squares::kingside_castle_dest(moved_piece.player) {
                let rook_remove_square = squares::kingside_rook_start(moved_piece.player);
                let rook_add_square = match moved_piece.player {
                    Player::White => F1,
                    Player::Black => F8,
                };

                board.remove_at(rook_remove_square);
                board.set_at(
                    rook_add_square,
                    Piece::new(moved_piece.player, PieceKind::Rook),
                );
            } else if to == squares::queenside_castle_dest(moved_piece.player) {
                let rook_remove_square = squares::queenside_rook_start(moved_piece.player);
                let rook_add_square = match moved_piece.player {
                    Player::White => D1,
                    Player::Black => D8,
                };
                board.remove_at(rook_remove_square);
                board.set_at(
                    rook_add_square,
                    Piece::new(moved_piece.player, PieceKind::Rook),
                );
            }
        }

        let (mut castle_rights, mut other_player_castle_rights) = match self.player {
            Player::White => (self.white_castle_rights, self.black_castle_rights),
            Player::Black => (self.black_castle_rights, self.white_castle_rights),
        };

        if moved_piece.kind == PieceKind::King && from == squares::king_start(self.player) {
            castle_rights.remove_kingside_rights();
            castle_rights.remove_queenside_rights();
        } else if moved_piece.kind == PieceKind::Rook {
            if from == squares::kingside_rook_start(self.player) {
                castle_rights.remove_kingside_rights();
            } else if from == squares::queenside_rook_start(self.player) {
                castle_rights.remove_queenside_rights();
            }
        }

        if maybe_captured_piece.is_some() {
            if to == squares::kingside_rook_start(self.player.other()) {
                other_player_castle_rights.remove_kingside_rights();
            } else if to == squares::queenside_rook_start(self.player.other()) {
                other_player_castle_rights.remove_queenside_rights();
            }
        }

        let (white_castle_rights, black_castle_rights) = match self.player {
            Player::White => (castle_rights, other_player_castle_rights),
            Player::Black => (other_player_castle_rights, castle_rights),
        };

        let should_reset_halfmove_clock =
            maybe_captured_piece.is_some() || moved_piece.kind == PieceKind::Pawn;
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
