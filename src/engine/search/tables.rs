use crate::chess::moves::Move;
use crate::chess::player::Player;
use crate::chess::square::Square;
use crate::engine::search::{move_ordering, MAX_SEARCH_DEPTH_SIZE};

pub struct KillersTable([[Option<Move>; 2]; MAX_SEARCH_DEPTH_SIZE]);

impl KillersTable {
    pub const fn new() -> Self {
        Self([[None; 2]; MAX_SEARCH_DEPTH_SIZE])
    }

    pub fn get_0(&self, plies: u8) -> Option<Move> {
        let plies = plies as usize;
        self.0[plies][0]
    }

    pub fn get_1(&self, plies: u8) -> Option<Move> {
        let plies = plies as usize;
        self.0[plies][1]
    }

    pub fn try_push(&mut self, plies: u8, mv: Move) {
        let plies = plies as usize;

        let killer_1 = self.0[plies][0];

        if Some(mv) != killer_1 {
            self.0[plies][1] = killer_1;
            self.0[plies][0] = Some(mv);
        }
    }
}

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
