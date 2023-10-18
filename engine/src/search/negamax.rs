use chess::game::GameStatus;
use chess::{game::Game, moves::Move};
use rand::Rng;

use crate::eval::Eval;
use crate::eval::{self};
use crate::search::time_control::TimeControl;

use super::{move_ordering, negamax_eval::NegamaxEval, SearchState};

pub fn negamax(
    game: &Game,
    mut alpha: NegamaxEval,
    beta: NegamaxEval,
    depth: u8,
    plies: u8,
    pv: &mut Vec<Move>,
    best_previous_move: Option<Move>,
    time_control: &TimeControl,
    state: &mut SearchState,
) -> Result<NegamaxEval, ()> {
    state.max_depth_reached = state.max_depth_reached.max(plies);
    state.nodes_visited += 1;

    if depth == 0 {
        pv.clear();

        // Introduce a tiny bit of noise into the evaluation function to add some variation
        // to play in the same situations where we'd otherwise always pick the first move
        // with the same score.
        let eval_noise = rand::thread_rng().gen_range(0..10);
        let eval = eval::eval(game) + Eval(eval_noise);

        return Ok(NegamaxEval::from_eval(eval, game.player));
    }

    let mut line: Vec<Move> = vec![];

    let game_status = game.game_status();
    if let Some(status) = game_status {
        pv.clear();

        return Ok(match status {
            GameStatus::Won => NegamaxEval::mate_in(plies),
            GameStatus::Lost => NegamaxEval::mated_in(plies),
            GameStatus::Stalemate => NegamaxEval::DRAW,
        });
    }

    // Check periodically to see if we're out of time. If we are, we shouldn't continue the search
    // so we return Err to signal to the caller that the search did not complete.
    if state.nodes_visited % 10000 == 0 && time_control.should_stop() {
        return Err(());
    }

    let mut legal_moves = game.legal_moves();
    move_ordering::order_moves(game, &mut legal_moves, best_previous_move);

    for mv in &legal_moves {
        let game_after_move = game.make_move(mv).unwrap();

        let move_score = -negamax(
            &game_after_move,
            -beta,
            -alpha,
            depth - 1,
            plies + 1,
            &mut line,
            None,
            time_control,
            state,
        )?;

        if move_score >= beta {
            state.beta_cutoffs += 1;
            return Ok(beta);
        }

        if move_score > alpha {
            alpha = move_score;

            pv.clear();
            pv.push(*mv);
            pv.extend_from_slice(&line);
        }
    }

    Ok(alpha)
}
