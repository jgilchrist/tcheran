use crate::chess::game::Game;
use crate::chess::moves::Move;
use crate::engine::search::move_ordering::{score_move, ScoredMove};

pub struct MoveProvider {
    moves: Vec<ScoredMove>,
    previous_best_move: Option<Move>,
    killer_moves: [Option<Move>; 2],

    move_idx: usize,
    initialised_moves: bool,
}

impl MoveProvider {
    pub fn new(
        game: &Game,
        previous_best_move: Option<Move>,
        killer_moves: [Option<Move>; 2],
    ) -> Self {
        let moves = game.moves().into_iter().map(ScoredMove::new).collect();

        Self {
            moves,
            previous_best_move,
            killer_moves,

            move_idx: 0,
            initialised_moves: false,
        }
    }

    pub fn new_loud(game: &Game, previous_best_move: Option<Move>) -> Self {
        let moves = game.loud_moves().into_iter().map(ScoredMove::new).collect();

        Self {
            moves,
            previous_best_move,
            killer_moves: [None; 2],

            move_idx: 0,
            initialised_moves: false,
        }
    }

    pub fn next(&mut self, game: &Game) -> Option<Move> {
        if !self.initialised_moves {
            self.init(game);
            self.initialised_moves = true;
        }

        if self.move_idx == self.moves.len() {
            return None;
        }

        let next_best_move = &self.moves[self.move_idx];
        self.move_idx += 1;

        Some(next_best_move.mv)
    }

    fn init(&mut self, game: &Game) {
        // First, score each move
        for mv in &mut self.moves {
            mv.score = score_move(game, mv.mv, self.previous_best_move, self.killer_moves);
        }

        // Then, sort each move by its score
        self.moves.sort_unstable_by_key(|m| -m.score);
    }
}
