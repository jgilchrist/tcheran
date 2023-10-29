use crate::piece::Piece;
use crate::zobrist::ZobristHash;
use crate::{
    board::Board,
    direction::Direction,
    fen,
    movegen::generate_moves,
    moves::Move,
    piece::PieceKind,
    player::Player,
    square::Square,
    squares::{self, Squares},
    zobrist,
};
use anyhow::Result;

#[derive(Debug)]
pub enum MoveError {
    InvalidMove,
}

#[derive(Debug, Copy, Clone)]
pub enum CastleRightsSide {
    Kingside,
    Queenside,
}

impl CastleRightsSide {
    pub const N: usize = 2;

    #[must_use]
    pub fn array_idx(&self) -> usize {
        match self {
            Self::Kingside => 0,
            Self::Queenside => 1,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CastleRights {
    pub king_side: bool,
    pub queen_side: bool,
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
pub struct History {
    pub mv: Move,
    pub captured: Option<Piece>,
    pub white_castle_rights: CastleRights,
    pub black_castle_rights: CastleRights,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u32,
    pub zobrist: ZobristHash,
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

    pub zobrist: ZobristHash,
    pub history: Vec<History>,
}

impl Game {
    #[must_use]
    pub fn new() -> Self {
        Self::from_state(
            Board::start(),
            Player::White,
            CastleRights::default(),
            CastleRights::default(),
            None,
            0,
            0,
        )
    }

    #[must_use]
    pub fn from_state(
        board: Board,
        player: Player,
        white_castle_rights: CastleRights,
        black_castle_rights: CastleRights,
        en_passant_target: Option<Square>,
        halfmove_clock: u32,
        plies: u32,
    ) -> Self {
        let mut game = Self {
            board,
            player,
            white_castle_rights,
            black_castle_rights,
            en_passant_target,
            halfmove_clock,
            plies,

            zobrist: ZobristHash::uninit(),
            history: Vec::new(),
        };

        let zobrist = zobrist::hash(&game);
        game.zobrist = zobrist;
        game
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
    pub fn is_stalemate_by_fifty_move_rule(&self) -> bool {
        // TODO: Make sure that the order of checking checkmates vs. draws in search
        // won't cause us to detect a draw when we should have checkmate.
        self.halfmove_clock >= 100
    }

    #[must_use]
    pub fn is_stalemate_by_repetition(&self) -> bool {
        // PERF: We only need to search up to the last irreversible move
        let mut count = 0;

        for seen_state in self.history.iter().rev() {
            if self.zobrist == seen_state.zobrist {
                count += 1;
            }

            // We've seen the current state twice before, so it has occurred three times overall
            // This is a draw by threefold repetition.
            if count == 2 {
                return true;
            }
        }

        false
    }

    fn set_at(&mut self, sq: Square, piece: Piece) {
        self.board.set_at(sq, piece);
        self.zobrist.toggle_piece_on_square(sq, piece);
    }

    fn remove_at(&mut self, sq: Square) -> Piece {
        let removed_piece = self.board.piece_at(sq).unwrap();
        self.board.remove_at(sq);
        self.zobrist.toggle_piece_on_square(sq, removed_piece);
        removed_piece
    }

    pub fn make_move(&mut self, mv: &Move) {
        let from = mv.src;
        let to = mv.dst;
        let player = self.player;
        let other_player = player.other();

        let moved_piece = self.board.piece_at(from).unwrap();
        let maybe_captured_piece = self.board.piece_at(to);

        // Capture the irreversible aspects of the position so that they can be restored
        // if we undo this move.
        let history = History {
            mv: *mv,
            captured: maybe_captured_piece,
            white_castle_rights: self.white_castle_rights,
            black_castle_rights: self.black_castle_rights,
            en_passant_target: self.en_passant_target,
            halfmove_clock: self.halfmove_clock,
            zobrist: self.zobrist.clone(),
        };

        self.history.push(history);

        self.remove_at(from);

        if maybe_captured_piece.is_some() {
            self.remove_at(to);
        }

        if let Some(promoted_to) = mv.promotion {
            let promoted_piece = Piece::new(player, promoted_to.piece());
            self.set_at(to, promoted_piece);
        } else {
            self.set_at(to, moved_piece);
        }

        let pawn_move_direction = Direction::pawn_move_direction(player);

        // If we moved a pawn to the en passant target, this was an en passant capture, so we
        // remove the captured pawn from the board.
        if let Some(en_passant_target) = self.en_passant_target {
            if moved_piece.kind == PieceKind::Pawn && to == en_passant_target {
                // Remove the piece behind the square the pawn just moved to
                let capture_square = to.in_direction(!pawn_move_direction).unwrap();
                self.remove_at(capture_square);
            }
        }

        let new_en_passant_target = if moved_piece.kind == PieceKind::Pawn
            && squares::pawn_back_rank(player).contains(from)
            && squares::pawn_double_push_rank(player).contains(to)
        {
            let to_square = Squares::from_square(to);
            let en_passant_attacker_squares = to_square.west() | to_square.east();
            let enemy_pawns = self.board.player_pieces(other_player).pawns;
            let en_passant_can_happen = (en_passant_attacker_squares & enemy_pawns).any();

            if en_passant_can_happen {
                Some(from.in_direction(pawn_move_direction).unwrap())
            } else {
                None
            }
        } else {
            None
        };

        self.zobrist
            .set_en_passant(self.en_passant_target, new_en_passant_target);
        self.en_passant_target = new_en_passant_target;

        // If we just moved a king from its start square, we may have castled.
        //
        // PERF: Here, we figure out if the move was castling. It may be more performant to
        // tell this function that the move was castling, but it loses the cleanliness of
        // just telling the board the start and end destination for the piece.
        //
        // TODO: Collapse the queenside and kingside code paths into one here
        if moved_piece.kind == PieceKind::King && from == squares::king_start(player) {
            // We're castling!
            if to == squares::kingside_castle_dest(player) {
                let rook_remove_square = squares::kingside_rook_start(player);
                let rook_add_square = squares::kingside_rook_castle_end(player);

                let rook = self.remove_at(rook_remove_square);
                self.set_at(rook_add_square, rook);
            } else if to == squares::queenside_castle_dest(player) {
                let rook_remove_square = squares::queenside_rook_start(player);
                let rook_add_square = squares::queenside_rook_castle_end(player);

                let rook = self.remove_at(rook_remove_square);
                self.set_at(rook_add_square, rook);
            }
        }

        let (castle_rights, other_player_castle_rights) = match player {
            Player::White => (&mut self.white_castle_rights, &mut self.black_castle_rights),
            Player::Black => (&mut self.black_castle_rights, &mut self.white_castle_rights),
        };

        if moved_piece.kind == PieceKind::King && from == squares::king_start(player) {
            if castle_rights.king_side {
                castle_rights.remove_kingside_rights();
                self.zobrist
                    .toggle_castle_rights(player, CastleRightsSide::Kingside);
            }

            if castle_rights.queen_side {
                castle_rights.remove_queenside_rights();
                self.zobrist
                    .toggle_castle_rights(player, CastleRightsSide::Queenside);
            }
        } else if moved_piece.kind == PieceKind::Rook {
            if from == squares::kingside_rook_start(player) && castle_rights.king_side {
                self.zobrist
                    .toggle_castle_rights(player, CastleRightsSide::Kingside);
                castle_rights.remove_kingside_rights();
            } else if from == squares::queenside_rook_start(player) && castle_rights.queen_side {
                castle_rights.remove_queenside_rights();
                self.zobrist
                    .toggle_castle_rights(player, CastleRightsSide::Queenside);
            }
        }

        if maybe_captured_piece.is_some() {
            if to == squares::kingside_rook_start(other_player)
                && other_player_castle_rights.king_side
            {
                other_player_castle_rights.remove_kingside_rights();
                self.zobrist
                    .toggle_castle_rights(other_player, CastleRightsSide::Kingside);
            } else if to == squares::queenside_rook_start(other_player)
                && other_player_castle_rights.queen_side
            {
                other_player_castle_rights.remove_queenside_rights();
                self.zobrist
                    .toggle_castle_rights(other_player, CastleRightsSide::Queenside);
            }
        }

        let should_reset_halfmove_clock =
            maybe_captured_piece.is_some() || moved_piece.kind == PieceKind::Pawn;

        if should_reset_halfmove_clock {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        self.plies += 1;

        self.player = other_player;
        self.zobrist.toggle_side_to_play();
    }

    pub fn undo_move(&mut self) {
        let history = self.history.pop().unwrap();
        let mv = history.mv;
        let from = mv.src;
        let to = mv.dst;

        // The player that made this move is the one whose turn it was before
        // we start undoing the move.
        let player = self.player.other();
        let other_player = self.player;

        self.player = player;
        self.zobrist.toggle_side_to_play();

        let moved_piece = self.board.piece_at(to).unwrap();

        self.plies -= 1;

        self.halfmove_clock = history.halfmove_clock;

        // If either player lost their castle rights during the move, we restore them
        if !self.white_castle_rights.king_side && history.white_castle_rights.king_side {
            self.zobrist
                .toggle_castle_rights(Player::White, CastleRightsSide::Kingside);
        }
        if !self.white_castle_rights.queen_side && history.white_castle_rights.queen_side {
            self.zobrist
                .toggle_castle_rights(Player::White, CastleRightsSide::Queenside);
        }
        if !self.black_castle_rights.king_side && history.black_castle_rights.king_side {
            self.zobrist
                .toggle_castle_rights(Player::Black, CastleRightsSide::Kingside);
        }
        if !self.black_castle_rights.queen_side && history.black_castle_rights.queen_side {
            self.zobrist
                .toggle_castle_rights(Player::Black, CastleRightsSide::Queenside);
        }

        self.white_castle_rights = history.white_castle_rights;
        self.black_castle_rights = history.black_castle_rights;

        // Undo castling, if we castled
        if moved_piece.kind == PieceKind::King && from == squares::king_start(player) {
            let our_rook = Piece::new(player, PieceKind::Rook);

            if to == squares::kingside_castle_dest(player) {
                let rook_removed_square = squares::kingside_rook_start(player);
                let rook_added_square = squares::kingside_rook_castle_end(player);

                self.remove_at(rook_added_square);
                self.set_at(rook_removed_square, our_rook);
            } else if to == squares::queenside_castle_dest(player) {
                let rook_removed_square = squares::queenside_rook_start(player);
                let rook_added_square = squares::queenside_rook_castle_end(player);

                self.remove_at(rook_added_square);
                self.set_at(rook_removed_square, our_rook);
            }
        }

        // Replace the pawn taken by en-passant capture
        if let Some(en_passant_target) = history.en_passant_target {
            if moved_piece.kind == PieceKind::Pawn && to == en_passant_target {
                let capture_square = to
                    .in_direction(!Direction::pawn_move_direction(player))
                    .unwrap();
                self.set_at(capture_square, Piece::new(other_player, PieceKind::Pawn));
            }
        }

        let en_passant_target_before_undo = self.en_passant_target;
        self.en_passant_target = history.en_passant_target;

        self.zobrist
            .set_en_passant(en_passant_target_before_undo, history.en_passant_target);

        let moved_piece = self.board.piece_at(to).unwrap();
        self.remove_at(to);

        if let Some(captured_piece) = history.captured {
            self.set_at(to, captured_piece);
        }

        if mv.promotion.is_some() {
            self.set_at(from, Piece::new(player, PieceKind::Pawn));
        } else {
            self.set_at(from, moved_piece);
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
