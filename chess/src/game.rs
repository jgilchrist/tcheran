use crate::{
    bitboard::Bitboard,
    board::Board,
    direction::Direction,
    player::Player,
    r#move::Move,
    square::{Rank, Square},
};

// TODO: Flesh out this error type
#[derive(Debug)]
pub enum MoveError {
    InvalidMove,
}

#[derive(Copy, Clone, Debug)]
#[allow(unused)]
pub struct CastleRights {
    king_side: bool,
    queen_side: bool,
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

        let en_passant_target = if from.rank == back_rank
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

        // TODO: Update castle rights

        Ok(Game {
            board,
            player: self.player.other(),
            white_castle_rights: self.white_castle_rights,
            black_castle_rights: self.black_castle_rights,
            en_passant_target,
        })
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}
