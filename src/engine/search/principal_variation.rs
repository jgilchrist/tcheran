use crate::chess::moves::Move;
use crate::chess::square::squares::all::*;
use crate::engine::search::MAX_SEARCH_DEPTH_SIZE;

#[derive(Clone)]
pub struct PrincipalVariation {
    moves: [Move; MAX_SEARCH_DEPTH_SIZE],
    length: usize,
}

impl PrincipalVariation {
    const EMPTY_MOVE: Move = Move::new(A1, A1);

    pub const fn new() -> Self {
        Self {
            moves: [Self::EMPTY_MOVE; MAX_SEARCH_DEPTH_SIZE],
            length: 0,
        }
    }

    pub fn clear(&mut self) {
        self.length = 0;
    }

    pub fn push(&mut self, mv: Move, child_pv: &Self) {
        self.length = child_pv.length + 1;
        self.moves[0] = mv;
        self.moves[1..self.length].copy_from_slice(child_pv.as_slice());
    }

    #[inline]
    pub fn as_slice(&self) -> &[Move] {
        &self.moves[0..self.length]
    }

    pub fn first(&self) -> Option<Move> {
        if self.length == 0 {
            None
        } else {
            Some(self.moves[0])
        }
    }

    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pv_has_no_moves() {
        let pv = PrincipalVariation::new();
        assert_eq!(pv.len(), 0);
    }

    #[test]
    fn test_combining_pvs_works() {
        let pv = PrincipalVariation::new();

        let mut pv_2 = PrincipalVariation::new();
        pv_2.push(Move::new(A1, B1), &pv);

        let mut pv_3 = PrincipalVariation::new();
        pv_3.push(Move::new(C1, D1), &pv_2);

        let mut pv_4 = PrincipalVariation::new();
        pv_4.push(Move::new(E1, F1), &pv_3);

        assert_eq!(pv_4.len(), 3);
        assert_eq!(pv_4.moves[0].src, E1);
        assert_eq!(pv_4.moves[1].src, C1);
        assert_eq!(pv_4.moves[2].src, A1);
    }
}
