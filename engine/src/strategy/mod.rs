use std::time::Duration;

use chess::{game::Game, moves::Move};

pub use self::{main::MainStrategy, random::RandomMoveStrategy, top_eval::TopEvalStrategy};

pub trait Strategy<T: Reporter>: Send + Sync {
    fn go(&mut self, game: &Game, reporter: T);
}

pub trait Reporter {
    fn should_stop(&self) -> bool;

    fn generic_report(&self, s: &str);

    // TODO: Collect these fields into a struct
    fn report_current_move(&self, currmove: Move, nodes: u32, nps: u32, score: i32);
    fn report_search_progress(&self, depth: u32, time: Duration, nodes: u32, nps: u32, score: i32);

    fn best_move(&self, mv: Move);
}

mod main;
mod random;
mod top_eval;

pub enum KnownStrategy {
    Main,
    Random,
    TopEval,
}

impl KnownStrategy {
    #[must_use]
    pub fn create<T: Reporter>(&self) -> Box<dyn Strategy<T> + Send + Sync> {
        match self {
            Self::Main => Box::<MainStrategy>::default(),
            Self::Random => Box::<RandomMoveStrategy>::default(),
            Self::TopEval => Box::<TopEvalStrategy>::default(),
        }
    }
}
