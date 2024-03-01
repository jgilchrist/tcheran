use crate::chess::moves::Move;
use crate::chess::player::Player;
use crate::chess::square::Square;
use crate::engine::search::move_ordering;

pub struct HistoryTable([[[i32; Square::N]; Square::N]; Player::N]);

impl HistoryTable {
    pub const fn new() -> Self {
        Self([[[0; Square::N]; Square::N]; Player::N])
    }

    fn bonus(depth: u8) -> i32 {
        let depthi32 = i32::from(depth);

        depthi32 * depthi32
    }

    pub fn get(&self, player: Player, mv: Move) -> i32 {
        self.0[player.array_idx()][mv.src.array_idx()][mv.dst.array_idx()]
    }

    pub fn add_bonus_for(&mut self, player: Player, mv: Move, depth: u8) {
        let bonus = Self::bonus(depth);
        let existing_score = self.get(player, mv);
        let new_score = std::cmp::min(existing_score + bonus, move_ordering::HISTORY_MAX_SCORE);

        self.0[player.array_idx()][mv.src.array_idx()][mv.dst.array_idx()] = new_score;
    }

    pub fn add_malus_for(&mut self, player: Player, mv: Move, depth: u8) {
        let malus = Self::bonus(depth);
        let existing_score = self.get(player, mv);
        let new_score = std::cmp::max(existing_score - malus, 0);

        self.0[player.array_idx()][mv.src.array_idx()][mv.dst.array_idx()] = new_score;
    }

    pub fn decay(&mut self, decay_factor: i32) {
        for from_square in 0..Square::N {
            for to_square in 0..Square::N {
                for player in 0..Player::N {
                    self.0[player][from_square][to_square] /= decay_factor;
                }
            }
        }
    }
}
