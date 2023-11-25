use chess::moves::Move;
use chess::piece::PromotionPieceKind;
use chess::square::Square;

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
    size: usize,
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
    pub best_move: Option<TTMove>,
}

#[derive(Debug, Clone)]
pub struct TTMove {
    start_square_idx: u8,
    end_square_idx: u8,
    promotion: Option<PromotionPieceKind>,
}

impl TTMove {
    pub fn from_move(mv: &Move) -> Self {
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

impl TTOverwriteable for SearchTranspositionTableData {
    fn should_overwrite_with(&self, new: &Self) -> bool {
        // Always store new PV nodes
        if new.bound == NodeBound::Exact {
            return true;
        }

        // Try to keep old 'exact' nodes over new bound nodes
        // TODO: Don't keep exact nodes from previous searches
        self.bound != NodeBound::Exact
    }
}

pub type SearchTranspositionTable = TranspositionTable<SearchTranspositionTableData>;

impl<T: Clone + TTOverwriteable> TranspositionTable<T> {
    pub fn new() -> Self {
        Self {
            data: vec![None; 0],
            occupied: 0,
            size: 0,
        }
    }

    pub fn resize(&mut self, size_mb: usize) {
        if self.size == size_mb {
            return;
        }

        let size_of_entry = std::mem::size_of::<TranspositionTableEntry<T>>();
        let total_size_in_bytes = size_mb * 1024 * 1024;
        let number_of_entries = total_size_in_bytes / size_of_entry;

        self.data = vec![None; number_of_entries];
        self.size = size_mb;
        self.occupied = 0;
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

impl<T: Clone + TTOverwriteable> Default for TranspositionTable<T> {
    fn default() -> Self {
        Self::new()
    }
}
