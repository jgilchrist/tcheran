use crate::eval::{self};
use crate::search::time_control::TimeStrategy;
use crate::strategy::Control;
use chess::game::Game;

use super::{move_ordering, negamax_eval::NegamaxEval, SearchState, MAX_SEARCH_DEPTH};

pub fn quiescence(
    game: &mut Game,
    mut alpha: NegamaxEval,
    beta: NegamaxEval,
    plies: u8,
    time_control: &TimeStrategy,
    state: &mut SearchState,
    control: &impl Control,
) -> Result<NegamaxEval, ()> {
    state.max_depth_reached = state.max_depth_reached.max(plies);
    state.nodes_visited += 1;

    if plies == MAX_SEARCH_DEPTH {
        let eval = eval::eval(game);
        return Ok(NegamaxEval::from_eval(eval, game.player));
    }

    if game.is_stalemate_by_repetition() || game.is_stalemate_by_fifty_move_rule() {
        return Ok(NegamaxEval::DRAW);
    }

    // Check periodically to see if we're out of time. If we are, we shouldn't continue the search
    // so we return Err to signal to the caller that the search did not complete.
    if state.nodes_visited % 10000 == 0 && (time_control.should_stop() || control.should_stop()) {
        return Err(());
    }

    let raw_eval = eval::eval(game);
    let eval = NegamaxEval::from_eval(raw_eval, game.player);

    if eval >= beta {
        return Ok(beta);
    }

    if eval > alpha {
        alpha = eval;
    }

    let mut moves = game.pseudo_legal_moves();

    // Only look at captures
    moves.retain(|m| game.board.piece_at(m.dst).is_some());

    move_ordering::order_moves(game, &mut moves, None);

    let mut best_eval = NegamaxEval::MIN;

    for mv in &moves {
        let player = game.player;

        // First, check if the move is legal.
        game.make_move(mv);

        if game.board.king_in_check(player) {
            game.undo_move();
            continue;
        }

        let move_score = -quiescence(game, -beta, -alpha, plies + 1, time_control, state, control)?;

        game.undo_move();

        if move_score > best_eval {
            best_eval = move_score;
        }

        // Cutoff: This move is so good that our opponent won't let it be played.
        if move_score >= beta {
            state.beta_cutoffs += 1;
            return Ok(beta);
        }

        if move_score > alpha {
            alpha = move_score;
        }
    }

    Ok(alpha)
}
