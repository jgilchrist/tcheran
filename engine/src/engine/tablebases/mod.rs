use crate::chess::game::Game;
use crate::chess::moves::{Move, MoveListExt};
use crate::chess::piece::PromotionPieceKind;
use crate::chess::player::Player;
use crate::chess::square::Square;
use std::ffi::{c_uint, CString};
use std::ptr;

#[allow(
    unused,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    clippy::unreadable_literal
)]
mod bindings;

pub enum Wdl {
    Win,
    Draw,
    Loss,
}

pub struct Tablebase {
    is_enabled: bool,
}

impl Tablebase {
    pub fn new() -> Self {
        Self { is_enabled: false }
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "n_men will be at most 7 as these are the largest syzygy tablebases"
    )]
    pub fn n_men(&self) -> u8 {
        if !self.is_enabled {
            return 0;
        }

        unsafe { bindings::TB_LARGEST as u8 }
    }

    pub fn set_paths(&mut self, path: &str) {
        let path = CString::new(path).unwrap();
        let was_set = unsafe { bindings::tb_init(path.as_ptr()) };
        let n_men = unsafe { bindings::TB_LARGEST as usize };

        assert!(
            was_set && n_men != 0,
            "Invalid tablebase path: {}",
            path.to_str().unwrap_or_default()
        );

        self.is_enabled = true;
    }

    pub fn wdl(&self, game: &Game) -> Option<Wdl> {
        if !self.is_enabled {
            return None;
        }

        unsafe {
            let wdl = bindings::tb_probe_wdl(
                game.board.occupancy_for(Player::White).as_u64(),
                game.board.occupancy_for(Player::Black).as_u64(),
                game.board.all_kings().as_u64(),
                game.board.all_queens().as_u64(),
                game.board.all_rooks().as_u64(),
                game.board.all_bishops().as_u64(),
                game.board.all_knights().as_u64(),
                game.board.all_pawns().as_u64(),
                0,
                0,
                0,
                game.player == Player::White,
            );

            Self::to_wdl(wdl)
        }
    }

    #[rustfmt::skip]
    pub fn best_move(&self, game: &Game) -> Option<Move> {
        if !self.is_enabled {
            return None;
        }

        unsafe {
            let result = bindings::tb_probe_root(
                game.board.occupancy_for(Player::White).as_u64(),
                game.board.occupancy_for(Player::Black).as_u64(),
                game.board.all_kings().as_u64(),
                game.board.all_queens().as_u64(),
                game.board.all_rooks().as_u64(),
                game.board.all_bishops().as_u64(),
                game.board.all_knights().as_u64(),
                game.board.all_pawns().as_u64(),
                game.halfmove_clock,
                0,
                0,
                game.player == Player::White,
                ptr::null_mut(),
            );

            if result == bindings::TB_RESULT_FAILED {
                return None;
            }

            // let wdl_bits = result & bindings::TB_RESULT_WDL_MASK >> bindings::TB_RESULT_WDL_SHIFT;
            // let dtz_bits = (result & bindings::TB_RESULT_DTZ_MASK) >> bindings::TB_RESULT_DTZ_SHIFT;
            let from_bits =(result & bindings::TB_RESULT_FROM_MASK) >> bindings::TB_RESULT_FROM_SHIFT;
            let to_bits = (result & bindings::TB_RESULT_TO_MASK) >> bindings::TB_RESULT_TO_SHIFT;
            let promotion_bits = (result & bindings::TB_RESULT_PROMOTES_MASK) >> bindings::TB_RESULT_PROMOTES_SHIFT;

            let from = Square::from_index(from_bits as u8);
            let to = Square::from_index(to_bits as u8);

            let promotion = match promotion_bits {
                bindings::TB_PROMOTES_QUEEN => Some(PromotionPieceKind::Queen),
                bindings::TB_PROMOTES_ROOK => Some(PromotionPieceKind::Rook),
                bindings::TB_PROMOTES_BISHOP => Some(PromotionPieceKind::Bishop),
                bindings::TB_PROMOTES_KNIGHT => Some(PromotionPieceKind::Knight),
                _ => None,
            };

            let matching_move = game.moves().expect_matching(from, to, promotion);

            Some(matching_move)
        }
    }

    fn to_wdl(outcome: c_uint) -> Option<Wdl> {
        use Wdl::*;

        match outcome {
            bindings::TB_WIN => Some(Win),
            bindings::TB_LOSS => Some(Loss),
            bindings::TB_DRAW | bindings::TB_CURSED_WIN | bindings::TB_BLESSED_LOSS => Some(Draw),
            bindings::TB_RESULT_FAILED => None,
            _ => unreachable!(),
        }
    }
}
