use crate::chess::game::Game;
use crate::chess::moves::Move;
use crate::engine::eval::Eval;
use crate::engine::options::EngineOptions;
use crate::engine::search::time_control::TimeStrategy;
use crate::engine::search::transposition::{NodeBound, SearchTranspositionTable};
use crate::engine::search::{
    negamax, Control, PersistentState, Reporter, SearchInfo, SearchRestrictions, SearchScore,
    SearchState, SearchStats, MAX_SEARCH_DEPTH,
};
use crate::engine::util;

pub fn search(
    game: &mut Game,
    persistent_state: &mut PersistentState,
    search_restrictions: &SearchRestrictions,
    _options: &EngineOptions,
    state: &mut SearchState,
    time_control: &TimeStrategy,
    control: &impl Control,
    reporter: &mut impl Reporter,
) -> Option<Move> {
    let mut best_move: Option<Move> = None;

    let max_search_depth = search_restrictions.depth.unwrap_or(MAX_SEARCH_DEPTH);
    state.max_depth_reached = 0;

    for depth in 1..=max_search_depth {
        let Ok(eval) = negamax::negamax(
            game,
            Eval::MIN,
            Eval::MAX,
            depth,
            0,
            persistent_state,
            time_control,
            state,
            control,
        ) else {
            break;
        };

        let score = if let Some(nmoves) = eval.is_mate_in_moves() {
            SearchScore::Mate(nmoves)
        } else {
            SearchScore::Centipawns(eval.0)
        };

        let pv = get_pv(depth, game.clone(), &persistent_state.tt, state);
        best_move = Some(*pv.first().unwrap());

        reporter.report_search_progress(SearchInfo {
            depth,
            seldepth: state.max_depth_reached,
            score,
            pv: pv.clone(),
            hashfull: persistent_state.tt.occupancy(),
            stats: SearchStats {
                time: time_control.elapsed(),
                nodes: state.nodes_visited,
                nodes_per_second: util::metrics::nodes_per_second(
                    state.nodes_visited,
                    time_control.elapsed(),
                ),
            },
        });
    }

    best_move
}

fn get_pv(depth: u8, game: Game, tt: &SearchTranspositionTable, state: &SearchState) -> Vec<Move> {
    let mut current_position = game;
    let mut pv = Vec::new();

    let depth_reached = std::cmp::min(depth, state.max_depth_reached);

    for _ in 0..depth_reached {
        let Some(tt_entry) = tt.get(&current_position.zobrist) else {
            break;
        };

        if tt_entry.bound != NodeBound::Exact {
            assert!(!pv.is_empty());
            return pv;
        }

        let best_move_in_position = tt_entry.best_move.as_ref().unwrap().to_move();
        pv.push(best_move_in_position);
        current_position.make_move(best_move_in_position);
    }

    pv
}
