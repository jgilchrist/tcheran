use crate::chess::game::Game;
use crate::chess::movegen;
use crate::chess::moves::Move;
use crate::engine::search::move_ordering::score_move;

const MAX_MOVES: usize = u8::MAX as usize;

#[derive(Eq, PartialEq)]
enum GenStage {
    BestMove,
    GenCaptures,
    Captures,
    GenQuiets,
    Quiets,
    Done,
}

pub struct MoveProvider {
    moves: Vec<Move>,
    scores: [i32; MAX_MOVES],
    previous_best_move: Option<Move>,
    only_captures: bool,

    stage: GenStage,
    idx: usize,
    captures_end: usize,
}

impl MoveProvider {
    pub fn new(previous_best_move: Option<Move>) -> Self {
        Self {
            moves: vec![],
            scores: [0; MAX_MOVES],
            previous_best_move,
            only_captures: false,

            stage: GenStage::BestMove,
            idx: 0,
            captures_end: 0,
        }
    }

    pub fn new_loud(previous_best_move: Option<Move>) -> Self {
        Self {
            moves: vec![],
            scores: [0; MAX_MOVES],
            previous_best_move,
            only_captures: true,

            stage: GenStage::BestMove,
            idx: 0,
            captures_end: 0,
        }
    }

    pub fn next(&mut self, game: &Game) -> Option<Move> {
        if self.stage == GenStage::BestMove {
            self.stage = GenStage::GenCaptures;

            if let Some(previous_best_move) = self.previous_best_move {
                return Some(previous_best_move);
            }
        }

        if self.stage == GenStage::GenCaptures {
            self.stage = GenStage::Captures;

            let captures = movegen::generate_captures(game);
            self.moves.extend_from_slice(&captures);

            for (idx, mv) in self.moves.iter().enumerate() {
                self.scores[idx] = score_move(game, *mv, self.previous_best_move);
            }

            self.captures_end = self.moves.len();
        }

        if self.stage == GenStage::Captures {
            if self.idx == self.captures_end {
                self.stage = if self.only_captures {
                    GenStage::Done
                } else {
                    GenStage::GenQuiets
                }
            } else {
                let mut best_move_score = self.scores[self.idx];
                let mut best_move = self.moves[self.idx];
                let mut best_move_idx = self.idx;

                for i in self.idx + 1..self.captures_end {
                    let move_score = self.scores[i];
                    let mv = self.moves[i];

                    if move_score > best_move_score && Some(mv) != self.previous_best_move {
                        best_move_score = move_score;
                        best_move = mv;
                        best_move_idx = i;
                    }
                }

                if self.idx != best_move_idx {
                    self.moves.swap(self.idx, best_move_idx);
                    self.scores.swap(self.idx, best_move_idx);
                }

                self.idx += 1;

                return Some(best_move);
            }
        }

        if self.stage == GenStage::GenQuiets {
            self.stage = GenStage::Quiets;

            let quiets = movegen::generate_quiets(game);
            self.moves.extend_from_slice(&quiets);

            for i in self.captures_end..self.moves.len() {
                self.scores[i] = score_move(game, self.moves[i], self.previous_best_move);
            }
        }

        if self.stage == GenStage::Quiets {
            if self.idx >= self.moves.len() {
                self.stage = GenStage::Done;
            } else {
                let mut best_move_score = self.scores[self.idx];
                let mut best_move = self.moves[self.idx];
                let mut best_move_idx = self.idx;

                for i in self.idx + 1..self.moves.len() {
                    let move_score = self.scores[i];
                    let mv = self.moves[i];

                    if move_score > best_move_score && Some(mv) != self.previous_best_move {
                        best_move_score = move_score;
                        best_move = mv;
                        best_move_idx = i;
                    }
                }

                if self.idx != best_move_idx {
                    self.moves.swap(self.idx, best_move_idx);
                    self.scores.swap(self.idx, best_move_idx);
                }

                self.idx += 1;

                return Some(best_move);
            }
        }

        if self.stage == GenStage::Done {
            return None;
        }

        None
    }
}
