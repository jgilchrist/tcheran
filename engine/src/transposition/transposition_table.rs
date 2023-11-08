use chess::moves::Move;
use std::mem::size_of;

use crate::search::NegamaxEval;
use chess::zobrist::ZobristHash;

pub trait TTOverwriteable {
    fn should_overwrite_with(&self, new: &Self) -> bool;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeBound {
    Exact,
    Upper,
    Lower,
}

pub struct TranspositionTable<T: Clone + TTOverwriteable> {
    data: Vec<Option<TranspositionTableEntry<T>>>,
    pub occupied: usize,
}

#[derive(Clone)]
pub struct TranspositionTableEntry<T: Clone + TTOverwriteable> {
    pub key: ZobristHash,
    pub data: T,
}

#[derive(Debug, Clone)]
pub struct SearchTranspositionTableData {
    pub bound: NodeBound,
    pub eval: NegamaxEval,
    pub depth: u8,
    pub best_move: Option<Move>,
}

impl TTOverwriteable for SearchTranspositionTableData {
    fn should_overwrite_with(&self, new: &Self) -> bool {
        // If the new node is exact, always store it
        if new.bound == NodeBound::Exact {
            return true;
        }

        // We don't want to overwrite Exact nodes as we use them to retrieve the PV.
        self.bound != NodeBound::Exact
    }
}

pub type SearchTranspositionTable = TranspositionTable<SearchTranspositionTableData>;

impl<T: Clone + TTOverwriteable> TranspositionTable<T> {
    // TODO: Allow specifying the size
    pub fn new(size_pow_2: u32) -> Self {
        let size_of_entry = size_of::<T>();
        let total_size_in_bytes = 2usize.pow(size_pow_2) * 1024 * 1024;
        let number_of_entries = total_size_in_bytes / size_of_entry;

        Self {
            data: vec![None; number_of_entries],
            occupied: 0,
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn get_entry_idx(&self, key: &ZobristHash) -> usize {
        // PERF: There's likely a more performant way to do this
        key.0 as usize % self.data.len()
    }

    #[allow(clippy::cast_precision_loss)] // This is just an approximation, so a loss of precision is fine
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[must_use]
    pub fn occupancy(&self) -> usize {
        let decimal = self.occupied as f32 / self.data.len() as f32;
        let permille = decimal * 1000.0;
        permille as usize
    }

    pub fn insert(&mut self, key: &ZobristHash, data: T) {
        let idx = self.get_entry_idx(key);

        // !: We know the exact size of the table and will always access within the bounds.
        unsafe {
            let maybe_existing_data = self.data.get_unchecked(idx);
            if maybe_existing_data.is_none() {
                self.occupied += 1;
            }

            if let Some(existing_data) = maybe_existing_data {
                if existing_data.data.should_overwrite_with(&data) {
                    self.data[idx] = Some(TranspositionTableEntry {
                        key: key.clone(),
                        data,
                    });
                }
            } else {
                self.data[idx] = Some(TranspositionTableEntry {
                    key: key.clone(),
                    data,
                });
            }
        }
    }

    #[must_use]
    pub fn get(&self, key: &ZobristHash) -> Option<&T> {
        let idx = self.get_entry_idx(key);

        // !: We know the exact size of the table and will always access within the bounds.
        unsafe {
            if let Some(entry) = self.data.get_unchecked(idx) {
                if entry.key == *key {
                    return Some(&entry.data);
                }
            }
        }

        None
    }
}
