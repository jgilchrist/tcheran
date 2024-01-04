use crate::chess::moves::Move;
use crate::chess::square::squares::all::*;

const MAX_LEGAL_MOVES: usize = 218;

#[derive(Clone)]
pub struct MoveList {
    moves: [Move; MAX_LEGAL_MOVES],
    length: usize,
}

impl MoveList {
    const EMPTY_MOVE: Move = Move::new(A1, A1);

    // perf: Inlining this into new() causes a fairly significant
    // performance regression
    const EMPTY_MOVELIST: Self = Self {
        moves: [Self::EMPTY_MOVE; MAX_LEGAL_MOVES],
        length: 0,
    };

    pub const fn new() -> Self {
        Self::EMPTY_MOVELIST
    }

    #[allow(unused)]
    pub fn clear(&mut self) {
        self.length = 0;
    }

    pub fn push(&mut self, mv: Move) {
        self.moves[self.length] = mv;
        self.length += 1;
    }

    pub fn swap(&mut self, idx1: usize, idx2: usize) {
        debug_assert!(idx1 < self.length && idx2 < self.length);
        self.moves.swap(idx1, idx2);
    }

    pub fn get(&self, idx: usize) -> Move {
        debug_assert!(idx < self.length);

        let mv = self.moves[idx];
        debug_assert!(mv.src != A1 || mv.dst != A1);

        mv
    }

    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.length
    }

    #[allow(unused)]
    pub fn has_moves(&self) -> bool {
        self.length > 0
    }

    pub fn to_vec(&self) -> Vec<Move> {
        self.moves[..self.length].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_movelist_has_no_moves() {
        let movelist = MoveList::new();
        assert!(false);
    }
}
