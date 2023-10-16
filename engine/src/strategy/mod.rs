use std::time::Duration;

use chess::{game::Game, moves::Move};

pub use self::{main::MainStrategy, random::RandomMoveStrategy, top_eval::TopEvalStrategy};

pub trait Strategy<TCx: Control, TRx: Reporter>: Send + Sync {
    fn go(&mut self, game: &Game, control: TCx, reporter: TRx);
}

pub enum SearchScore {
    Centipawns(i32),
    Mate(u32),
}

pub struct SearchInfo {
    pub depth: u32,
    pub score: SearchScore,
    pub stats: SearchStats,
    pub pv: Vec<Move>,
}

pub struct SearchStats {
    pub time: Duration,
    pub nodes: u32,
    pub nodes_per_second: u32,
}

pub trait Control {
    fn stop(&self);
    fn should_stop(&self) -> bool;
}

pub trait Reporter {
    fn generic_report(&self, s: &str);

    fn report_search_progress(&self, progress: SearchInfo);
    fn report_search_stats(&self, stats: SearchStats);

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
    pub fn create<TCx: Control, TRx: Reporter>(&self) -> Box<dyn Strategy<TCx, TRx> + Send + Sync> {
        match self {
            Self::Main => Box::<MainStrategy>::default(),
            Self::Random => Box::<RandomMoveStrategy>::default(),
            Self::TopEval => Box::<TopEvalStrategy>::default(),
        }
    }
}
