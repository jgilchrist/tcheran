use crate::chess::bitboard::{bitboards, Bitboard};
use crate::chess::board::Board;
use crate::chess::player::Player;
use crate::chess::square::{Rank, Square};
use crate::engine::eval::PhasedEval;

static mut ENEMY_PASSED_PAWN_MASKS: [[Bitboard; Square::N]; Player::N] =
    [[Bitboard::EMPTY; Square::N]; Player::N];

static mut OWN_PAWN_BLOCKER_MASKS: [[Bitboard; Square::N]; Player::N] =
    [[Bitboard::EMPTY; Square::N]; Player::N];

#[rustfmt::skip]
const PASSED_PAWN_BONUSES: [PhasedEval; 8] = [
    PhasedEval::new(0, 0),
    PhasedEval::new(74, 260),
    PhasedEval::new(53, 176),
    PhasedEval::new(19, 84),
    PhasedEval::new(-15, 43),
    PhasedEval::new(-15, 12),
    PhasedEval::new(-1, 5),
    PhasedEval::new(0, 0),
];

struct PassedPawnMasks {
    // A pawn is not passed if any enemy pawns are in front of it or could capture
    // if it were to advance
    enemy_pawns_mask: Bitboard,

    // A pawn is not passed if one of our own pawns is in front of it
    own_pawns_blockers_mask: Bitboard,
}

impl PassedPawnMasks {
    pub fn empty() -> Self {
        Self {
            enemy_pawns_mask: Bitboard::EMPTY,
            own_pawns_blockers_mask: Bitboard::EMPTY,
        }
    }
}

fn generate_passed_pawn_masks(player: Player, square: Square) -> PassedPawnMasks {
    let pawn_back_rank = bitboards::pawn_back_rank(player);
    let their_pawn_back_rank = bitboards::pawn_back_rank(player.other());
    let our_bank_rank = pawn_back_rank.backward(player);

    // Pawns cannot be on our back rank
    if (square.bb() & our_bank_rank).any() {
        return PassedPawnMasks::empty();
    }

    // Pawns on their pawn rank cannot be blocked, as pawns cannot be on their back rank
    if (square.bb() & their_pawn_back_rank).any() {
        return PassedPawnMasks::empty();
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

    let enemy_pawns_mask = relevant_files & relevant_ranks;
    let own_pawns_blockers_mask = file & relevant_ranks;

    PassedPawnMasks {
        enemy_pawns_mask,
        own_pawns_blockers_mask,
    }
}

fn enemy_passed_pawn_mask(player: Player, square: Square) -> Bitboard {
    *unsafe {
        ENEMY_PASSED_PAWN_MASKS
            .get_unchecked(player.array_idx())
            .get_unchecked(square.array_idx())
    }
}

fn our_pawn_blocker_mask(player: Player, square: Square) -> Bitboard {
    *unsafe {
        OWN_PAWN_BLOCKER_MASKS
            .get_unchecked(player.array_idx())
            .get_unchecked(square.array_idx())
    }
}

pub fn eval(board: &Board) -> PhasedEval {
    let white_bonus = calculate_passed_pawn_bonus(board, Player::White);
    let black_bonus = calculate_passed_pawn_bonus(board, Player::Black);

    white_bonus - black_bonus
}

fn calculate_passed_pawn_bonus(board: &Board, player: Player) -> PhasedEval {
    let mut bonus = PhasedEval::ZERO;

    let our_pawns = board.pieces(player).pawns();
    let their_pawns = board.pieces(player.other()).pawns();

    let their_back_rank = match player {
        Player::White => Rank::R8,
        Player::Black => Rank::R1,
    };

    for pawn in our_pawns {
        if (enemy_passed_pawn_mask(player, pawn) & their_pawns).is_empty()
            && (our_pawn_blocker_mask(player, pawn) & our_pawns).is_empty()
        {
            let distance_from_promotion = their_back_rank
                .array_idx()
                .abs_diff(pawn.rank().array_idx());
            bonus += PASSED_PAWN_BONUSES[distance_from_promotion];
        }
    }

    bonus
}

pub fn init() {
    for player in [Player::White, Player::Black] {
        for square in Bitboard::FULL {
            let masks = generate_passed_pawn_masks(player, square);
            unsafe {
                ENEMY_PASSED_PAWN_MASKS[player.array_idx()][square.array_idx()] =
                    masks.enemy_pawns_mask;
                OWN_PAWN_BLOCKER_MASKS[player.array_idx()][square.array_idx()] =
                    masks.own_pawns_blockers_mask;
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
