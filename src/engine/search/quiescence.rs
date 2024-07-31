use crate::chess::game::Game;
use crate::chess::moves::Move;
use crate::engine::eval;
use crate::engine::eval::Eval;
use crate::engine::search::move_provider::MoveProvider;
use crate::engine::search::time_control::TimeStrategy;
use crate::engine::search::transposition::{NodeBound, SearchTranspositionTableData, TTMove};

use super::{params, Control, PersistentState, SearchState, MAX_SEARCH_DEPTH};

pub fn quiescence(
    game: &mut Game,
    mut alpha: Eval,
    beta: Eval,
    plies: u8,
    time_control: &TimeStrategy,
    persistent_state: &mut PersistentState,
    state: &mut SearchState,
    control: &impl Control,
) -> Result<Eval, ()> {
    state.max_depth_reached = state.max_depth_reached.max(plies);
    state.nodes_visited += 1;

    if plies == MAX_SEARCH_DEPTH {
        return Ok(eval::eval(game));
    }

    if game.is_repeated_position()
        || game.is_stalemate_by_fifty_move_rule()
        || game.is_stalemate_by_insufficient_material()
    {
        return Ok(Eval::DRAW);
    }

    // Check periodically to see if we're out of time. If we are, we shouldn't continue the search
    // so we return Err to signal to the caller that the search did not complete.
    if state.nodes_visited % params::CHECK_TERMINATION_NODE_FREQUENCY == 0
        && (time_control.should_stop() || control.should_stop())
    {
        return Err(());
    }

    let eval = eval::eval(game);

    if eval >= beta {
        return Ok(beta);
    }

    if eval > alpha {
        alpha = eval;
    }

    let mut previous_best_move: Option<Move> = None;

    if let Some(tt_entry) = persistent_state.tt.get(&game.zobrist) {
        let tt_score = tt_entry.eval.with_mate_distance_from_root(plies);

        match tt_entry.bound {
            NodeBound::Exact => return Ok(tt_score),
            NodeBound::Upper if tt_entry.eval <= alpha => return Ok(tt_score),
            NodeBound::Lower if tt_entry.eval >= beta => return Ok(tt_score),
            _ => {}
        }

        previous_best_move = tt_entry.best_move.as_ref().map(TTMove::to_move);
    }

    let mut best_eval = eval;
    let mut tt_node_bound = NodeBound::Upper;
    let mut best_move = None;

    let mut moves = MoveProvider::new_loud(previous_best_move);
    while let Some(mv) = moves.next(game, persistent_state, state, plies) {
        game.make_move(mv);

        let move_score = -quiescence(
            game,
            -beta,
            -alpha,
            plies + 1,
            time_control,
            persistent_state,
            state,
            control,
        )?;

        game.undo_move();

        if move_score > best_eval {
            best_eval = move_score;
            best_move = Some(mv);
        }

        // Cutoff: This move is so good that our opponent won't let it be played.
        if move_score >= beta {
            tt_node_bound = NodeBound::Lower;
            break;
        }

        if move_score > alpha {
            alpha = move_score;
            tt_node_bound = NodeBound::Exact;
        }
    }

    let tt_data = SearchTranspositionTableData {
        bound: tt_node_bound,
        eval: best_eval.with_mate_distance_from_position(plies),
        best_move: best_move.map(TTMove::from_move),
        age: persistent_state.tt.generation,
        depth: 0,
    };

    persistent_state.tt.insert(&game.zobrist, tt_data);

    Ok(best_eval)
}
