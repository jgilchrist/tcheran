use std::time::Duration;

use crate::chess::moves::Move;
use crate::engine::game::EngineGame;
use crate::engine::options::EngineOptions;

pub use self::{main::MainStrategy, random::RandomMoveStrategy, top_eval::TopEvalStrategy};

pub trait Strategy<TCx: Control, TRx: Reporter>: Send + Sync {
    fn go(
        &mut self,
        game: &mut EngineGame,
        time_control: &TimeControl,
        search_restrictions: &SearchRestrictions,
        options: &EngineOptions,
        control: TCx,
        reporter: TRx,
    );
}

pub enum SearchScore {
    Centipawns(i16),
    Mate(i16),
}

#[derive(Default)]
pub struct SearchRestrictions {
    pub depth: Option<u8>,
}

#[derive(Debug, Clone)]
pub enum TimeControl {
    Clocks(Clocks),
    ExactTime(Duration),
    Infinite,
}

#[derive(Debug, Clone)]
pub struct Clocks {
    pub white_clock: Option<Duration>,
    pub black_clock: Option<Duration>,
    pub white_increment: Option<Duration>,
    pub black_increment: Option<Duration>,
    pub moves_to_go: Option<u32>,
}

pub struct SearchInfo {
    pub depth: u8,
    pub seldepth: u8,
    pub score: SearchScore,
    pub stats: SearchStats,
    pub pv: Vec<Move>,
    pub hashfull: usize,
}

pub struct SearchStats {
    pub time: Duration,
    pub nodes: u64,
    pub nodes_per_second: u64,
}

pub trait Control {
    fn stop(&self);
    fn should_stop(&self) -> bool;
}

pub struct NullControl;

impl Control for NullControl {
    fn stop(&self) {}

    fn should_stop(&self) -> bool {
        false
    }
}

pub trait Reporter {
    fn generic_report(&self, s: &str);

    fn report_search_progress(&self, progress: SearchInfo);
    fn report_search_stats(&self, stats: SearchStats);

    fn best_move(&self, mv: Move);
}

pub struct NullReporter;

impl Reporter for NullReporter {
    fn generic_report(&self, _: &str) {}

    fn report_search_progress(&self, _: SearchInfo) {}

    fn report_search_stats(&self, _: SearchStats) {}

    fn best_move(&self, _: Move) {}
}

mod main;
mod random;
mod top_eval;

#[derive(Debug, Clone)]
pub enum KnownStrategy {
    Main,
    Random,
    TopEval,
}

impl KnownStrategy {
    pub fn create<TCx: Control, TRx: Reporter>(&self) -> Box<dyn Strategy<TCx, TRx> + Send + Sync> {
        match self {
            Self::Main => Box::<MainStrategy>::default(),
            Self::Random => Box::<RandomMoveStrategy>::default(),
            Self::TopEval => Box::<TopEvalStrategy>::default(),
        }
    }

    pub const fn to_string(&self) -> &'static str {
        match self {
            Self::Main => "Main",
            Self::Random => "Random",
            Self::TopEval => "TopEval",
        }
    }
}
