use crate::chess::game::Game;
use crate::chess::movelist::MoveList;
use crate::chess::moves::Move;
use crate::engine::search::move_ordering::score_move;

const MAX_MOVES: usize = u8::MAX as usize;

pub struct MoveProvider {
    scores: [i32; MAX_MOVES],
    previous_best_move: Option<Move>,
    killer_moves: [Option<Move>; 2],

    move_idx: usize,
    scored_moves: bool,
}

impl MoveProvider {
    pub fn new(
        game: &Game,
        moves: &mut MoveList,
        previous_best_move: Option<Move>,
        killer_moves: [Option<Move>; 2],
    ) -> Self {
        moves.clear();
        game.fill_moves(moves);

        Self {
            scores: [0; MAX_MOVES],
            previous_best_move,
            killer_moves,

            move_idx: 0,
            scored_moves: false,
        }
    }

    pub fn new_loud(game: &Game, moves: &mut MoveList, previous_best_move: Option<Move>) -> Self {
        moves.clear();
        game.fill_loud_moves(moves);

        Self {
            scores: [0; MAX_MOVES],
            previous_best_move,
            killer_moves: [None; 2],

            move_idx: 0,
            scored_moves: false,
        }
    }

    pub fn next(&mut self, game: &Game, moves: &mut MoveList) -> Option<Move> {
        if !self.scored_moves {
            self.score_moves(game, moves);
            self.scored_moves = true;
        }

        if self.move_idx == moves.len() {
            return None;
        }

        let mut best_move_score = self.scores[self.move_idx];
        let mut best_move = moves.get(self.move_idx);
        let mut best_move_idx = self.move_idx;

        for i in self.move_idx + 1..moves.len() {
            let move_score = self.scores[i];

            if move_score > best_move_score {
                best_move_score = move_score;
                best_move = moves.get(i);
                best_move_idx = i;
            }
        }

        if self.move_idx != best_move_idx {
            moves.swap(self.move_idx, best_move_idx);
            self.scores.swap(self.move_idx, best_move_idx);
        }

        self.move_idx += 1;
        Some(best_move)
    }

    fn score_moves(&mut self, game: &Game, moves: &mut MoveList) {
        for i in 0..moves.len() {
            self.scores[i] = score_move(
                game,
                moves.get(i),
                self.previous_best_move,
                self.killer_moves,
            );
        }
    }
}
