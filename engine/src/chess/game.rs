use crate::chess::bitboard::bitboards;
use crate::chess::moves::MoveList;
use crate::chess::piece::Piece;
use crate::chess::player::ByPlayer;
use crate::chess::square::squares;
use crate::chess::zobrist::ZobristHash;
use crate::chess::{
    board::Board, fen, movegen::generate_legal_moves, moves::Move, piece::PieceKind,
    player::Player, square::Square, zobrist,
};
use crate::engine::eval::IncrementalEvalFields;

#[derive(Debug, Copy, Clone)]
pub enum CastleRightsSide {
    Kingside,
    Queenside,
}

impl CastleRightsSide {
    pub const N: usize = 2;

    #[inline(always)]
    pub const fn array_idx(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CastleRights {
    pub king_side: bool,
    pub queen_side: bool,
}

impl CastleRights {
    pub const fn none() -> Self {
        Self {
            king_side: false,
            queen_side: false,
        }
    }

    pub fn can_castle_to_side(self, side: CastleRightsSide) -> bool {
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
    pub mv: Option<Move>,
    pub captured: Option<Piece>,
    pub castle_rights: ByPlayer<CastleRights>,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u32,
    pub zobrist: ZobristHash,
    pub incremental_eval: IncrementalEvalFields,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub player: Player,
    pub board: Board,
    pub castle_rights: ByPlayer<CastleRights>,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u32,
    pub plies: u32,

    pub zobrist: ZobristHash,
    pub incremental_eval: IncrementalEvalFields,
    pub history: Vec<History>,
}

impl Game {
    pub fn new() -> Self {
        Self::from_fen(fen::START_POS).unwrap()
    }

    pub fn from_state(
        board: Board,
        player: Player,
        castle_rights: ByPlayer<CastleRights>,
        en_passant_target: Option<Square>,
        halfmove_clock: u32,
        plies: u32,
    ) -> Self {
        let incremental_eval_fields = IncrementalEvalFields::init(&board);

        let mut game = Self {
            board,
            player,
            castle_rights,
            en_passant_target,
            halfmove_clock,
            plies,

            zobrist: ZobristHash::uninit(),
            incremental_eval: incremental_eval_fields,
            history: Vec::new(),
        };

        game.zobrist = zobrist::hash(&game);
        game
    }

    pub fn from_fen(fen: &str) -> Result<Self, String> {
        fen::parse(fen)
    }

    pub fn to_fen(&self) -> String {
        fen::write(self)
    }

    pub fn turn(&self) -> u32 {
        self.plies / 2 + 1
    }

    pub fn is_stalemate_by_fifty_move_rule(&self) -> bool {
        if self.halfmove_clock >= 100 {
            let mut movelist = MoveList::new();
            generate_legal_moves(self, &mut movelist);
            return !movelist.is_empty();
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

    pub fn is_stalemate_by_insufficient_material(&self) -> bool {
        let all_pieces = self.board.occupancy();

        match all_pieces.count() {
            // King vs king is always a draw
            2 => true,

            // If the sole remaining non-king piece on the board is a knight or bishop,
            // it's a draw
            3 => (self.board.all_knights() | self.board.all_bishops()).any(),

            4 => {
                let player_pieces = self.board.occupancy_for(self.player);
                let knights = self.board.all_knights();
                let bishops = self.board.all_bishops();
                let kings = self.board.all_kings();

                let one_piece_each = player_pieces.count() == 2;

                let knight_count = knights.count();
                let bishop_count = bishops.count();
                let king_in_corner = (kings & bitboards::CORNERS).any();
                let king_on_edge = (kings & bitboards::EDGES).any();

                // This logic is from Carp
                (knight_count == 2 && !king_on_edge)
                    || (bishop_count == 2
                        && ((bishops & bitboards::LIGHT_SQUARES).count() != 1
                            || (one_piece_each && !king_in_corner)))
                    || (knight_count == 1 && bishop_count == 1 && one_piece_each && !king_in_corner)
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
        self.incremental_eval.set_at(sq, piece);
    }

    fn remove_at(&mut self, sq: Square) -> Piece {
        let removed_piece = self.board.piece_at(sq).unwrap();
        self.board.remove_at(sq);
        self.zobrist.toggle_piece_on_square(sq, removed_piece);
        self.incremental_eval.remove_at(sq, removed_piece);
        removed_piece
    }

    fn try_remove_castle_rights(&mut self, player: Player, castle_rights_side: CastleRightsSide) {
        let castle_rights = self.castle_rights.for_player_mut(player);

        // We don't want to modify anything if the castle rights on this side were already lost
        if !castle_rights.can_castle_to_side(castle_rights_side) {
            return;
        }

        castle_rights.remove_rights(castle_rights_side);

        self.zobrist
            .toggle_castle_rights(player, castle_rights_side);
    }

    // Convenience method to prevent tests from having to construct their own
    // movelist and allow them to iterate easily over the resulting list of moves
    pub fn moves(&self) -> MoveList {
        let mut movelist = MoveList::new();
        generate_legal_moves(self, &mut movelist);
        movelist
    }

    pub fn make_move(&mut self, mv: Move) {
        let from = mv.src();
        let to = mv.dst();
        let player = self.player;
        let other_player = player.other();

        let maybe_captured_piece = self.board.piece_at(to);

        // Capture the irreversible aspects of the position so that they can be restored
        // if we undo this move.
        let history = History {
            mv: Some(mv),
            captured: maybe_captured_piece,
            castle_rights: self.castle_rights.clone(),
            en_passant_target: self.en_passant_target,
            halfmove_clock: self.halfmove_clock,
            zobrist: self.zobrist.clone(),
            incremental_eval: self.incremental_eval.clone(),
        };

        self.history.push(history);

        let moved_piece = self.remove_at(from);

        if maybe_captured_piece.is_some() {
            self.remove_at(to);
        }

        if let Some(promoted_to) = mv.promotion() {
            let promoted_piece = Piece::new(player, promoted_to.piece());
            self.set_at(to, promoted_piece);
        } else {
            self.set_at(to, moved_piece);
        }

        // If we moved a pawn to the en passant target, this was an en passant capture, so we
        // remove the captured pawn from the board.
        if mv.is_en_passant() {
            // Remove the piece behind the square the pawn just moved to
            let capture_square = to.backward(player);
            self.remove_at(capture_square);
        }

        let new_en_passant_target = if moved_piece.kind == PieceKind::Pawn
            && bitboards::pawn_back_rank(player).contains(from)
            && bitboards::pawn_double_push_rank(player).contains(to)
        {
            let to_bb = to.bb();
            let en_passant_attacker_squares = to_bb.west() | to_bb.east();
            let enemy_pawns = self.board.pawns(other_player);
            let en_passant_can_happen = (en_passant_attacker_squares & enemy_pawns).any();

            if en_passant_can_happen {
                Some(from.forward(player))
            } else {
                None
            }
        } else {
            None
        };

        self.zobrist
            .set_en_passant(self.en_passant_target, new_en_passant_target);
        self.en_passant_target = new_en_passant_target;

        if mv.is_castling() {
            if let Some((rook_from, rook_to)) = squares::castle_squares(player, to) {
                let rook = self.remove_at(rook_from);
                self.set_at(rook_to, rook);
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

    pub fn make_null_move(&mut self) {
        // Capture the irreversible aspects of the position so that they can be restored
        // if we undo this move.
        let history = History {
            mv: None,
            captured: None,
            castle_rights: self.castle_rights.clone(),
            en_passant_target: self.en_passant_target,
            halfmove_clock: self.halfmove_clock,
            zobrist: self.zobrist.clone(),
            incremental_eval: self.incremental_eval.clone(),
        };

        self.history.push(history);

        self.zobrist.set_en_passant(self.en_passant_target, None);
        self.en_passant_target = None;

        self.plies += 1;

        self.player = self.player.other();
        self.zobrist.toggle_side_to_play();
    }

    pub fn undo_move(&mut self) {
        let history = self.history.pop().unwrap();
        let mv = history.mv.unwrap();
        let from = mv.src();
        let to = mv.dst();

        // The player that made this move is the one whose turn it was before
        // we start undoing the move.
        let player = self.player.other();
        let other_player = self.player;

        self.plies -= 1;
        self.player = player;
        self.zobrist = history.zobrist;
        self.halfmove_clock = history.halfmove_clock;
        self.castle_rights = history.castle_rights;
        self.en_passant_target = history.en_passant_target;
        self.incremental_eval = history.incremental_eval;

        // Undo castling, if we castled
        if mv.is_castling() {
            if let Some((rook_from, rook_to)) = squares::castle_squares(player, to) {
                self.board.remove_at(rook_to);
                self.board
                    .set_at(rook_from, Piece::new(player, PieceKind::Rook));
            }
        }

        // Replace the pawn taken by en-passant capture
        if mv.is_en_passant() {
            let capture_square = to.backward(player);

            self.board
                .set_at(capture_square, Piece::new(other_player, PieceKind::Pawn));
        }

        let moved_piece = self.board.piece_at(to).unwrap();
        self.board.remove_at(to);

        if let Some(captured_piece) = history.captured {
            self.board.set_at(to, captured_piece);
        }

        if mv.promotion().is_some() {
            self.board.set_at(from, Piece::new(player, PieceKind::Pawn));
        } else {
            self.board.set_at(from, moved_piece);
        }
    }

    pub fn undo_null_move(&mut self) {
        let history = self.history.pop().unwrap();
        assert!(history.mv.is_none());

        self.plies -= 1;
        self.player = self.player.other();
        self.zobrist = history.zobrist;
        self.en_passant_target = history.en_passant_target;
        self.halfmove_clock = history.halfmove_clock;
        self.incremental_eval = history.incremental_eval;
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
        crate::init();

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
