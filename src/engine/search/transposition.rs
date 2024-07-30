use crate::chess::moves::Move;
use crate::chess::piece::PromotionPieceKind;
use crate::chess::square::Square;
use crate::engine::eval::Eval;
use crate::engine::transposition_table::{
    TTOverwriteable, TranspositionTable, TranspositionTableEntry,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeBound {
    Exact,
    Upper,
    Lower,
}

#[derive(Debug, Clone)]
pub struct SearchTranspositionTableData {
    pub bound: NodeBound,
    pub eval: Eval,
    pub depth: u8,
    pub age: u8,
    pub best_move: Option<TTMove>,
}

#[derive(Debug, Clone)]
pub struct TTMove {
    start_square_idx: u8,
    end_square_idx: u8,
    promotion: Option<PromotionPieceKind>,
}

impl TTMove {
    pub fn from_move(mv: Move) -> Self {
        Self {
            start_square_idx: mv.src.idx(),
            end_square_idx: mv.dst.idx(),
            promotion: mv.promotion,
        }
    }

    pub fn to_move(&self) -> Move {
        Move {
            src: Square::from_index(self.start_square_idx),
            dst: Square::from_index(self.end_square_idx),
            promotion: self.promotion,
        }
    }
}

impl TTOverwriteable for TranspositionTableEntry<SearchTranspositionTableData> {
    fn should_overwrite_with(&self, new: &Self) -> bool {
        // Always prioritise results from new searches
        if new.data.age != self.data.age {
            return true;
        }

        // Always prefer results that have been searched to a higher depth,
        // since they're more accurate
        if new.data.depth > self.data.depth {
            return true;
        }

        // Prefer newer positions
        if new.key != self.key {
            return true;
        }

        // If the new node is exact, always store it
        if new.data.bound == NodeBound::Exact {
            return true;
        }

        // Don't overwrite exact nodes
        self.data.bound != NodeBound::Exact
    }
}

pub type SearchTranspositionTable = TranspositionTable<SearchTranspositionTableData>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::transposition_table;
    use crate::engine::transposition_table::TranspositionTableEntry;

    #[test]
    fn assert_tt_size() {
        assert_eq!(
            std::mem::size_of::<TranspositionTableEntry<SearchTranspositionTableData>>(),
            16
        );
    }

    #[test]
    fn assert_tt_entries_at_256mb() {
        let number_of_entries =
            transposition_table::calculate_number_of_entries::<SearchTranspositionTableData>(256);

        assert_eq!(number_of_entries, 16_777_216);
    }
}
