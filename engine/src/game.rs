use crate::eval;
use crate::eval::Eval;
use chess::board::Board;
use chess::direction::Direction;
use chess::game::{CastleRights, Game};
use chess::movegen::MoveTypes;
use chess::moves::Move;
use chess::piece::{Piece, PieceKind};
use chess::player::Player;
use chess::square::{squares, Square};
use chess::zobrist::ZobristHash;
use color_eyre::Result;

#[derive(Debug, Clone)]
struct History {
    pub eval: Eval,
}

#[derive(Debug, Clone)]
pub struct EngineGame {
    pub game: Game,
    pub eval: Eval,
    history: Vec<History>,
}

impl Default for EngineGame {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineGame {
    #[must_use]
    pub fn new() -> Self {
        let game = Game::new();
        let eval = eval::eval(&game);

        Self {
            game,
            eval,
            history: Vec::new(),
        }
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
        let game = Game::from_state(
            board,
            player,
            white_castle_rights,
            black_castle_rights,
            en_passant_target,
            halfmove_clock,
            plies,
        );
        let eval = eval::eval(&game);

        Self {
            game,
            eval,
            history: Vec::new(),
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self> {
        let game = Game::from_fen(fen)?;
        let eval = eval::eval(&game);

        Ok(Self {
            game,
            eval,
            history: Vec::new(),
        })
    }

    #[must_use]
    pub fn moves(&self) -> Vec<Move> {
        self.game.moves()
    }

    #[must_use]
    pub fn moves_with_type(&self, move_types: &MoveTypes) -> Vec<Move> {
        self.game.moves_with_type(move_types)
    }

    #[must_use]
    pub fn is_stalemate_by_fifty_move_rule(&self) -> bool {
        self.game.is_stalemate_by_fifty_move_rule()
    }

    #[must_use]
    pub fn is_repeated_position(&self) -> bool {
        self.game.is_repeated_position()
    }

    #[must_use]
    pub fn is_stalemate_by_repetition(&self) -> bool {
        self.game.is_stalemate_by_repetition()
    }

    #[inline(always)]
    #[must_use]
    pub fn is_king_in_check(&self) -> bool {
        self.game.board.king_in_check(self.game.player)
    }

    #[inline(always)]
    #[must_use]
    pub fn player(&self) -> Player {
        self.game.player
    }

    #[inline(always)]
    #[must_use]
    pub fn zobrist(&self) -> ZobristHash {
        self.game.zobrist.clone()
    }

    fn set_at(&mut self, sq: Square, piece: Piece) {
        self.eval += Eval(piece.value());
        self.eval += eval::piece_square_tables::piece_contribution(sq, piece);
    }

    fn remove_at(&mut self, sq: Square) -> Piece {
        let removed_piece = self.game.board.piece_at(sq).unwrap();

        self.eval -= Eval(removed_piece.value());
        self.eval -= eval::piece_square_tables::piece_contribution(sq, removed_piece);
        removed_piece
    }

    pub fn make_move(&mut self, mv: &Move) {
        let from = mv.src;
        let to = mv.dst;

        let player = self.game.player;

        let moved_piece = self.game.board.piece_at(from).unwrap();
        let maybe_captured_piece = self.game.board.piece_at(to);

        // Capture the irreversible aspects of the position so that they can be restored
        // if we undo this move.
        let history = History { eval: self.eval };
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
        if let Some(en_passant_target) = self.game.en_passant_target {
            if moved_piece.kind == PieceKind::Pawn && to == en_passant_target {
                // Remove the piece behind the square the pawn just moved to
                let capture_square = to.in_direction(!pawn_move_direction);
                self.remove_at(capture_square);
            }
        }

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

        self.game.make_move(mv);
    }

    pub fn undo_move(&mut self) {
        let history = self.history.pop().unwrap();
        self.eval = history.eval;

        self.game.undo_move();
    }
}
