use crate::bitboard::bitboards;
use crate::piece::Piece;
use crate::square::squares;
use crate::zobrist::ZobristHash;
use crate::{
    board::Board, direction::Direction, fen, movegen::generate_moves, moves::Move,
    piece::PieceKind, player::Player, square::Square, zobrist,
};
use color_eyre::Result;

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

    #[inline(always)]
    pub fn array_idx(&self) -> usize {
        *self as usize
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CastleRights {
    pub king_side: bool,
    pub queen_side: bool,
}

impl CastleRights {
    pub const fn can_castle(&self) -> bool {
        self.king_side || self.queen_side
    }

    pub const fn none() -> Self {
        Self {
            king_side: false,
            queen_side: false,
        }
    }

    pub const fn without_kingside(&self) -> Self {
        Self {
            king_side: false,
            queen_side: self.queen_side,
        }
    }

    pub const fn without_queenside(&self) -> Self {
        Self {
            king_side: self.king_side,
            queen_side: false,
        }
    }

    pub fn can_castle_to_side(&self, side: CastleRightsSide) -> bool {
        match side {
            CastleRightsSide::Kingside => self.king_side,
            CastleRightsSide::Queenside => self.queen_side,
        }
    }

    pub fn remove_rights(&mut self, side: CastleRightsSide) {
        match side {
            CastleRightsSide::Kingside => self.king_side = false,
            CastleRightsSide::Queenside => self.queen_side = false,
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

    pub fn to_fen(&self) -> String {
        fen::write(self)
    }

    pub fn turn(&self) -> u32 {
        self.plies / 2 + 1
    }

    pub fn moves(&self) -> Vec<Move> {
        generate_moves::<true>(self)
    }

    pub fn loud_moves(&self) -> Vec<Move> {
        generate_moves::<false>(self)
    }

    pub fn is_stalemate_by_fifty_move_rule(&self) -> bool {
        if self.halfmove_clock >= 100 {
            return self.moves().first().is_some();
        }

        false
    }

    pub fn is_repeated_position(&self) -> bool {
        self.history
            .iter()
            .rev()
            .take(self.halfmove_clock as usize)
            .any(|h| h.zobrist == self.zobrist)
    }

    pub fn is_stalemate_by_repetition(&self) -> bool {
        let mut count = 0;

        for seen_state in self.history.iter().rev().take(self.halfmove_clock as usize) {
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

    pub fn is_stalemate_by_insufficient_material(&self) -> bool {
        let all_pieces = self.board.white_pieces.all() | self.board.black_pieces.all();

        match all_pieces.count() {
            // King vs king is always a draw
            2 => true,

            // If the sole remaining non-king piece on the board is a knight or bishop,
            // it's a draw
            3 => (self.board.white_pieces.knights
                | self.board.black_pieces.knights
                | self.board.white_pieces.bishops
                | self.board.black_pieces.bishops)
                .any(),

            4 => {
                let player_pieces = self.board.player_pieces(self.player);
                let knights = self.board.white_pieces.knights | self.board.black_pieces.knights;
                let bishops = self.board.white_pieces.bishops | self.board.black_pieces.bishops;
                let kings = self.board.white_pieces.king | self.board.black_pieces.king;

                let one_piece_each = player_pieces.all().count() == 2;

                let knight_count = knights.count();
                let bishop_count = bishops.count();
                let king_in_corner = (kings & bitboards::CORNERS).any();
                let king_on_edge = (kings & bitboards::EDGES).any();

                // This logic is from Carp
                (knight_count == 2 && !king_on_edge) ||
                    (bishop_count == 2 && (
                        (bishops & bitboards::LIGHT_SQUARES).count() != 1 ||
                            (one_piece_each && !king_in_corner))) ||
                    (knight_count == 1 && bishop_count == 1 && one_piece_each && !king_in_corner)
            }
            _ => false,
        }
    }

    #[inline(always)]
    pub fn is_king_in_check(&self) -> bool {
        self.board.king_in_check(self.player)
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

    // PERF: Make fetching the castle rights for a player more efficient
    fn try_remove_castle_rights(&mut self, player: Player, castle_rights_side: CastleRightsSide) {
        let castle_rights = match player {
            Player::White => &mut self.white_castle_rights,
            Player::Black => &mut self.black_castle_rights,
        };

        // We don't want to modify anything if the castle rights on this side were already lost
        if !castle_rights.can_castle_to_side(castle_rights_side) {
            return;
        }

        castle_rights.remove_rights(castle_rights_side);

        self.zobrist
            .toggle_castle_rights(player, castle_rights_side);
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
                let capture_square = to.in_direction(!pawn_move_direction);
                self.remove_at(capture_square);
            }
        }

        let new_en_passant_target = if moved_piece.kind == PieceKind::Pawn
            && bitboards::pawn_back_rank(player).contains(from)
            && bitboards::pawn_double_push_rank(player).contains(to)
        {
            let en_passant_attacker_squares = to.0.west() | to.0.east();
            let enemy_pawns = self.board.player_pieces(other_player).pawns;
            let en_passant_can_happen = (en_passant_attacker_squares & enemy_pawns).any();

            if en_passant_can_happen {
                Some(from.in_direction(pawn_move_direction))
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

        // Check if we lost castle rights.
        // If we moved the king, we lose all rights to castle.
        // If we moved one of our rooks, we lose rights to castle on that side.
        if moved_piece.kind == PieceKind::King && from == squares::king_start(player) {
            self.try_remove_castle_rights(player, CastleRightsSide::Kingside);
            self.try_remove_castle_rights(player, CastleRightsSide::Queenside);
        } else if moved_piece.kind == PieceKind::Rook {
            if from == squares::kingside_rook_start(player) {
                self.try_remove_castle_rights(player, CastleRightsSide::Kingside);
            } else if from == squares::queenside_rook_start(player) {
                self.try_remove_castle_rights(player, CastleRightsSide::Queenside);
            }
        }

        // Check if we removed our enemy's ability to castle, i.e. if we took one of their rooks
        if maybe_captured_piece.is_some() {
            if to == squares::kingside_rook_start(other_player) {
                self.try_remove_castle_rights(other_player, CastleRightsSide::Kingside);
            } else if to == squares::queenside_rook_start(other_player) {
                self.try_remove_castle_rights(other_player, CastleRightsSide::Queenside);
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
            if to == squares::kingside_castle_dest(player) {
                let rook_removed_square = squares::kingside_rook_start(player);
                let rook_added_square = squares::kingside_rook_castle_end(player);

                let rook = self.remove_at(rook_added_square);
                self.set_at(rook_removed_square, rook);
            } else if to == squares::queenside_castle_dest(player) {
                let rook_removed_square = squares::queenside_rook_start(player);
                let rook_added_square = squares::queenside_rook_castle_end(player);

                let rook = self.remove_at(rook_added_square);
                self.set_at(rook_removed_square, rook);
            }
        }

        // Replace the pawn taken by en-passant capture
        if let Some(en_passant_target) = history.en_passant_target {
            if moved_piece.kind == PieceKind::Pawn && to == en_passant_target {
                let capture_square = to.in_direction(!Direction::pawn_move_direction(player));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_by_insufficient_material() {
        // Knight vs Bishop mate
        assert!(!Game::from_fen("5b1K/5k1N/8/8/8/8/8/8 b - - 1 1")
            .unwrap()
            .is_stalemate_by_insufficient_material());

        // Bishop vs Knight - draw
        assert!(Game::from_fen("8/8/3k4/4n3/8/2KB4/8/8 w - - 0 1")
            .unwrap()
            .is_stalemate_by_insufficient_material());

        // Rook vs Knight mate
        assert!(!Game::from_fen("8/8/4k3/4n3/8/2KR4/8/8 w - - 0 1")
            .unwrap()
            .is_stalemate_by_insufficient_material());
    }
}
