use std::time::Duration;

use chess::{game::Game, moves::Move};

pub use self::{main::MainStrategy, random::RandomMoveStrategy, top_eval::TopEvalStrategy};

pub trait Strategy<T: Reporter>: Send + Sync {
    fn go(&mut self, game: &Game, reporter: T);
}

pub enum SearchScore {
    Centipawns(i32),
    Mate(u32),
}

pub struct SearchInfo {
    pub depth: u32,
    pub score: SearchScore,
    pub stats: SearchStats,
}

pub struct SearchStats {
    pub time: Duration,
    pub nodes: u32,
    pub nodes_per_second: u32,
}

pub trait Reporter {
    fn should_stop(&self) -> bool;

    fn generic_report(&self, s: &str);

    fn report_search_progress(&self, progress: &SearchInfo);
    fn report_search_stats(&self, stats: &SearchStats);

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
