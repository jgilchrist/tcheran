use crate::options::EngineOptions;
use crate::search::negamax_eval::NegamaxEval;
use crate::search::time_control::TimeControl;
use crate::search::{negamax, SearchState};
use crate::strategy::{Control, Reporter, SearchInfo, SearchScore, SearchStats};
use crate::transposition::transposition_table::NodeBound::Exact;
use crate::transposition::transposition_table::SearchTranspositionTable;
use chess::game::Game;
use chess::moves::Move;

const MAX_SEARCH_DEPTH: u8 = 100;

pub fn search(
    game: &Game,
    _options: &EngineOptions,
    state: &mut SearchState,
    time_control: &TimeControl,
    control: &impl Control,
    reporter: &impl Reporter,
) -> (Move, NegamaxEval) {
    let mut overall_best_move: Option<Move> = None;
    let mut overall_eval: Option<NegamaxEval> = None;

    let mut tt = SearchTranspositionTable::new();

    for depth in 1..=MAX_SEARCH_DEPTH {
        // TODO: Are we counting nodes searched at this depth?
        state.nodes_visited = 0;

        let Ok(eval) = negamax::negamax(
            game,
            NegamaxEval::MIN,
            NegamaxEval::MAX,
            depth,
            0,
            &mut tt,
            time_control,
            state,
            control,
        ) else {
            // TODO: Send results, even if the search is cancelled, since they may still be better
            // than whatever we found at the previous depth even if we didn't finish the search.
            break;
        };

        let score = if let Some(nmoves) = eval.is_mate_in_moves() {
            SearchScore::Mate(nmoves)
        } else {
            SearchScore::Centipawns(eval.0)
        };

        let pv = get_pv(game, &tt);

        let best_move = pv.first().unwrap();

        overall_best_move = Some(*best_move);
        overall_eval = Some(eval);
        state.best_pv = Some(pv.clone());

        reporter.report_search_progress(SearchInfo {
            depth,
            seldepth: state.max_depth_reached,
            score,
            pv: pv.clone(),
            hashfull: tt.occupancy(),
            stats: SearchStats {
                time: state.elapsed_time(),
                nodes: state.nodes_visited,
                nodes_per_second: state.nodes_per_second(),
            },
        });
    }

    (overall_best_move.unwrap(), overall_eval.unwrap())
}

fn get_pv(game: &Game, tt: &SearchTranspositionTable) -> Vec<Move> {
    let mut current_position = game.clone();
    let mut pv = Vec::new();

    loop {
        let Some(tt_entry) = tt.get(&current_position.zobrist) else {
            break;
        };

        if tt_entry.bound != Exact {
            panic!("non-exact bound")
        }

        let best_move_in_position = tt_entry.best_move.unwrap();
        pv.push(best_move_in_position);
        current_position = current_position.make_move(&best_move_in_position).unwrap();
    }

    pv
}
