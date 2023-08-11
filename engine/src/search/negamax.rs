use chess::{game::Game, moves::Move};

use crate::{eval::{self}, strategy::{Reporter, SearchStats}};

use super::{negamax_eval::NegamaxEval, SearchState};

pub fn negamax(game: &Game, depth: u8, state: &mut SearchState, reporter: &impl Reporter) -> (Move, NegamaxEval) {
    let mut best_move: Option<Move> = None;
    let mut best_score = NegamaxEval::MIN;

    let root_moves = game.legal_moves();
    for mv in &root_moves {
        let game_after_move = game.make_move(mv).unwrap();
        state.nodes_visited += 1;

        let move_score = -negamax_inner(
            &game_after_move,
            NegamaxEval::MIN,
            NegamaxEval::MAX,
            depth - 1,
            state,
        );

        if move_score > best_score {
            best_score = move_score;
            best_move = Some(*mv);
        }

        reporter.report_search_stats(SearchStats {
            time: state.elapsed_time(),
            nodes: state.nodes_visited,
            nodes_per_second: state.nodes_per_second(),
        });
    }

    (best_move.unwrap(), best_score)
}

fn negamax_inner(
    game: &Game,
    mut alpha: NegamaxEval,
    beta: NegamaxEval,
    depth: u8,
    state: &mut SearchState,
) -> NegamaxEval {
    // TODO: Quiescence search
    // TODO: Check if game is over
    if depth == 0 {
        state.nodes_visited += 1;
        let eval = eval::eval(game);
        return NegamaxEval::from_eval(eval, game.player);
    }

    let legal_moves = game.legal_moves();

    for mv in &legal_moves {
        let game_after_move = game.make_move(mv).unwrap();
        state.nodes_visited += 1;

        let move_score = -negamax_inner(&game_after_move, -beta, -alpha, depth - 1, state);

        if move_score >= beta {
            state.beta_cutoffs += 1;
            return beta;
        }

        if move_score > alpha {
            alpha = move_score;
        }
    }

    alpha
}
