use chess::moves::Move;
use std::mem::size_of;

use crate::search::NegamaxEval;
use chess::zobrist::ZobristHash;

#[derive(Debug, Clone)]
pub enum NodeBound {
    Exact,
    Upper,
    Lower,
}

pub struct TranspositionTable<T: Clone> {
    data: Vec<Option<TranspositionTableEntry<T>>>,
    pub occupied: usize,
}

#[derive(Clone)]
pub struct TranspositionTableEntry<T: Clone> {
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

pub type SearchTranspositionTable = TranspositionTable<SearchTranspositionTableData>;

const TRANSPOSITION_TABLE_SIZE_POW_2: u32 = 8;

impl<T: Clone> TranspositionTable<T> {
    // TODO: Allow specifying the size
    pub fn new() -> Self {
        let size_of_entry = size_of::<T>();
        let total_size_in_bytes = 2usize.pow(TRANSPOSITION_TABLE_SIZE_POW_2) * 1024 * 1024;
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
    pub fn occupancy(&self) -> usize {
        let decimal = self.occupied as f32 / self.data.len() as f32;
        let permille = decimal * 1000.0;
        permille as usize
    }

    pub fn insert(&mut self, key: &ZobristHash, data: T) {
        let idx = self.get_entry_idx(key);

        // !: We know the exact size of the table and will always access within the bounds.
        unsafe {
            let existing_data = self.data.get_unchecked(idx);
            if existing_data.is_none() {
                self.occupied += 1;
            }
        }

        // TODO: Right now, we always replace for simplicity
        self.data[idx] = Some(TranspositionTableEntry {
            key: key.clone(),
            data,
        });
    }

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
