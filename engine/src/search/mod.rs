use chess::{game::Game, moves::Move};

use crate::eval::Eval;

mod negamax;

pub struct SearchState {
    nodes_visited: i32,
    beta_cutoffs: i32,
}

impl SearchState {
    const fn new() -> Self {
        Self {
            nodes_visited: 0,
            beta_cutoffs: 0,
        }
    }
}

pub fn search(game: &Game) -> (Move, Eval) {
    let mut state = SearchState::new();

    let (best_move, eval) = negamax::negamax(game, 6, &mut state);

    println!("Best move: {:?} (eval {})", best_move, eval);
    println!("Stats:");
    println!("  Total nodes visited: {}", state.nodes_visited);
    println!("  Beta cutoffs: {}", state.beta_cutoffs);

    (best_move, eval)
}
