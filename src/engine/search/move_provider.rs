use crate::chess::game::Game;
use crate::chess::moves::Move;
use crate::engine::search::move_ordering::score_move;

const MAX_MOVES: usize = u8::MAX as usize;

pub struct MoveProvider {
    moves: Vec<Move>,
    scores: [i32; MAX_MOVES],
    previous_best_move: Option<Move>,

    move_idx: usize,
    initialised_moves: bool,
}

impl MoveProvider {
    pub fn new(game: &Game, previous_best_move: Option<Move>) -> Self {
        let moves = game.moves();

        Self {
            moves,
            scores: [0; MAX_MOVES],
            previous_best_move,

            move_idx: 0,
            initialised_moves: false,
        }
    }

    pub fn new_loud(game: &Game, previous_best_move: Option<Move>) -> Self {
        let moves = game.loud_moves();

        Self {
            moves,
            scores: [0; MAX_MOVES],
            previous_best_move,

            move_idx: 0,
            initialised_moves: false,
        }
    }

    pub fn next(&mut self, game: &Game) -> Option<Move> {
        if !self.initialised_moves {
            self.init(game);
            self.initialised_moves = true;
        }

        if self.move_idx >= self.moves.len() {
            return None;
        }

        let mut best_move_score = self.scores[self.move_idx];
        let mut best_move = self.moves[self.move_idx];
        let mut best_move_idx = self.move_idx;

        for i in self.move_idx + 1..self.moves.len() {
            let move_score = self.scores[i];

            if move_score > best_move_score {
                best_move_score = move_score;
                best_move = self.moves[i];
                best_move_idx = i;
            }
        }

        if self.move_idx != best_move_idx {
            self.moves.swap(self.move_idx, best_move_idx);
            self.scores.swap(self.move_idx, best_move_idx);
        }

        self.move_idx += 1;
        Some(best_move)
    }

    fn init(&mut self, game: &Game) {
        for (idx, mv) in self.moves.iter().enumerate() {
            self.scores[idx] = score_move(game, *mv, self.previous_best_move);
        }
    }
}
