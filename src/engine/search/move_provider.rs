use crate::chess::game::Game;
use crate::chess::movegen;
use crate::chess::movegen::MovegenCache;
use crate::chess::movelist::MoveList;
use crate::chess::moves::Move;
use crate::engine::search::move_ordering::score_move;
use crate::engine::search::{PersistentState, SearchState};

const MAX_MOVES: usize = u8::MAX as usize;

#[derive(Eq, PartialEq)]
enum GenStage {
    BestMove,
    GenCaptures,
    Captures,
    GenQuiets,
    Killer1,
    Killer2,
    ScoreQuiets,
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

    pub fn next(
        &mut self,
        game: &Game,
        persistent_state: &PersistentState,
        state: &SearchState,
        plies: usize,
    ) -> Option<Move> {
        if self.stage == GenStage::BestMove {
            self.stage = GenStage::GenCaptures;

            if let Some(previous_best_move) = self.previous_best_move {
                return Some(previous_best_move);
            }
        }

        if self.stage == GenStage::GenCaptures {
            self.stage = GenStage::Captures;

            movegen::generate_captures(game, &mut self.moves, &mut self.movegencache);

            self.captures_end = self.moves.len();
            self.score_moves(0, self.captures_end, game, persistent_state);
        }

        if self.stage == GenStage::Captures {
            if let Some(mv) = self.next_best_move(self.captures_end) {
                return Some(mv);
            }

            self.stage = if self.only_captures {
                GenStage::Done
            } else {
                GenStage::GenQuiets
            };
        }

        if self.stage == GenStage::GenQuiets {
            self.stage = GenStage::Killer1;

            movegen::generate_quiets(game, &mut self.moves, &self.movegencache);
        }

        if self.stage == GenStage::Killer1 {
            self.stage = GenStage::Killer2;

            if let Some(killer1) = state.killer_moves[plies][0] {
                for i in self.idx..self.moves.len() {
                    if self.moves.get(i) == killer1 {
                        self.moves.swap(self.idx, i);
                        self.idx += 1;

                        if Some(killer1) != self.previous_best_move {
                            return Some(killer1);
                        }
                    }
                }
            }
        }

        if self.stage == GenStage::Killer2 {
            self.stage = GenStage::ScoreQuiets;

            if let Some(killer2) = state.killer_moves[plies][1] {
                for i in self.idx..self.moves.len() {
                    if self.moves.get(i) == killer2 {
                        self.moves.swap(self.idx, i);
                        self.idx += 1;

                        if Some(killer2) != self.previous_best_move {
                            return Some(killer2);
                        }
                    }
                }
            }
        }

        if self.stage == GenStage::ScoreQuiets {
            self.stage = GenStage::Quiets;
            self.score_moves(self.idx, self.moves.len(), game, persistent_state);
        }

        if self.stage == GenStage::Quiets {
            if let Some(mv) = self.next_best_move(self.moves.len()) {
                return Some(mv);
            }

            self.stage = GenStage::Done;
        }

        if self.stage == GenStage::Done {
            return None;
        }

        unreachable!()
    }

    fn next_best_move(&mut self, limit: usize) -> Option<Move> {
        loop {
            if self.idx == limit {
                return None;
            }

            // Start with the next move that we haven't tried yet
            let mut best_move_idx = self.idx;
            let mut best_move_score = self.scores[self.idx];

            // Check if there's a better move later on in the list
            for i in self.idx + 1..limit {
                let move_score = self.scores[i];

                if move_score > best_move_score {
                    best_move_score = move_score;
                    best_move_idx = i;
                }
            }

            let best_move = self.moves.get(best_move_idx);

            // Move our best move to the start of the moves we haven't tried
            self.moves.swap(self.idx, best_move_idx);
            self.scores.swap(self.idx, best_move_idx);

            self.idx += 1;

            // We always return the best move first, before doing move generation.
            // We don't want to return it again from the movelist, so skip it.
            if Some(best_move) == self.previous_best_move {
                continue;
            }

            return Some(best_move);
        }
    }

    fn score_moves(
        &mut self,
        start: usize,
        end: usize,
        game: &Game,
        persistent_state: &PersistentState,
    ) {
        for i in start..end {
            self.scores[i] = score_move(
                game,
                self.moves.get(i),
                &persistent_state.history[game.player.array_idx()],
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::game::Game;
    use crate::chess::square::squares::all::*;

    #[test]
    fn test_moveprovider_does_not_double_yield_best_move() {
        crate::init();

        let game = Game::new();

        let mut moves: Vec<Move> = Vec::new();
        let mut move_provider = MoveProvider::new(Some((G1, F3).into()));

        let search_state = SearchState::new();
        let persistent_state = PersistentState::new();

        while let Some(m) = move_provider.next(&game, &persistent_state, &search_state, 0) {
            moves.push(m);
        }

        assert_eq!(moves.len(), 20);
    }
}
