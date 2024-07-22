use crate::chess::moves::Move;
use crate::engine::search::MAX_SEARCH_DEPTH_SIZE;
use arrayvec::ArrayVec;

#[derive(Clone)]
pub struct PrincipalVariation(ArrayVec<Move, MAX_SEARCH_DEPTH_SIZE>);

impl PrincipalVariation {
    #[inline]
    pub const fn new() -> Self {
        Self(ArrayVec::new_const())
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    #[inline]
    pub fn push(&mut self, mv: Move, child_pv: &Self) {
        self.0.clear();
        self.0.push(mv);
        self.0
            .try_extend_from_slice(&child_pv.0)
            .expect("Could not construct PV");
    }

    #[inline]
    pub fn first(&self) -> Option<&Move> {
        self.0.first()
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl IntoIterator for PrincipalVariation {
    type Item = Move;
    type IntoIter = arrayvec::IntoIter<Self::Item, MAX_SEARCH_DEPTH_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::square::squares::all::*;

    #[test]
    fn test_new_pv_has_no_moves() {
        let pv = PrincipalVariation::new();
        assert_eq!(pv.len(), 0);
    }

    #[test]
    fn test_combining_pvs_works() {
        let pv = PrincipalVariation::new();

        let mut pv_2 = PrincipalVariation::new();
        pv_2.push(Move::quiet(A1, B1), &pv);

        let mut pv_3 = PrincipalVariation::new();
        pv_3.push(Move::quiet(C1, D1), &pv_2);

        let mut pv_4 = PrincipalVariation::new();
        pv_4.push(Move::quiet(E1, F1), &pv_3);

        assert_eq!(pv_4.len(), 3);
        assert_eq!(pv_4.0.get(0).unwrap().src(), E1);
        assert_eq!(pv_4.0.get(1).unwrap().src(), C1);
        assert_eq!(pv_4.0.get(2).unwrap().src(), A1);
    }
}
