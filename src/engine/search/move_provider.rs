use crate::chess::game::Game;
use crate::chess::movegen;
use crate::chess::movegen::MovegenCache;
use crate::chess::movelist::MoveList;
use crate::chess::moves::Move;
use crate::engine::search::move_ordering::score_move;
use crate::engine::search::SearchState;

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
    moves: MoveList,
    movegencache: MovegenCache,
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
            moves: MoveList::new(),
            movegencache: MovegenCache::new(),
            scores: [0; MAX_MOVES],
            previous_best_move,
            only_captures: false,

            stage: GenStage::BestMove,
            idx: 0,
            captures_end: 0,
        }
    }

    pub fn new_loud() -> Self {
        Self {
            moves: MoveList::new(),
            movegencache: MovegenCache::new(),
            scores: [0; MAX_MOVES],
            previous_best_move: None,
            only_captures: true,

            stage: GenStage::BestMove,
            idx: 0,
            captures_end: 0,
        }
    }

    pub fn next(&mut self, game: &Game, state: &SearchState, plies: usize) -> Option<Move> {
        if self.stage == GenStage::BestMove {
            self.stage = GenStage::GenCaptures;

            if let Some(previous_best_move) = self.previous_best_move {
                return Some(previous_best_move);
            }
        }

        if self.stage == GenStage::GenCaptures {
            self.stage = GenStage::Captures;

            movegen::generate_captures(game, &mut self.moves, &mut self.movegencache);

            for i in 0..self.moves.len() {
                self.scores[i] = score_move(
                    game,
                    self.moves.get(i),
                    self.previous_best_move,
                    state.killer_moves[plies],
                    &state.history[game.player.array_idx()],
                );
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
                let mut best_move_idx = self.idx;
                let mut best_move = self.moves.get(self.idx);
                let mut best_move_score = self.scores[self.idx];

                for i in self.idx + 1..self.captures_end {
                    let mv = self.moves.get(i);
                    let move_score = self.scores[i];

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

            movegen::generate_quiets(game, &mut self.moves, &self.movegencache);

            for i in self.captures_end..self.moves.len() {
                self.scores[i] = score_move(
                    game,
                    self.moves.get(i),
                    self.previous_best_move,
                    state.killer_moves[plies],
                    &state.history[game.player.array_idx()],
                );
            }
        }

        if self.stage == GenStage::Quiets {
            if self.idx >= self.moves.len() {
                self.stage = GenStage::Done;
            } else {
                let mut best_move_score = self.scores[self.idx];
                let mut best_move = self.moves.get(self.idx);
                let mut best_move_idx = self.idx;

                for i in self.idx + 1..self.moves.len() {
                    let mv = self.moves.get(i);
                    let move_score = self.scores[i];

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
