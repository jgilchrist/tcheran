use std::time::{Duration, Instant};

use chess::{game::Game, moves::Move};

use crate::options::EngineOptions;
use crate::search::time_control::TimeStrategy;
use crate::strategy::{Control, SearchRestrictions, TimeControl};
use crate::{eval::Eval, strategy::Reporter};

use crate::transposition::transposition_table::SearchTranspositionTable;
pub use negamax_eval::NegamaxEval;

mod iterative_deepening;
mod move_ordering;
mod negamax;
mod negamax_eval;
mod quiescence;
mod time_control;

const MAX_SEARCH_DEPTH: u8 = u8::MAX;

pub struct SearchState {
    start_time: Option<Instant>,
    best_pv: Option<Vec<Move>>,
    nodes_visited: u32,
    max_depth_reached: u8,
}

impl SearchState {
    const fn new() -> Self {
        Self {
            start_time: None,
            max_depth_reached: 0,
            nodes_visited: 0,
            best_pv: None,
        }
    }

    pub fn start_timer(&mut self) {
        self.start_time = Some(Instant::now());
    }

    #[must_use]
    pub fn elapsed_time(&self) -> Duration {
        let Some(t) = self.start_time else {
            panic!("Tried to fetch search's elapsed time without a start time.")
        };

        t.elapsed()
    }
}

pub fn search(
    game: &mut Game,
    tt: &mut SearchTranspositionTable,
    time_control: &TimeControl,
    search_restrictions: &SearchRestrictions,
    options: &EngineOptions,
    control: &impl Control,
    reporter: &impl Reporter,
) -> (Move, Eval) {
    let mut state = SearchState::new();
    state.start_timer();

    let mut time_strategy = TimeStrategy::new(game, time_control);
    time_strategy.init();

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

    (best_move, eval.to_eval(game.player))
}
