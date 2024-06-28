use crate::chess::game::Game;
use crate::chess::movegen;
use crate::chess::movegen::MovegenCache;
use crate::chess::movelist::MoveList;
use crate::chess::moves::Move;
use crate::engine::search::move_ordering::score_move;
use crate::engine::search::move_picker::GenStage::BadCaptures;
use crate::engine::search::{move_ordering, PersistentState, SearchState};
use GenStage::Done;

const MAX_MOVES: usize = u8::MAX as usize;

#[derive(Eq, PartialEq)]
enum GenStage {
    BestMove,
    GenCaptures,
    GoodCaptures,
    GenQuiets,
    BadCaptures,
    ScoreQuiets,
    Quiets,
    Done,
}

pub struct MovePicker {
    moves: MoveList,
    movegencache: MovegenCache,
    scores: [i32; MAX_MOVES],
    previous_best_move: Option<Move>,
    only_captures: bool,

    stage: GenStage,
    idx: usize,
    captures_end: usize,
    first_bad_capture: Option<usize>,
}

impl MovePicker {
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
            first_bad_capture: None,
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
            first_bad_capture: None,
        }
    }

    pub fn next(
        &mut self,
        game: &Game,
        persistent_state: &PersistentState,
        state: &SearchState,
        plies: u8,
    ) -> Option<Move> {
        if self.stage == GenStage::BestMove {
            self.stage = GenStage::GenCaptures;

            if let Some(previous_best_move) = self.previous_best_move {
                return Some(previous_best_move);
            }
        }

        if self.stage == GenStage::GenCaptures {
            self.stage = GenStage::GoodCaptures;

            movegen::generate_captures(game, &mut self.moves, &mut self.movegencache);

            self.captures_end = self.moves.len();
            self.score_moves(0, self.captures_end, plies, game, state, persistent_state);
        }

        if self.stage == GenStage::GoodCaptures {
            if let Some((mv, score)) = self.next_best_move(self.captures_end) {
                // If the move we just picked was a losing capture, we're going to skip the rest of the captures.
                // Record that, and skip the remainder of the captures since we'll be trying quiet moves next.
                if score < move_ordering::GOOD_CAPTURE_SCORE {
                    self.first_bad_capture = Some(self.idx - 1);
                    self.idx = self.captures_end;
                } else {
                    return Some(mv);
                }
            }

            if self.only_captures {
                self.stage = Done;
                // match self.first_bad_capture {
                //     // If we didn't see any bad captures before, we can skip straight to the end
                //     None => self.stage = Done,
                //
                //     // If we saw any bad captures, go back and try those too
                //     Some(first_bad_capture_idx) => {
                //         self.idx = first_bad_capture_idx;
                //         self.stage = GenStage::BadCaptures;
                //     }
                // }
            } else {
                self.stage = GenStage::GenQuiets;
            };
        }

        if self.stage == GenStage::GenQuiets {
            self.stage = GenStage::ScoreQuiets;

            movegen::generate_quiets(game, &mut self.moves, &self.movegencache);
        }

        if self.stage == GenStage::ScoreQuiets {
            self.stage = GenStage::Quiets;
            self.score_moves(
                self.idx,
                self.moves.len(),
                plies,
                game,
                state,
                persistent_state,
            );
        }

        if self.stage == GenStage::Quiets {
            if let Some((mv, _)) = self.next_best_move(self.moves.len()) {
                return Some(mv);
            }

            match self.first_bad_capture {
                None => self.stage = Done,
                Some(first_bad_capture_idx) => {
                    self.idx = first_bad_capture_idx;
                    self.stage = BadCaptures;
                }
            }
        }

        if self.stage == GenStage::BadCaptures {
            if let Some((mv, _)) = self.next_best_move(self.captures_end) {
                return Some(mv);
            }

            self.stage = GenStage::Done;
        }

        if self.stage == Done {
            return None;
        }

        unreachable!()
    }

    fn next_best_move(&mut self, limit: usize) -> Option<(Move, i32)> {
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

            let this_best_move = Some(best_move);

            // The TT move and the killers are handled separately, so skip them if we find them
            // here
            if this_best_move == self.previous_best_move {
                continue;
            }

            return Some((best_move, best_move_score));
        }
    }

    fn score_moves(
        &mut self,
        start: usize,
        end: usize,
        plies: u8,
        game: &Game,
        state: &SearchState,
        persistent_state: &PersistentState,
    ) {
        for i in start..end {
            self.scores[i] = score_move(
                game,
                self.moves.get(i),
                plies,
                &state.killer_moves,
                &persistent_state.history_table,
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
    fn test_movepicker_does_not_double_yield_best_move() {
        crate::init();

        let game = Game::new();

        let mut moves: Vec<Move> = Vec::new();
        let mut move_picker = MovePicker::new(Some((G1, F3).into()));

        let search_state = SearchState::new();
        let persistent_state = PersistentState::new();

        while let Some(m) = move_picker.next(&game, &persistent_state, &search_state, 0) {
            moves.push(m);
        }

        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn test_movepicker_does_not_skip_bad_captures_when_no_good_captures() {
        crate::init();

        let game = Game::from_fen("rnbqkbnr/pp1ppppp/8/2p5/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 2")
            .unwrap();

        let mut moves: Vec<Move> = Vec::new();
        let mut move_provider = MovePicker::new(None);

        let search_state = SearchState::new();
        let persistent_state = PersistentState::new();

        while let Some(m) = move_provider.next(&game, &persistent_state, &search_state, 0) {
            moves.push(m);
        }

        assert_eq!(moves.len(), 23);
    }

    #[test]
    fn test_movepicker_does_not_return_to_start_if_no_bad_captures() {
        crate::init();

        let game =
            Game::from_fen("rnbqkb1r/ppp1pppp/5n2/3p4/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 3")
                .unwrap();

        let mut moves: Vec<Move> = Vec::new();
        let mut move_provider = MovePicker::new(None);

        let search_state = SearchState::new();
        let persistent_state = PersistentState::new();

        while let Some(m) = move_provider.next(&game, &persistent_state, &search_state, 0) {
            moves.push(m);
        }

        assert_eq!(moves.len(), 33);
    }

    #[test]
    fn test_movepicker_yields_en_passant_correctly() {
        crate::init();

        let game =
            Game::from_fen("r1bqkb1r/ppp1pppp/2n2n2/2Pp4/8/5N2/PP1PPPPP/RNBQKB1R w KQkq d6 0 4")
                .unwrap();

        let mut moves: Vec<Move> = Vec::new();
        let mut move_provider = MovePicker::new(None);

        let search_state = SearchState::new();
        let persistent_state = PersistentState::new();

        while let Some(m) = move_provider.next(&game, &persistent_state, &search_state, 0) {
            moves.push(m);
        }

        assert_eq!(moves.len(), 24);
    }

    #[test]
    fn test_movepicker_generates_caps_in_quiescence() {
        crate::init();

        let game =
            Game::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap();

        let mut moves: Vec<Move> = Vec::new();
        let mut move_provider = MovePicker::new_loud();

        let search_state = SearchState::new();
        let persistent_state = PersistentState::new();

        while let Some(m) = move_provider.next(&game, &persistent_state, &search_state, 0) {
            moves.push(m);
        }

        assert_eq!(moves.len(), 1);
    }

    #[test]
    fn test_movepicker_bug_after_see_move_ordering_1() {
        crate::init();

        let game = Game::from_fen("r2k3r/1b4bq/8/3R4/8/8/7B/4K2R b K - 3 2").unwrap();

        let mut moves: Vec<Move> = Vec::new();
        let mut move_provider = MovePicker::new(Some(Move::new(D8, E7)));

        let mut search_state = SearchState::new();
        let persistent_state = PersistentState::new();

        search_state.killer_moves.try_push(0, Move::new(B7, D5));
        search_state.killer_moves.try_push(0, Move::new(D8, E8));

        while let Some(m) = move_provider.next(&game, &persistent_state, &search_state, 0) {
            moves.push(m);
        }

        assert_eq!(moves.len(), 4);
    }
}
