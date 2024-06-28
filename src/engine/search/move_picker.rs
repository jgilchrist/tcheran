use crate::chess::game::Game;
use crate::chess::movegen;
use crate::chess::movegen::MovegenCache;
use crate::chess::movelist::MoveList;
use crate::chess::moves::Move;
use crate::engine::eval::Eval;
use crate::engine::search::move_ordering::score_move;
use crate::engine::search::{PersistentState, SearchState};
use crate::engine::see::see;

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

pub struct MovePicker {
    moves: MoveList,
    movegencache: MovegenCache,
    scores: [i32; MAX_MOVES],
    previous_best_move: Option<Move>,
    only_captures: bool,

    stage: GenStage,
    idx: usize,
    captures_end: usize,
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
        plies: u8,
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
            if let Some(mv) = self.next_best_move(game, self.captures_end, self.only_captures) {
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

            if let Some(killer1) = state.killer_moves.get_0(plies) {
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

            if let Some(killer2) = state.killer_moves.get_1(plies) {
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
            if let Some(mv) = self.next_best_move(game, self.moves.len(), false) {
                return Some(mv);
            }

            self.stage = GenStage::Done;
        }

        if self.stage == GenStage::Done {
            return None;
        }

        unreachable!()
    }

    fn next_best_move(
        &mut self,
        game: &Game,
        limit: usize,
        skip_bad_captures: bool,
    ) -> Option<Move> {
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

            if skip_bad_captures && !see(game, best_move, Eval(1)) {
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
            self.scores[i] = score_move(game, self.moves.get(i), &persistent_state.history_table);
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
}
