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
    // Account for the possibility that there's some overhead making the move
    // e.g. sending the best move over the internet.
    const MOVE_OVERHEAD: Duration = Duration::from_millis(50);

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
            .saturating_sub(Self::MOVE_OVERHEAD)
            .max(Self::MOVE_OVERHEAD);

        let time_to_use = std::cmp::min(
            time_remaining.mul_f64(0.5),
            time_remaining.mul_f64(0.03333) + increment,
        );

        time_to_use
    }

    pub fn should_stop(&self) -> bool {
        match self.stop_searching_at {
            None => false,
            Some(time_to_stop) => Instant::now() > time_to_stop,
        }
    }
}
