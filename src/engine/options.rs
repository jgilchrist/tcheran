pub mod defaults {
    pub const HASH_SIZE: usize = 256;
    pub const THREADS: usize = 1;
    pub const MOVE_OVERHEAD: usize = 0;
}

#[derive(Debug, Clone)]
pub struct EngineOptions {
    pub hash_size: usize,
    pub threads: usize,

    // Account for the possibility that there's some overhead making the move
    // e.g. sending the best move over the internet.
    pub move_overhead: usize,
}

impl Default for EngineOptions {
    fn default() -> Self {
        Self {
            hash_size: defaults::HASH_SIZE,
            threads: defaults::THREADS,
            move_overhead: defaults::MOVE_OVERHEAD,
        }
    }
}
