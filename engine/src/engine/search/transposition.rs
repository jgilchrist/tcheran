use crate::chess::moves::Move;
use crate::engine::eval::Eval;
use crate::engine::transposition_table::{TTOverwriteable, TranspositionTable};

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
    pub best_move: Option<Move>,
}

impl TTOverwriteable for SearchTranspositionTableData {
    fn should_overwrite_with(&self, new: &Self) -> bool {
        // Always prioritise results from new searches
        if new.age != self.age {
            return true;
        }

        // Always prefer results that have been searched to a higher depth,
        // since they're more accurate
        if new.depth > self.depth {
            return true;
        }

        // If the new node is exact, always store it
        if new.bound == NodeBound::Exact {
            return true;
        }

        // Don't overwrite exact nodes
        self.bound != NodeBound::Exact
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
