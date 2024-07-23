use crate::chess::moves::Move;
use std::time::Duration;

use crate::engine::options::EngineOptions;
use crate::engine::search::time_control::TimeStrategy;

use crate::chess::game::Game;
use crate::engine::search::move_provider::MoveProvider;
use crate::engine::search::tables::{HistoryTable, KillersTable};
use crate::engine::search::transposition::SearchTranspositionTable;

mod iterative_deepening;
mod move_ordering;
pub mod move_provider;
mod negamax;
mod quiescence;
mod tables;
mod time_control;
pub mod transposition;

const MAX_SEARCH_DEPTH: u8 = u8::MAX;
const MAX_SEARCH_DEPTH_SIZE: usize = MAX_SEARCH_DEPTH as usize;

mod params {
    use crate::engine::eval::Eval;

    pub const CHECK_TERMINATION_NODE_FREQUENCY: u64 = 10000;

    pub const NULL_MOVE_PRUNING_DEPTH_LIMIT: u8 = 3;
    pub const NULL_MOVE_PRUNING_DEPTH_REDUCTION: u8 = 2;

    pub const REVERSE_FUTILITY_PRUNE_DEPTH: u8 = 4;
    pub const REVERSE_FUTILITY_PRUNE_MARGIN_PER_PLY: Eval = Eval::new(150);

    pub const HISTORY_DECAY_FACTOR: i32 = 8;
}

pub struct PersistentState {
    pub tt: SearchTranspositionTable,
    pub history_table: HistoryTable,
}

impl PersistentState {
    pub fn new() -> Self {
        Self {
            tt: SearchTranspositionTable::default(),
            history_table: HistoryTable::new(),
        }
    }
}

pub struct SearchState {
    pub killer_moves: KillersTable,

    nodes_visited: u64,
    max_depth_reached: u8,
}

impl SearchState {
    pub const fn new() -> Self {
        Self {
            killer_moves: KillersTable::new(),

            max_depth_reached: 0,
            nodes_visited: 0,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
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
    fn reset(&self);

    fn should_stop(&self) -> bool;
    fn set_stopped(&self);
}

pub struct NullControl;

impl Control for NullControl {
    fn stop(&self) {}
    fn reset(&self) {}

    fn set_stopped(&self) {}

    fn should_stop(&self) -> bool {
        false
    }
}

pub trait Reporter {
    #[allow(unused)]
    fn generic_report(&self, s: &str);

    fn report_search_progress(&mut self, progress: SearchInfo);

    #[allow(unused)]
    fn report_search_stats(&mut self, stats: SearchStats);

    fn best_move(&self, mv: Move);
}

pub struct NullReporter;

impl Reporter for NullReporter {
    fn generic_report(&self, _: &str) {}

    fn report_search_progress(&mut self, _: SearchInfo) {}

    fn report_search_stats(&mut self, _: SearchStats) {}

    fn best_move(&self, _: Move) {}
}

pub struct CapturingReporter {
    pub score: Option<SearchScore>,
    pub nodes: u64,
    pub nps: u64,
}

impl CapturingReporter {
    pub fn new() -> Self {
        Self {
            score: None,
            nodes: 0,
            nps: 0,
        }
    }
}

impl Reporter for CapturingReporter {
    fn generic_report(&self, _: &str) {}

    fn report_search_progress(&mut self, stats: SearchInfo) {
        self.score = Some(stats.score);
        self.nodes = stats.stats.nodes;
        self.nps = stats.stats.nodes_per_second;
    }

    fn report_search_stats(&mut self, _: SearchStats) {}

    fn best_move(&self, _: Move) {}
}

pub fn search(
    game: &Game,
    persistent_state: &mut PersistentState,
    time_control: &TimeControl,
    search_restrictions: &SearchRestrictions,
    options: &EngineOptions,
    control: &impl Control,
    reporter: &mut impl Reporter,
) -> Move {
    let mut state = SearchState::new();

    let mut time_strategy = TimeStrategy::new(game, time_control, options);
    time_strategy.init();

    persistent_state.tt.new_generation();
    persistent_state
        .history_table
        .decay(params::HISTORY_DECAY_FACTOR);

    // The game is modified as moves are played during search. When the search terminates,
    // the game will be left in a dirty state since we will not undo the moves played to
    // reach the terminating node in the search tree. To keep our original 'game' copy clean
    // we perform the search on a copy of the game.
    let mut search_game = game.clone();

    let best_move = iterative_deepening::search(
        &mut search_game,
        persistent_state,
        search_restrictions,
        options,
        &mut state,
        &time_strategy,
        control,
        reporter,
    );

    best_move.unwrap_or_else(|| panic_move(game, persistent_state, &state))
}

// If we have so little time to search that we couldn't determine a best move, we'll need to spend
// a bit of extra time so that we still make a move.
// Rather than returning a random move, we return the first move that is returned after move ordering
fn panic_move(game: &Game, persistent_state: &PersistentState, search_state: &SearchState) -> Move {
    let mut move_provider = MoveProvider::new(None);
    move_provider
        .next(game, persistent_state, search_state, 0)
        .unwrap()
}
