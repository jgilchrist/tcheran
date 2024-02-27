use crate::chess::game::Game;
use crate::chess::player::Player;
use crate::engine::options::EngineOptions;
use crate::engine::search::{Clocks, TimeControl};
use std::time::{Duration, Instant};

pub struct TimeStrategy {
    player: Player,
    time_control: TimeControl,
    move_overhead: Duration,

    started_at: Option<Instant>,
    stop_searching_at: Option<Instant>,
}

impl TimeStrategy {
    const MINIMUM_MOVE_TIME: Duration = Duration::from_millis(10);

    pub fn new(game: &Game, time_control: &TimeControl, options: &EngineOptions) -> Self {
        Self {
            time_control: time_control.clone(),
            player: game.player,
            move_overhead: Duration::from_millis(options.move_overhead as u64),

            started_at: None,
            stop_searching_at: None,
        }
    }

    pub fn init(&mut self) {
        self.started_at = Some(Instant::now());

        self.stop_searching_at = match self.time_control {
            TimeControl::Infinite => None,
            TimeControl::ExactTime(move_time) => Some(Instant::now() + move_time),
            TimeControl::Clocks(ref clocks) => Some(Instant::now() + self.time_allotted(clocks)),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.started_at.unwrap().elapsed()
    }

    fn time_allotted(&self, clocks: &Clocks) -> Duration {
        let (time_remaining, increment) = match self.player {
            Player::White => (clocks.white_clock, clocks.white_increment),
            Player::Black => (clocks.black_clock, clocks.black_increment),
        };

        let increment = increment.unwrap_or_default();

        // If we don't have a time limit, spend a minute per move
        let Some(mut time_remaining) = time_remaining else {
            return Duration::from_secs(60);
        };

        time_remaining = time_remaining
            .saturating_sub(self.move_overhead)
            .max(self.move_overhead);

        if let Some(moves_to_go) = clocks.moves_to_go {
            // Try to use a roughly even amount of time per move
            return time_remaining / moves_to_go + increment;
        }

        let time_to_use = std::cmp::min(
            time_remaining.mul_f64(0.5),
            time_remaining.mul_f64(0.03333) + increment,
        );

        std::cmp::max(time_to_use, Self::MINIMUM_MOVE_TIME)
    }

    pub fn should_stop(&self) -> bool {
        match self.stop_searching_at {
            None => false,
            Some(time_to_stop) => Instant::now() > time_to_stop,
        }
    }
}
