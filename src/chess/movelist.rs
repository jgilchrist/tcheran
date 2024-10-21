use crate::chess::moves::Move;
use arrayvec::ArrayVec;

const MAX_LEGAL_MOVES: usize = 218;
pub type MoveList = ArrayVec<Move, MAX_LEGAL_MOVES>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_movelist_has_no_moves() {
        let movelist = MoveList::new();
        assert_eq!(movelist.len(), 0);
    }
}
