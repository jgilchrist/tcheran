use crate::{board::Board, player::Player, r#move::Move};

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
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::start(),
            player: Player::White,
            white_castle_rights: CastleRights::default(),
            black_castle_rights: CastleRights::default(),
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

        // TODO: Update castle rights

        Ok(Game {
            board,
            player: self.player.other(),
            white_castle_rights: self.white_castle_rights,
            black_castle_rights: self.black_castle_rights,
        })
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}
