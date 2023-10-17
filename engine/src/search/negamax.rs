use chess::game::GameStatus;
use chess::{game::Game, moves::Move};
use rand::Rng;

use crate::eval::Eval;
use crate::{
    eval::{self},
    strategy::{Reporter, SearchStats},
};

use super::{move_ordering, negamax_eval::NegamaxEval, SearchState};

pub fn negamax(
    game: &Game,
    depth: u8,
    state: &mut SearchState,
    reporter: &impl Reporter,
) -> (Move, Vec<Move>, NegamaxEval) {
    let mut best_move: Option<Move> = None;
    let mut best_line: Option<Vec<Move>> = None;
    let mut best_score = NegamaxEval::MIN;

    let mut root_moves = game.legal_moves();
    move_ordering::order_moves(game, &mut root_moves);

    for mv in &root_moves {
        let game_after_move = game.make_move(mv).unwrap();
        state.nodes_visited += 1;

        let mut line: Vec<Move> = vec![];

        let move_score = -negamax_inner(
            &game_after_move,
            NegamaxEval::MIN,
            NegamaxEval::MAX,
            depth - 1,
            1,
            &mut line,
            state,
        );

        if move_score > best_score {
            line.insert(0, *mv);
            best_score = move_score;
            best_line = Some(line);
            best_move = Some(*mv);
        }

        reporter.report_search_stats(SearchStats {
            time: state.elapsed_time(),
            nodes: state.nodes_visited,
            nodes_per_second: state.nodes_per_second(),
        });
    }

    (best_move.unwrap(), best_line.unwrap(), best_score)
}

fn negamax_inner(
    game: &Game,
    mut alpha: NegamaxEval,
    beta: NegamaxEval,
    depth: u8,
    plies: u8,
    pv: &mut Vec<Move>,
    state: &mut SearchState,
) -> NegamaxEval {
    state.max_depth_reached = state.max_depth_reached.max(plies);

    if depth == 0 {
        pv.clear();

        state.nodes_visited += 1;

        // Introduce a tiny bit of noise into the evaluation function to add some variation
        // to play in the same situations where we'd otherwise always pick the first move
        // with the same score.
        let eval_noise = rand::thread_rng().gen_range(0..10);
        let eval = eval::eval(game) + Eval(eval_noise);

        return NegamaxEval::from_eval(eval, game.player);
    }

    let mut line: Vec<Move> = vec![];

    let game_status = game.game_status();
    if let Some(status) = game_status {
        pv.clear();

        return match status {
            GameStatus::Won => NegamaxEval::mate_in(plies),
            GameStatus::Lost => NegamaxEval::mated_in(plies),
            GameStatus::Stalemate => NegamaxEval::DRAW,
        };
    }

    let mut legal_moves = game.legal_moves();
    move_ordering::order_moves(game, &mut legal_moves);

    for mv in &legal_moves {
        let game_after_move = game.make_move(mv).unwrap();
        state.nodes_visited += 1;

        let move_score = -negamax_inner(
            &game_after_move,
            -beta,
            -alpha,
            depth - 1,
            plies + 1,
            &mut line,
            state,
        );

        if move_score >= beta {
            state.beta_cutoffs += 1;
            return beta;
        }

        if move_score > alpha {
            alpha = move_score;

            pv.clear();
            pv.push(*mv);
            pv.extend_from_slice(&line);
        }
    }

    alpha
}
