use crate::{
    attacks::generate_all_attacks,
    bitboard,
    board::Board,
    direction::Direction,
    fen,
    movegen::generate_moves,
    moves::Move,
    piece::PieceKind,
    player::Player,
    square::{
        squares::{self, *},
        Square,
    },
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

    pub fn to_fen(&self) -> String {
        fen::write(&self)
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
        let piece_to_move = self.board.player_piece_at(&self.player, &mv.src).unwrap();

        let king_start_square = *squares::king_start(&self.player);
        let kingside_dst_square = *squares::kingside_castle_dest(&self.player);
        let queenside_dst_square = *squares::queenside_castle_dest(&self.player);

        if piece_to_move == PieceKind::King
            // PERF: Don't create these moves on every single request
            && (*mv == Move::new(king_start_square, kingside_dst_square)
                || *mv == Move::new(king_start_square, queenside_dst_square))
        {
            // If the king is in check, it cannot castle
            if enemy_attacks.has_square(&king_start_square) {
                return false;
            }

            // The king cannot castle if the intervening squares are under attack
            if *mv == Move::new(king_start_square, kingside_dst_square) {
                let kingside_required_not_attacked_squares = match self.player {
                    // PERF: Don't allocate these vectors on every single call
                    Player::White => vec![F1, G1],
                    Player::Black => vec![F8, G8],
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
                    Player::White => vec![C1, D1],
                    Player::Black => vec![C8, D8],
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
            Player::White => bitboard::known::RANK_2,
            Player::Black => bitboard::known::RANK_7,
        };

        let en_passant_target = if piece_to_move == PieceKind::Pawn
            && back_rank.has_square(&from)
            && to
                == from
                    .in_direction(&pawn_move_direction)
                    .and_then(|s| s.in_direction(&pawn_move_direction))
                    .unwrap()
        {
            let to_bb = to.bitboard();
            let en_passant_attacker_squares = to_bb.west() | to_bb.east();
            let enemy_pawns = self.board.player_pieces(&self.player.other()).pawns;
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
        let castle_rights = |player: &Player, castle_rights: &CastleRights| {
            let our_king_start = *squares::king_start(player);
            let our_kingside_rook = *squares::kingside_rook_start(player);
            let our_queenside_rook = *squares::queenside_rook_start(player);

            if self.player == *player {
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

        let white_castle_rights = castle_rights(&Player::White, &self.white_castle_rights);
        let black_castle_rights = castle_rights(&Player::Black, &self.black_castle_rights);

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
