use crate::chess::moves::Move;

use crate::engine::options::EngineOptions;
use crate::engine::search::time_control::TimeStrategy;
use crate::engine::strategy::{Control, SearchRestrictions, TimeControl};
use crate::engine::{eval::WhiteEval, strategy::Reporter};

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
