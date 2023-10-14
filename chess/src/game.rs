use crate::{
    board::Board,
    direction::Direction,
    fen,
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
    pub fullmove_number: u32,
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
            fullmove_number: 1,
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

        let (board, move_info) = self
            .board
            .make_move(mv)
            .map_err(|()| MoveError::InvalidMove)?;

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

        let fullmove_number = if self.player == Player::Black {
            self.fullmove_number + 1
        } else {
            self.fullmove_number
        };

        Ok(Self {
            board,
            player: self.player.other(),
            white_castle_rights,
            black_castle_rights,
            en_passant_target,
            fullmove_number,
        })
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
