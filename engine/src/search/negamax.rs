use crate::search::quiescence::quiescence;
use crate::search::time_control::TimeStrategy;
use crate::strategy::Control;
use crate::transposition::transposition_table::{
    NodeBound, SearchTranspositionTable, SearchTranspositionTableData,
};
use chess::{game::Game, moves::Move};

use super::{move_ordering, negamax_eval::NegamaxEval, SearchState};

pub fn negamax(
    game: &mut Game,
    mut alpha: NegamaxEval,
    beta: NegamaxEval,
    depth: u8,
    plies: u8,
    tt: &mut SearchTranspositionTable,
    time_control: &TimeStrategy,
    state: &mut SearchState,
    control: &impl Control,
) -> Result<NegamaxEval, ()> {
    let is_root = plies == 0;
    state.max_depth_reached = state.max_depth_reached.max(plies);

    if !is_root {
        state.nodes_visited += 1;
    }

    if !is_root && (game.is_stalemate_by_repetition() || game.is_stalemate_by_fifty_move_rule()) {
        return Ok(NegamaxEval::DRAW);
    }

    if depth == 0 {
        return quiescence(game, alpha, beta, plies, time_control, state, control);
    }

    let mut previous_best_move: Option<Move> = None;

    if let Some(tt_entry) = tt.get(&game.zobrist) {
        if !is_root && tt_entry.depth > depth {
            match tt_entry.bound {
                NodeBound::Exact => return Ok(tt_entry.eval),
                NodeBound::Upper if tt_entry.eval <= alpha => return Ok(alpha),
                NodeBound::Lower if tt_entry.eval >= beta => return Ok(beta),
                _ => {}
            }
        }

        previous_best_move = tt_entry.best_move;
    }

    // Check periodically to see if we're out of time. If we are, we shouldn't continue the search
    // so we return Err to signal to the caller that the search did not complete.
    if !is_root
        && state.nodes_visited % 10000 == 0
        && (time_control.should_stop() || control.should_stop())
    {
        return Err(());
    }

    // TODO: Generate only legal moves
    let mut moves = game.pseudo_legal_moves();

    move_ordering::order_moves(game, &mut moves, previous_best_move);

    let mut tt_node_bound = NodeBound::Upper;
    let mut best_move = None;
    let mut best_eval = NegamaxEval::MIN;
    let mut number_of_legal_moves = 0;

    for mv in &moves {
        let player = game.player;

        game.make_move(mv);
        if game.board.king_in_check(player) {
            game.undo_move();
            continue;
        }

        number_of_legal_moves += 1;

        let move_score = -negamax(
            game,
            -beta,
            -alpha,
            depth - 1,
            plies + 1,
            tt,
            time_control,
            state,
            control,
        )?;

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
            };

            tt.insert(&game.zobrist, tt_data);

            state.beta_cutoffs += 1;
            return Ok(beta);
        }

        if move_score > alpha {
            alpha = move_score;
            tt_node_bound = NodeBound::Exact;
        }
    }

    if number_of_legal_moves == 0 {
        return if game.board.king_in_check(game.player) {
            Ok(NegamaxEval::mated_in(plies))
        } else if game.board.king_in_check(game.player.other()) {
            Ok(NegamaxEval::mate_in(plies))
        } else {
            Ok(NegamaxEval::DRAW)
        };
    }

    let tt_data = SearchTranspositionTableData {
        bound: tt_node_bound,
        eval: alpha,
        best_move,
        depth,
    };

    tt.insert(&game.zobrist, tt_data);

    Ok(alpha)
}
