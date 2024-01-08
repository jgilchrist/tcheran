use crate::chess::moves::Move;
use std::time::Duration;

use crate::engine::eval::WhiteEval;
use crate::engine::options::EngineOptions;
use crate::engine::search::time_control::TimeStrategy;

use crate::chess::game::Game;
use crate::engine::search::transposition::SearchTranspositionTable;

mod iterative_deepening;
mod move_ordering;
mod negamax;
mod quiescence;
mod time_control;
pub mod transposition;

const MAX_SEARCH_DEPTH: u8 = u8::MAX;

pub struct SearchState {
    nodes_visited: u64,
    max_depth_reached: u8,
}

impl SearchState {
    const fn new() -> Self {
        Self {
            max_depth_reached: 0,
            nodes_visited: 0,
        }
    }
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

pub fn search(
    game: &mut Game,
    tt: &mut SearchTranspositionTable,
    time_control: &TimeControl,
    search_restrictions: &SearchRestrictions,
    options: &EngineOptions,
    control: &impl Control,
    reporter: &impl Reporter,
) -> (Move, WhiteEval) {
    let mut state = SearchState::new();

    let mut time_strategy = TimeStrategy::new(game, time_control);
    time_strategy.init();

    tt.new_generation();

    let (best_move, eval) = iterative_deepening::search(
        game,
        tt,
        search_restrictions,
        options,
        &mut state,
        &time_strategy,
        control,
        reporter,
    );

    (best_move, eval.to_white_eval(game.player))
}
