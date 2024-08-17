use crate::chess::game::Game;
use crate::chess::player::Player;
use crate::engine::options::EngineOptions;
use crate::engine::search::{params, Clocks, TimeControl};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct TimeStrategy {
    started_at: Instant,
    stop_searching_at: Option<Instant>,

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

        let stop_searching_at = match time_control {
            TimeControl::Infinite => None,
            TimeControl::ExactTime(move_time) => Some(now + *move_time),
            TimeControl::Clocks(ref clocks) => {
                Some(now + Self::time_allotted(game, clocks, move_overhead))
            }
        };

        let force_stop = Arc::new(AtomicBool::new(false));

        let control = Control {
            force_stop: force_stop.clone(),
        };

        let time_strategy = Self {
            started_at: now,
            stop_searching_at,

            next_check_at: params::CHECK_TERMINATION_NODE_FREQUENCY,

            force_stop,
        };

        (time_strategy, control)
    }

    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    fn time_allotted(game: &Game, clocks: &Clocks, move_overhead: Duration) -> Duration {
        let (time_remaining, increment) = match game.player {
            Player::White => (clocks.white_clock, clocks.white_increment),
            Player::Black => (clocks.black_clock, clocks.black_increment),
        };

        let increment = increment.unwrap_or_default();

        // If we don't have a time limit, spend a minute per move
        let Some(mut time_remaining) = time_remaining else {
            return Duration::from_secs(60);
        };

        time_remaining = time_remaining
            .saturating_sub(move_overhead)
            .max(move_overhead);

        if let Some(moves_to_go) = clocks.moves_to_go {
            // Try to use a roughly even amount of time per move
            return time_remaining / moves_to_go + increment;
        }

        let time_to_use = std::cmp::min(
            time_remaining.mul_f64(0.5),
            time_remaining.mul_f64(0.03333) + increment,
        );

        time_to_use
    }

    pub fn should_stop(&mut self, nodes_visited: u64) -> bool {
        if nodes_visited < self.next_check_at {
            return false;
        }

        if self.is_force_stopped() {
            return true;
        }

        self.next_check_at = nodes_visited + params::CHECK_TERMINATION_NODE_FREQUENCY;

        match self.stop_searching_at {
            None => false,
            Some(time_to_stop) => Instant::now() > time_to_stop,
        }
    }

    fn is_force_stopped(&self) -> bool {
        self.force_stop.load(Ordering::Relaxed)
    }
}
