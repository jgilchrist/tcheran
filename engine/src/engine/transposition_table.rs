use crate::chess::zobrist::ZobristHash;

pub trait TTOverwriteable {
    fn should_overwrite_with(&self, new: &Self) -> bool;
}

pub struct TranspositionTable<T: Clone + TTOverwriteable> {
    data: Vec<Option<TranspositionTableEntry<T>>>,
    pub generation: u8,
    pub occupied: usize,
    size: usize,
}

#[derive(Clone)]
pub struct TranspositionTableEntry<T: Clone + TTOverwriteable> {
    pub key: ZobristHash,
    pub data: T,
}

pub fn calculate_number_of_entries<T: Clone + TTOverwriteable>(size_mb: usize) -> usize {
    let size_of_entry = std::mem::size_of::<TranspositionTableEntry<T>>();
    let total_size_in_bytes = size_mb * 1024 * 1024;
    total_size_in_bytes / size_of_entry
}

impl<T: Clone + TTOverwriteable> TranspositionTable<T> {
    pub fn new(size_mb: usize) -> Self {
        let mut tt = Self {
            data: Vec::new(),
            size: 0,
            occupied: 0,
            generation: 0,
        };

        tt.resize(size_mb);
        tt
    }

    pub fn reset(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = None;
        }

        self.generation = 0;
        self.occupied = 0;
    }

    pub fn resize(&mut self, size_mb: usize) {
        if self.size == size_mb {
            return;
        }

        let number_of_entries = calculate_number_of_entries::<T>(size_mb);

        self.data.clear();
        self.data.resize(number_of_entries, None);
        self.data.shrink_to_fit();
        self.size = size_mb;
        self.occupied = 0;
        self.generation = 0;
    }

    pub fn new_generation(&mut self) {
        self.generation += 1;
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "The truncation is intended to get an index"
    )]
    fn get_entry_idx(&self, key: &ZobristHash) -> usize {
        // PERF: There's likely a more performant way to do this
        key.0 as usize % self.data.len()
    }

    #[expect(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "This is just an approximation, so a loss of precision is fine"
    )]
    pub fn occupancy(&self) -> usize {
        let decimal = self.occupied as f32 / self.data.len() as f32;
        let permille = decimal * 1000.0;
        permille as usize
    }

    pub fn insert(&mut self, key: &ZobristHash, data: T) {
        let idx = self.get_entry_idx(key);

        // !: We know the exact size of the table and will always access within the bounds.
        unsafe {
            if let Some(existing_data) = self.data.get_unchecked(idx) {
                if existing_data.data.should_overwrite_with(&data) {
                    self.data[idx] = Some(TranspositionTableEntry {
                        key: key.clone(),
                        data,
                    });
                }
            } else {
                self.occupied += 1;

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
