mod pawn_structure;
pub mod piece_square_tables;
mod player_eval;
mod tapered_eval;
mod white_eval;

use crate::chess::bitboard::Bitboard;
use crate::chess::board::Board;
pub use player_eval::Eval;
pub use white_eval::WhiteEval;

use crate::chess::game::Game;
use crate::chess::piece::{Piece, PieceKind};
use crate::chess::player::Player;
use crate::chess::square::{Square, FILES};
pub use crate::engine::eval::tapered_eval::PhasedEval;

pub fn init() {
    piece_square_tables::init();
}

#[derive(Debug, Clone)]
pub struct IncrementalEvalFields {
    pub phase_value: i16,

    pub piece_square_tables: PhasedEval,

    pub doubled_pawn_files: [Bitboard; Player::N],
}

impl IncrementalEvalFields {
    pub fn set_at(&mut self, board: &Board, sq: Square, piece: Piece) {
        self.phase_value += tapered_eval::piece_phase_value_contribution(piece.kind);
        self.piece_square_tables += piece_square_tables::piece_contributions(sq, piece);

        if piece.kind == PieceKind::Pawn {
            let file = sq.file();

            let is_doubled_pawn_on_file =
                (board.pieces(piece.player).pawns() & file.bitboard()).count() > 1;

            if is_doubled_pawn_on_file {
                self.doubled_pawn_files[piece.player.array_idx()] |= file.bitboard()
            } else {
                self.doubled_pawn_files[piece.player.array_idx()] ^= file.bitboard()
            }
        }
    }

    pub fn remove_at(&mut self, board: &Board, sq: Square, piece: Piece) {
        self.phase_value -= tapered_eval::piece_phase_value_contribution(piece.kind);
        self.piece_square_tables -= piece_square_tables::piece_contributions(sq, piece);

        if piece.kind == PieceKind::Pawn {
            let file = sq.file();

            let is_doubled_pawn_on_file =
                (board.pieces(piece.player).pawns() & file.bitboard()).count() > 1;

            if is_doubled_pawn_on_file {
                self.doubled_pawn_files[piece.player.array_idx()] ^= file.bitboard()
            } else {
                self.doubled_pawn_files[piece.player.array_idx()] |= file.bitboard()
            }
        }
    }
}

impl IncrementalEvalFields {
    pub fn init(board: &Board) -> Self {
        let phase_value = tapered_eval::phase_value(board);
        let piece_square_tables = piece_square_tables::eval(board);

        let mut doubled_pawn_files_white = Bitboard::EMPTY;
        let mut doubled_pawn_files_black = Bitboard::EMPTY;

        for f in FILES {
            let has_doubled_white_pawn =
                (board.pieces(Player::White).pawns() & f.bitboard()).count() > 1;
            let has_doubled_black_pawn =
                (board.pieces(Player::Black).pawns() & f.bitboard()).count() > 1;

            if has_doubled_white_pawn {
                doubled_pawn_files_white |= f.bitboard();
            }

            if has_doubled_black_pawn {
                doubled_pawn_files_black |= f.bitboard();
            }
        }

        Self {
            phase_value,

            piece_square_tables,

            doubled_pawn_files: [doubled_pawn_files_white, doubled_pawn_files_black],
        }
    }
}

pub fn eval(game: &Game) -> Eval {
    let absolute_eval = absolute_eval(game);
    Eval::from_white_eval(absolute_eval, game.player)
}

pub fn absolute_eval(game: &Game) -> WhiteEval {
    let eval =
        game.incremental_eval.piece_square_tables + pawn_structure::eval(&game.incremental_eval);

    tapered_eval::taper(game.incremental_eval.phase_value, eval)
}

#[derive(Debug)]
pub struct EvalComponents {
    pub eval: WhiteEval,
    pub phase_value: i16,

    pub phased_piece_square: PhasedEval,
    pub piece_square: WhiteEval,
}

pub fn eval_components(game: &Game) -> EvalComponents {
    let eval = absolute_eval(game);
    let phase_value = tapered_eval::phase_value(&game.board);

    let phased_piece_square = piece_square_tables::eval(&game.board);
    let piece_square = tapered_eval::taper(phase_value, phased_piece_square);

    EvalComponents {
        eval,
        phase_value,

        phased_piece_square,
        piece_square,
    }
}
