use crate::eval::{self};
use crate::search::time_control::TimeControl;
use crate::strategy::Control;
use crate::transposition::transposition_table::{
    NodeBound, SearchTranspositionTable, SearchTranspositionTableData,
};
use chess::{game::Game, moves::Move};

use super::{move_ordering, negamax_eval::NegamaxEval, SearchState};

pub fn negamax(
    game: &Game,
    mut alpha: NegamaxEval,
    beta: NegamaxEval,
    depth: u8,
    plies: u8,
    pv: &mut Vec<Move>,
    tt: &mut SearchTranspositionTable,
    time_control: &TimeControl,
    state: &mut SearchState,
    control: &impl Control,
) -> Result<NegamaxEval, ()> {
    let is_root = plies == 0;
    state.max_depth_reached = state.max_depth_reached.max(plies);

    if !is_root {
        state.nodes_visited += 1;
    }

    if !is_root && (game.is_stalemate_by_repetition() || game.is_stalemate_by_fifty_move_rule()) {
        pv.clear();
        return Ok(NegamaxEval::DRAW);
    }

    if depth == 0 {
        let eval = eval::eval(game);
        return Ok(NegamaxEval::from_eval(eval, game.player));
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

    let mut node_pv: Vec<Move> = vec![];

    // Check periodically to see if we're out of time. If we are, we shouldn't continue the search
    // so we return Err to signal to the caller that the search did not complete.
    if state.nodes_visited % 10000 == 0 && (time_control.should_stop() || control.should_stop()) {
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
        if !game.is_legal(mv) {
            continue;
        }

        let game_after_move = game.make_move(mv).unwrap();
        number_of_legal_moves += 1;

        let move_score = -negamax(
            &game_after_move,
            -beta,
            -alpha,
            depth - 1,
            plies + 1,
            &mut node_pv,
            tt,
            time_control,
            state,
            control,
        )?;

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

            pv.clear();
            pv.push(*mv);
            pv.extend_from_slice(&node_pv);
        }
    }

    if number_of_legal_moves == 0 {
        pv.clear();

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
