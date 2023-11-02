use crate::eval::Eval;
use crate::game::EngineGame;
use crate::search::quiescence::quiescence;
use crate::search::time_control::TimeStrategy;
use crate::search::transposition::{
    NodeBound, SearchTranspositionTable, SearchTranspositionTableData, TTMove,
};
use crate::strategy::Control;
use chess::moves::Move;

use super::{move_ordering, SearchState, MAX_SEARCH_DEPTH};

pub fn negamax(
    game: &mut EngineGame,
    mut alpha: Eval,
    beta: Eval,
    mut depth: u8,
    plies: u8,
    tt: &mut SearchTranspositionTable,
    time_control: &TimeStrategy,
    state: &mut SearchState,
    control: &impl Control,
) -> Result<Eval, ()> {
    let is_root = plies == 0;
    state.max_depth_reached = state.max_depth_reached.max(plies);

    // Keep track of whether we're doing a full search. If we raised alpha at this node, we've found
    // a new PV (or re-confirmed the PV we found at a previous search depth) - so for the remainder
    // of moves we search, we just need to check that they're worse. We can do this with more restrictive
    // alpha & beta bounds, and thus search less of the tree.
    let mut full_pv_search = true;

    // Check extension: If we're about to finish searching, but we are in check, we
    // should keep going.
    if depth == 0 {
        let in_check = game.is_king_in_check();
        if in_check && depth < MAX_SEARCH_DEPTH {
            depth += 1;
        }
    }

    if !is_root {
        state.nodes_visited += 1;
    }

    if !is_root && (game.is_repeated_position() || game.is_stalemate_by_fifty_move_rule()) {
        return Ok(Eval::DRAW);
    }

    if depth == 0 {
        return quiescence(game, alpha, beta, plies, time_control, state, control);
    }

    let mut previous_best_move: Option<Move> = None;

    if let Some(tt_entry) = tt.get(&game.zobrist()) {
        if !is_root && tt_entry.depth > depth {
            match tt_entry.bound {
                NodeBound::Exact => return Ok(tt_entry.eval),
                NodeBound::Upper if tt_entry.eval <= alpha => return Ok(alpha),
                NodeBound::Lower if tt_entry.eval >= beta => return Ok(beta),
                _ => {}
            }
        }

        previous_best_move = tt_entry.best_move.as_ref().map(TTMove::to_move);
    }

    // Check periodically to see if we're out of time. If we are, we shouldn't continue the search
    // so we return Err to signal to the caller that the search did not complete.
    if !is_root
        && state.nodes_visited % 10000 == 0
        && (time_control.should_stop() || control.should_stop())
    {
        return Err(());
    }

    let mut moves = game.moves();

    if moves.is_empty() {
        return Ok(if game.is_king_in_check() {
            Eval::mated_in(plies)
        } else {
            Eval::DRAW
        });
    }

    move_ordering::order_moves(&game.game, &mut moves, previous_best_move);

    let mut tt_node_bound = NodeBound::Upper;
    let mut best_move = None;
    let mut best_eval = Eval::MIN;

    for mv in &moves {
        game.make_move(mv);

        let move_score = if full_pv_search {
            -negamax(
                game,
                -beta,
                -alpha,
                depth - 1,
                plies + 1,
                tt,
                time_control,
                state,
                control,
            )?
        } else {
            // We already found a good move (i.e. we raised alpha).
            // Now, we just need to prove that the other moves are worse.
            // We search them with a reduced window to prove that they are at least worse.
            let pvs_score = -negamax(
                game,
                -alpha - Eval(1),
                -alpha,
                depth - 1,
                plies + 1,
                tt,
                time_control,
                state,
                control,
            )?;

            // Turns out the move we just searched could be better than our current PV, so we re-search
            // with the normal alpha/beta bounds.
            if pvs_score > alpha && pvs_score < beta {
                -negamax(
                    game,
                    -beta,
                    -alpha,
                    depth - 1,
                    plies + 1,
                    tt,
                    time_control,
                    state,
                    control,
                )?
            } else {
                pvs_score
            }
        };

        game.undo_move();

        if move_score > best_eval {
            best_move = Some(*mv);
            best_eval = move_score;
        }

        // Cutoff: This move is so good that our opponent won't let it be played.
        if move_score >= beta {
            let tt_data = SearchTranspositionTableData {
                bound: NodeBound::Lower,
                eval: move_score,
                best_move: None,
                depth,
                age: tt.generation,
            };

            tt.insert(&game.zobrist(), tt_data);

            return Ok(beta);
        }

        if move_score > alpha {
            alpha = move_score;
            tt_node_bound = NodeBound::Exact;

            // We've found a PV move, so we can try and prove that the rest of the moves in this
            // position are worse.
            full_pv_search = false;
        }
    }

    let tt_data = SearchTranspositionTableData {
        bound: tt_node_bound,
        eval: alpha,
        best_move: best_move.map(|m| TTMove::from_move(&m)),
        age: tt.generation,
        depth,
    };

    tt.insert(&game.zobrist(), tt_data);

    Ok(alpha)
}
