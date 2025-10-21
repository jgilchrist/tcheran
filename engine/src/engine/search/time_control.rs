use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use crate::{
    chess::{game::Game, player::Player},
    engine::{
        options::EngineOptions,
        search::{TimeControl, params},
        util::log::crashlog,
    },
};

pub(crate) struct TimeStrategy {
    time_control: TimeControl,
    started_at: Instant,
    stopped: bool,

    soft_stop: Duration,
    hard_stop: Duration,

    next_check_at: u64,

    game: Game,
    control: Option<StopControl>,
}

#[derive(Clone)]
pub struct StopControl {
    force_stop: Arc<AtomicBool>,
}

impl StopControl {
    pub fn new() -> Self {
        Self {
            force_stop: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn stop(&self) {
        self.force_stop.store(true, Ordering::Relaxed);
    }

    pub fn should_stop(&self) -> bool {
        self.force_stop.load(Ordering::Relaxed)
    }
}

impl TimeStrategy {
    pub fn new(
        game: &Game,
        time_control: &TimeControl,
        control: Option<StopControl>,
        options: &EngineOptions,
    ) -> Self {
        let now = Instant::now();
        let move_overhead = Duration::from_millis(options.move_overhead as u64);

        let mut soft_stop = Duration::default();
        let mut hard_stop = Duration::default();

        match time_control {
            TimeControl::Infinite => {}
            TimeControl::Depth(_, stop) => {
                hard_stop = *stop;
            }
            TimeControl::ExactTime(move_time) => {
                soft_stop = *move_time;
                hard_stop = *move_time;
            }
            TimeControl::Clocks(clocks) => {
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

        Self {
            time_control: time_control.clone(),
            started_at: now,
            stopped: false,

            soft_stop,
            hard_stop,

            next_check_at: params::CHECK_TERMINATION_NODE_FREQUENCY,

            game: game.clone(),
            control,
        }
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
            TimeControl::Depth(d, _) => d >= depth,
        }
    }

    pub fn stopped(&self) -> bool {
        self.stopped
    }

    pub fn update(&mut self, nodes_visited: u64) {
        self.stopped = self.should_stop(nodes_visited);
    }

    fn should_stop(&mut self, nodes_visited: u64) -> bool {
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
            TimeControl::Depth(depth, hard_stop) => {
                if self.elapsed() > hard_stop {
                    crashlog(format!(
                        "Took longer than {:?} secs to search at depth {} for game {:?}",
                        hard_stop, depth, self.game
                    ));

                    return true;
                }

                false
            }
        }
    }

    fn is_force_stopped(&self) -> bool {
        let Some(control) = &self.control else {
            return false;
        };

        control.should_stop()
    }
}
