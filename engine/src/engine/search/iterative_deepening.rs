use crate::chess::game::Game;
use crate::chess::moves::Move;
use crate::engine::eval::Eval;
use crate::engine::search::aspiration::aspiration_search;
use crate::engine::search::principal_variation::PrincipalVariation;
use crate::engine::search::{MAX_SEARCH_DEPTH, Reporter, SearchContext, SearchInfo, SearchStats};
use crate::engine::util;

pub fn search(
    game: &mut Game,
    ctx: &mut SearchContext<'_>,
    pv: &mut PrincipalVariation,
    reporter: &mut impl Reporter,
) -> Option<Move> {
    let mut best_move: Option<Move> = None;
    let mut overall_eval: Option<Eval> = None;

    ctx.max_depth_reached = 0;

    for depth in 1..=MAX_SEARCH_DEPTH {
        if !ctx.time_control.should_start_new_search(depth) {
            break;
        }

        let Ok(eval) = aspiration_search(game, depth, overall_eval, pv, ctx) else {
            break;
        };

        best_move = Some(*pv.first().unwrap());
        overall_eval = Some(eval);

        reporter.report_search_progress(
            game,
            SearchInfo {
                depth,
                seldepth: ctx.max_depth_reached,
                eval,
                pv: pv.clone(),
                hashfull: ctx.tt.occupancy(),
                stats: SearchStats {
                    time: ctx.time_control.elapsed(),
                    nodes: ctx.nodes_visited,
                    nodes_per_second: util::metrics::nodes_per_second(
                        ctx.nodes_visited,
                        ctx.time_control.elapsed(),
                    ),
                    tbhits: ctx.tbhits,
                },
            },
        );
    }

    best_move
}
