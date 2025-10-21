use crate::{
    chess::moves::Move,
    engine::{
        transposition_table,
        transposition_table::{TTOverwriteable, TranspositionTable, TranspositionTableEntry},
    },
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
    pub eval: i16,
    pub depth: u8,
    pub age: u8,
    pub best_move: Option<Move>,
}

const _ASSERT_TT_DATA_SIZE: () = assert!(
    size_of::<TranspositionTableEntry<SearchTranspositionTableData>>() == 16,
    "Transposition table entry size changed"
);

const _ASSERT_TT_NUMBER_OF_ENTRIES: () = assert!(
    transposition_table::calculate_number_of_entries::<SearchTranspositionTableData>(256)
        == 16_777_216,
    "Transposition table expected number of entries changed"
);

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
