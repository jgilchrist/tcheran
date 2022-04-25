use crate::{
    bitboard::Bitboard,
    board::Board,
    direction::Direction,
    movegen::generate_moves,
    player::Player,
    r#move::Move,
    square::{self, Rank, Square},
};

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

    pub fn pseudo_legal_moves(&self) -> Vec<Move> {
        generate_moves(self)
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        self.pseudo_legal_moves()
            .into_iter()
            .filter(|m| {
                !self
                    .make_move(m)
                    .unwrap()
                    .king_in_check(&self.player)
            })
            .collect()
    }

    // FIXME: Should be able to be determined from `Board`, but movegen
    // currently requires a full `Game`. Generating attacked pieces should
    // be a more straightforward way to check this.
    //
    // PERF: There's likely more efficient ways to do this than generating
    // all legal moves
    pub fn king_in_check(&self, player: &Player) -> bool {
        let king = match player {
            Player::White => self.board.white_pieces.king,
            Player::Black => self.board.black_pieces.king,
        }
        .to_square_definite();

        let other_player_moves = generate_moves(&Game {
            player: player.other(),
            ..*self
        });

        other_player_moves.iter().any(|m| m.dst == king)
    }

    #[allow(unused)]
    pub fn make_move(&self, r#move: &Move) -> Result<Game, MoveError> {
        let from = r#move.src;
        let to = r#move.dst;

        let (board, move_info) = self
            .board
            .make_move(r#move)
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
            match (&r#move.src, self.white_castle_rights) {
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
            self.white_castle_rights
        };

        let black_castle_rights = if self.player == Player::Black {
            match (&r#move.src, self.black_castle_rights) {
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
            self.black_castle_rights
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
