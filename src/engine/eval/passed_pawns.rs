use crate::chess::bitboard::{bitboards, Bitboard};
use crate::chess::board::Board;
use crate::chess::player::Player;
use crate::chess::square::{Rank, Square};
use crate::engine::eval::PhasedEval;

static mut PASSED_PAWN_MASKS: [[Bitboard; Square::N]; Player::N] =
    [[Bitboard::EMPTY; Square::N]; Player::N];

#[rustfmt::skip]
const PASSED_PAWN_BONUSES: [PhasedEval; 8] = [
    PhasedEval::new(0, 0),
    PhasedEval::new(260, 280),
    PhasedEval::new(170, 180),
    PhasedEval::new(60, 70),
    PhasedEval::new(15, 40),
    PhasedEval::new(15, 30),
    PhasedEval::new(10, 30),
    PhasedEval::new(0, 0)
];

fn generate_passed_pawn_mask(player: Player, square: Square) -> Bitboard {
    let pawn_back_rank = bitboards::pawn_back_rank(player);
    let their_pawn_back_rank = bitboards::pawn_back_rank(player.other());
    let our_bank_rank = pawn_back_rank.backward(player);

    // Pawns cannot be on our back rank
    if (square.bb() & our_bank_rank).any() {
        return Bitboard::EMPTY;
    }

    // Pawns on their pawn rank cannot be blocked, as pawns cannot be on their back rank
    if (square.bb() & their_pawn_back_rank).any() {
        return Bitboard::EMPTY;
    }

    let file = square.file().bitboard();
    let file_left = file.west();
    let file_right = file.east();

    let relevant_files = file_left | file | file_right;

    let rank = square.rank();
    let mut relevant_ranks = Bitboard::FULL;

    let back_rank_idx = match player {
        Player::White => Rank::R1,
        Player::Black => Rank::R8,
    };

    let distance_from_back_rank = back_rank_idx.array_idx().abs_diff(rank.array_idx());

    for _ in 0..=distance_from_back_rank {
        relevant_ranks = relevant_ranks.forward(player);
    }

    relevant_files & relevant_ranks
}

fn passed_pawn_mask(player: Player, square: Square) -> Bitboard {
    *unsafe {
        PASSED_PAWN_MASKS
            .get_unchecked(player.array_idx())
            .get_unchecked(square.array_idx())
    }
}

// perf: We don't need to recalculate this unless a pawn moves or is taken, so this can be part
// of the incremental eval fields
pub fn eval(board: &Board) -> PhasedEval {
    let white_pawns = board.player_pieces(Player::White).pawns();
    let black_pawns = board.player_pieces(Player::Black).pawns();

    let mut white_bonus = PhasedEval::ZERO;
    let mut black_bonus = PhasedEval::ZERO;

    // White
    for pawn in white_pawns {
        if (passed_pawn_mask(Player::White, pawn) & black_pawns).is_empty() {
            let distance_from_promotion = Rank::R8.array_idx().abs_diff(pawn.rank().array_idx());
            white_bonus += PASSED_PAWN_BONUSES[distance_from_promotion];
        }
    }

    // Black
    for pawn in black_pawns {
        if (passed_pawn_mask(Player::Black, pawn) & white_pawns).is_empty() {
            let distance_from_promotion = Rank::R1.array_idx().abs_diff(pawn.rank().array_idx());
            black_bonus += PASSED_PAWN_BONUSES[distance_from_promotion];
        }
    }

    white_bonus - black_bonus
}

pub fn init() {
    for player in [Player::White, Player::Black] {
        for square in Bitboard::FULL {
            let mask = generate_passed_pawn_mask(player, square);
            unsafe {
                PASSED_PAWN_MASKS[player.array_idx()][square.array_idx()] = mask;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::game::Game;
    use crate::engine::eval::WhiteEval;

    #[test]
    pub fn test_bonus_for_passed_pawn_white() {
        crate::init();

        let game = Game::from_fen("k7/4P3/8/8/8/8/8/K7 w - - 0 1").unwrap();

        let passed_pawn_eval = eval(&game.board);
        assert_eq!(passed_pawn_eval.midgame(), WhiteEval(260));
        assert_eq!(passed_pawn_eval.endgame(), WhiteEval(280));
    }

    #[test]
    pub fn test_bonus_for_passed_pawn_black() {
        crate::init();

        let game = Game::from_fen("k7/8/8/8/8/8/4p3/K7 w - - 0 1").unwrap();

        let passed_pawn_eval = eval(&game.board);
        assert_eq!(passed_pawn_eval.midgame(), WhiteEval(-260));
        assert_eq!(passed_pawn_eval.endgame(), WhiteEval(-280));
    }
}
