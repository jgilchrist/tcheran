use chess::{game::Game, moves::Move, player::Player};

use crate::eval::{Eval, self};

use super::SearchState;

pub fn negamax(game: &Game, depth: u8, state: &mut SearchState) -> (Move, Eval) {
    let mut best_move: Option<Move> = None;
    let mut best_score = Eval::MIN;
    
    let root_moves = game.legal_moves();
    for mv in &root_moves {
        let game_after_move = game.make_move(mv).unwrap();
        state.nodes_visited += 1;

        let move_score = -negamax_inner(&game_after_move, Eval::MIN, Eval::MAX, depth - 1, state);
        println!("Candidate {mv} - eval={move_score} best={best_score}");

        if move_score > best_score {
            best_score = move_score;
            best_move = Some(*mv);
        }
    }

    (best_move.unwrap(), best_score)
}

fn negamax_inner(game: &Game, mut alpha: Eval, beta: Eval, depth: u8, state: &mut SearchState) -> Eval {
    // TODO: Quiescence search
    // TODO: Check if game is over
    if depth == 0 {
        state.nodes_visited += 1;

        let multiplier: i32 = match game.player {
            chess::player::Player::White => 1,
            chess::player::Player::Black => -1,
        };

        let leaf_eval = eval::eval(game) * multiplier;

        return leaf_eval;
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
