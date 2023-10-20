use std::time::{Duration, Instant};

// TODO: Handle increments

pub struct TimeControl {
    time_remaining: Option<Duration>,
    increment: Option<Duration>,
    stop_searching_at: Option<Instant>,
}

impl TimeControl {
    pub fn new(time_remaining: Option<Duration>, increment: Option<Duration>) -> Self {
        Self {
            time_remaining,
            increment,
            stop_searching_at: None,
        }
    }

    pub fn init(&mut self) {
        let time_allotted_for_move = self.time_allotted();
        self.stop_searching_at = Some(Instant::now() + time_allotted_for_move);
    }

    fn time_allotted(&self) -> Duration {
        let base_time_allotted = self.simple_time_allotted();

        let increment = self.increment.unwrap_or_default();

        base_time_allotted + increment
    }

    // TODO: Improve this - for now, it's super simple.
    fn simple_time_allotted(&self) -> Duration {
        // If we don't have a time limit, spend a minute per move
        let Some(time_remaining) = self.time_remaining else {
            return Duration::from_secs(60);
        };

        // Extreme time pressure - start blitzing
        if time_remaining < Duration::from_secs(10) {
            return Duration::from_millis(100);
        }

        // Time pressure - we have less than two minutes.
        if time_remaining < Duration::from_secs(60 * 2) {
            return Duration::from_secs(4);
        }

        Duration::from_secs(20)
    }

    pub fn should_stop(&self) -> bool {
        Instant::now() > self.stop_searching_at.unwrap()
    }
}
