use chess::game::Game;
use chess::moves::Move;
use crate::options::EngineOptions;
use crate::search::negamax_eval::NegamaxEval;
use crate::search::{negamax, SearchState};
use crate::strategy::{Reporter, SearchInfo, SearchScore, SearchStats};

pub(crate) fn search(game: &Game, options: &EngineOptions, state: &mut SearchState, reporter: &impl Reporter) -> (Move, NegamaxEval) {
    let mut overall_best_move: Option<Move> = None;
    let mut overall_eval: Option<NegamaxEval> = None;

    for depth in 1..=options.max_search_depth {
        let (best_move, pv, eval) = negamax::negamax(game, depth, state, reporter);

        let score = if let Some(nmoves) = eval.is_mate_in_moves() {
            SearchScore::Mate(nmoves)
        } else {
            SearchScore::Centipawns(eval.0)
        };

        overall_best_move = Some(best_move);
        overall_eval = Some(eval);

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