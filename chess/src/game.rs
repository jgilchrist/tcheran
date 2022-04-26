use crate::{
    attacks::generate_all_attacks,
    bitboard::Bitboard,
    board::Board,
    direction::Direction,
    fen,
    movegen::generate_moves,
    moves::Move,
    player::Player,
    square::{self, Rank, Square},
};
use anyhow::Result;

// TODO: Flesh out this error type
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

impl CastleRights {
    pub fn can_castle(&self) -> bool {
        self.king_side || self.queen_side
    }

    pub fn none() -> CastleRights {
        CastleRights {
            king_side: false,
            queen_side: false,
        }
    }

    pub fn without_kingside(&self) -> CastleRights {
        CastleRights {
            king_side: false,
            queen_side: self.queen_side,
        }
    }

    pub fn without_queenside(&self) -> CastleRights {
        CastleRights {
            king_side: self.king_side,
            queen_side: false,
        }
    }
}

impl Default for CastleRights {
    fn default() -> Self {
        CastleRights {
            king_side: true,
            queen_side: true,
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub player: Player,
    pub board: Board,
    pub white_castle_rights: CastleRights,
    pub black_castle_rights: CastleRights,
    pub en_passant_target: Option<Square>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::start(),
            player: Player::White,
            white_castle_rights: CastleRights::default(),
            black_castle_rights: CastleRights::default(),
            en_passant_target: None,
        }
    }

    pub fn from_fen(fen: &str) -> Result<Game> {
        fen::parse(fen)
    }

    pub fn pseudo_legal_moves(&self) -> Vec<Move> {
        generate_moves(self)
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        self.pseudo_legal_moves()
            .into_iter()
            .filter(|m| self.is_legal(m))
            .collect()
    }

    fn is_legal(&self, mv: &Move) -> bool {
        let enemy_attacks = generate_all_attacks(&self.board, &self.player.other());

        let king_start_square = match self.player {
            Player::White => square::known::WHITE_KING_START,
            Player::Black => square::known::BLACK_KING_START,
        };

        let kingside_dst_square = match self.player {
            Player::White => square::known::WHITE_KINGSIDE_CASTLE,
            Player::Black => square::known::BLACK_KINGSIDE_CASTLE,
        };

        let queenside_dst_square = match self.player {
            Player::White => square::known::WHITE_QUEENSIDE_CASTLE,
            Player::Black => square::known::BLACK_QUEENSIDE_CASTLE,
        };

        if *mv == Move::new(king_start_square, kingside_dst_square)
            || *mv == Move::new(king_start_square, queenside_dst_square)
        {
            // If the king is in check, it cannot castle
            if enemy_attacks.has_square(&king_start_square) {
                return false;
            }

            // The king cannot castle if the intervening squares are under attack
            if *mv == Move::new(king_start_square, kingside_dst_square) {
                let kingside_required_not_attacked_squares = match self.player {
                    Player::White => vec![Square::F1, Square::G1],
                    Player::Black => vec![Square::F8, Square::G8],
                };

                if kingside_required_not_attacked_squares
                    .iter()
                    .any(|s| enemy_attacks.has_square(s))
                {
                    return false;
                }
            }

            if *mv == Move::new(king_start_square, queenside_dst_square) {
                let queenside_required_not_attacked_squares = match self.player {
                    Player::White => vec![Square::C1, Square::D1],
                    Player::Black => vec![Square::C8, Square::D8],
                };

                if queenside_required_not_attacked_squares
                    .iter()
                    .any(|s| enemy_attacks.has_square(s))
                {
                    return false;
                }
            }
        }

        !self
            .make_move(mv)
            .unwrap()
            .board
            .king_in_check(&self.player)
    }

    #[allow(unused)]
    pub fn make_move(&self, mv: &Move) -> Result<Game, MoveError> {
        let from = mv.src;
        let to = mv.dst;

        let (board, move_info) = self
            .board
            .make_move(mv)
            .map_err(|()| MoveError::InvalidMove)?;

        let piece_to_move = self
            .board
            .player_piece_at(&self.player, &from)
            .ok_or(MoveError::InvalidMove)?;

        let dst_square_occupation = self.board.piece_at(&to);

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
            Player::White => Rank::R2,
            Player::Black => Rank::R7,
        };

        let en_passant_target = if from.rank() == back_rank
            && to
                == from
                    .in_direction(&pawn_move_direction)
                    .and_then(|s| s.in_direction(&pawn_move_direction))
                    .unwrap()
        {
            let to_bb = Bitboard::from_square(&to);
            let en_passant_attacker_squares = to_bb.west() | to_bb.east();
            let enemy_pawns = match self.player {
                Player::White => self.board.black_pieces.pawns,
                Player::Black => self.board.white_pieces.pawns,
            };
            let en_passant_can_happen = !(en_passant_attacker_squares & enemy_pawns).is_empty();

            if en_passant_can_happen {
                Some(from.in_direction(&pawn_move_direction).unwrap())
            } else {
                None
            }
        } else {
            None
        };

        let king_start_square = match self.player {
            Player::White => square::known::WHITE_KING_START,
            Player::Black => square::known::BLACK_KING_START,
        };

        let kingside_rook_start_square = match self.player {
            Player::White => square::known::WHITE_KING_START,
            Player::Black => square::known::BLACK_KING_START,
        };

        let white_castle_rights = if self.player == Player::White {
            match (mv.src, self.white_castle_rights) {
                (square::known::WHITE_KING_START, _) => CastleRights::none(),
                (
                    square::known::WHITE_KINGSIDE_ROOK_START,
                    rights @ CastleRights {
                        king_side: true, ..
                    },
                ) => rights.without_kingside(),
                (
                    square::known::WHITE_QUEENSIDE_ROOK_START,
                    rights @ CastleRights {
                        queen_side: true, ..
                    },
                ) => rights.without_queenside(),
                _ => self.white_castle_rights,
            }
        } else {
            match mv.dst {
                square::known::WHITE_KINGSIDE_ROOK_START => {
                    self.white_castle_rights.without_kingside()
                }
                square::known::WHITE_QUEENSIDE_ROOK_START => {
                    self.white_castle_rights.without_queenside()
                }
                _ => self.white_castle_rights,
            }
        };

        let black_castle_rights = if self.player == Player::Black {
            match (mv.src, self.black_castle_rights) {
                (square::known::BLACK_KING_START, _) => CastleRights::none(),
                (
                    square::known::BLACK_KINGSIDE_ROOK_START,
                    rights @ CastleRights {
                        king_side: true, ..
                    },
                ) => rights.without_kingside(),
                (
                    square::known::BLACK_QUEENSIDE_ROOK_START,
                    rights @ CastleRights {
                        queen_side: true, ..
                    },
                ) => rights.without_queenside(),
                _ => self.black_castle_rights,
            }
        } else {
            match mv.dst {
                square::known::BLACK_KINGSIDE_ROOK_START => {
                    self.black_castle_rights.without_kingside()
                }
                square::known::BLACK_QUEENSIDE_ROOK_START => {
                    self.black_castle_rights.without_queenside()
                }
                _ => self.black_castle_rights,
            }
        };

        Ok(Game {
            board,
            player: self.player.other(),
            white_castle_rights,
            black_castle_rights,
            en_passant_target,
        })
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}
