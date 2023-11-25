use crate::strategy::{Clocks, TimeControl};
use chess::game::Game;
use chess::player::Player;
use std::time::{Duration, Instant};

pub struct TimeStrategy {
    player: Player,
    time_control: TimeControl,

    started_at: Option<Instant>,
    stop_searching_at: Option<Instant>,
}

impl TimeStrategy {
    pub fn new(game: &Game, time_control: &TimeControl) -> Self {
        Self {
            time_control: time_control.clone(),
            player: game.player,
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

    // TODO: Improve this - for now, it's super simple.
    fn time_allotted(&self, clocks: &Clocks) -> Duration {
        let (time_remaining, increment) = match self.player {
            Player::White => (clocks.white_clock, clocks.white_increment),
            Player::Black => (clocks.black_clock, clocks.black_increment),
        };

        let increment = increment.unwrap_or_default();

        // If we don't have a time limit, spend a minute per move
        let Some(time_remaining) = time_remaining else {
            return Duration::from_secs(60);
        };

        // About to lose on time
        if time_remaining < Duration::from_secs(1) {
            return Duration::from_millis(100);
        }

        // Extreme time pressure - start blitzing
        if time_remaining < Duration::from_secs(2) {
            return Duration::from_millis(500);
        }

        // Moderate time pressure - less than a minute.
        if time_remaining < Duration::from_secs(60) {
            return Duration::from_secs(1) + increment;
        }

        // Time pressure - we have less than two minutes.
        if time_remaining < Duration::from_secs(60 * 3) {
            return Duration::from_secs(4) + increment;
        }

        Duration::from_secs(20) + increment
    }

    pub fn should_stop(&self) -> bool {
        match self.stop_searching_at {
            None => false,
            Some(time_to_stop) => Instant::now() > time_to_stop,
        }
    }
}
