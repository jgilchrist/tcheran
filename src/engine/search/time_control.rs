use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::chess::game::Game;
use crate::chess::player::Player;
use crate::engine::options::EngineOptions;
use crate::engine::search::{params, TimeControl};

pub struct TimeStrategy {
    time_control: TimeControl,
    started_at: Instant,

    soft_stop: Duration,
    hard_stop: Duration,

    next_check_at: u64,

    force_stop: Arc<AtomicBool>,
}

pub struct Control {
    force_stop: Arc<AtomicBool>,
}

impl Control {
    pub fn stop(&self) {
        self.force_stop.store(true, Ordering::Relaxed);
    }
}

impl TimeStrategy {
    pub fn new(
        game: &Game,
        time_control: &TimeControl,
        options: &EngineOptions,
    ) -> (Self, Control) {
        let now = Instant::now();
        let move_overhead = Duration::from_millis(options.move_overhead as u64);

        let mut soft_stop = Duration::default();
        let mut hard_stop = Duration::default();

        match time_control {
            TimeControl::Infinite => {}
            TimeControl::ExactTime(move_time) => {
                soft_stop = *move_time;
                hard_stop = *move_time;
            }
            TimeControl::Clocks(ref clocks) => {
                let (time_remaining, increment) = match game.player {
                    Player::White => (clocks.white_clock, clocks.white_increment),
                    Player::Black => (clocks.black_clock, clocks.black_increment),
                };
                let increment = increment.unwrap_or_default();

                let mut time_remaining = time_remaining.unwrap_or_default();

                time_remaining = time_remaining
                    .saturating_sub(move_overhead)
                    .max(move_overhead);

                let max_time_per_move = time_remaining.mul_f32(params::MAX_TIME_PER_MOVE);

                let base_time = if let Some(moves_to_go) = clocks.moves_to_go {
                    // Try to use a roughly even amount of time per move
                    time_remaining / moves_to_go
                } else {
                    time_remaining.mul_f32(params::BASE_TIME_PER_MOVE)
                } + increment.mul_f32(params::INCREMENT_TO_USE);

                soft_stop = std::cmp::min(
                    base_time.mul_f32(params::SOFT_TIME_MULTIPLIER),
                    max_time_per_move,
                );

                hard_stop = std::cmp::min(
                    base_time.mul_f32(params::HARD_TIME_MULTIPLIER),
                    max_time_per_move,
                );
            }
        }

        let force_stop = Arc::new(AtomicBool::new(false));

        let control = Control {
            force_stop: force_stop.clone(),
        };

        let time_strategy = Self {
            time_control: time_control.clone(),
            started_at: now,

            soft_stop,
            hard_stop,

            next_check_at: params::CHECK_TERMINATION_NODE_FREQUENCY,

            force_stop,
        };

        (time_strategy, control)
    }

    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    pub fn should_start_new_search(&self, depth: u8) -> bool {
        if depth == 1 {
            return true;
        }

        if self.is_force_stopped() {
            return false;
        }

        match self.time_control {
            TimeControl::Clocks(_) => self.elapsed() < self.soft_stop,
            TimeControl::ExactTime(time) => self.elapsed() < time,
            TimeControl::Infinite => true,
        }
    }

    pub fn should_stop(&mut self, nodes_visited: u64) -> bool {
        if nodes_visited < self.next_check_at {
            return false;
        }

        if self.is_force_stopped() {
            return true;
        }

        self.next_check_at = nodes_visited + params::CHECK_TERMINATION_NODE_FREQUENCY;

        match self.time_control {
            TimeControl::Clocks(_) => self.elapsed() > self.hard_stop,
            TimeControl::ExactTime(time) => self.elapsed() > time,
            TimeControl::Infinite => false,
        }
    }

    fn is_force_stopped(&self) -> bool {
        self.force_stop.load(Ordering::Relaxed)
    }
}
