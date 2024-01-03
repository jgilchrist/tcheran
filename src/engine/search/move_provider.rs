use crate::chess::game::Game;
use crate::chess::moves::Move;
use crate::engine::search::move_ordering::score_move;

const MAX_MOVES: usize = u8::MAX as usize;

pub struct MoveProvider {
    moves: Vec<Move>,
    scores: [i32; MAX_MOVES],
    previous_best_move: Option<Move>,
    killer_moves: [Option<Move>; 2],

    move_idx: usize,
    scored_moves: bool,
}

impl MoveProvider {
    pub fn new(
        game: &Game,
        previous_best_move: Option<Move>,
        killer_moves: [Option<Move>; 2],
    ) -> Self {
        let moves = game.moves();

        Self {
            moves,
            scores: [0; MAX_MOVES],
            previous_best_move,
            killer_moves,

            move_idx: 0,
            scored_moves: false,
        }
    }

    pub fn new_loud(game: &Game) -> Self {
        let moves = game.loud_moves();

        Self {
            moves,
            scores: [0; MAX_MOVES],
            previous_best_move: None,
            killer_moves: [None; 2],

            move_idx: 0,
            scored_moves: false,
        }
    }

    pub fn next(&mut self, game: &Game) -> Option<Move> {
        if !self.scored_moves {
            self.score_moves(game);
            self.scored_moves = true;
        }

        if self.move_idx == self.moves.len() {
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

    fn score_moves(&mut self, game: &Game) {
        for (i, mv) in self.moves.iter().enumerate() {
            self.scores[i] = score_move(game, *mv, self.previous_best_move, self.killer_moves);
        }
    }
}
