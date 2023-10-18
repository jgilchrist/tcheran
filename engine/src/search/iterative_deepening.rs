use crate::options::EngineOptions;
use crate::search::negamax_eval::NegamaxEval;
use crate::search::time_control::TimeControl;
use crate::search::{negamax, SearchState};
use crate::strategy::{Reporter, SearchInfo, SearchScore, SearchStats};
use chess::game::Game;
use chess::moves::Move;

const MAX_SEARCH_DEPTH: u8 = 100;

pub fn search(
    game: &Game,
    _options: &EngineOptions,
    state: &mut SearchState,
    time_control: &TimeControl,
    reporter: &impl Reporter,
) -> (Move, NegamaxEval) {
    let mut overall_best_move: Option<Move> = None;
    let mut overall_eval: Option<NegamaxEval> = None;

    for depth in 1..=MAX_SEARCH_DEPTH {
        let Ok((best_move, pv, eval)) =
            negamax::negamax(game, depth, state, time_control, reporter)
        else {
            // TODO: Send results, even if the search is cancelled, since they may still be better
            // than whatever we found at the previous depth even if we didn't finish the search.
            break;
        };

        let score = if let Some(nmoves) = eval.is_mate_in_moves() {
            SearchScore::Mate(nmoves)
        } else {
            SearchScore::Centipawns(eval.0)
        };

        overall_best_move = Some(best_move);
        overall_eval = Some(eval);
        state.best_pv = Some(pv.clone());

        reporter.report_search_progress(SearchInfo {
            depth,
            seldepth: state.max_depth_reached,
            score,
            pv,
            stats: SearchStats {
                time: state.elapsed_time(),
                nodes: state.nodes_visited,
                nodes_per_second: state.nodes_per_second(),
            },
        });
    }

    (overall_best_move.unwrap(), overall_eval.unwrap())
}
