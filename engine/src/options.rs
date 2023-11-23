pub mod defaults {
    pub const HASH_SIZE: usize = 256;
}

#[derive(Debug, Clone)]
pub struct EngineOptions {
    pub hash_size: usize,
}

impl Default for EngineOptions {
    fn default() -> Self {
        Self {
            hash_size: defaults::HASH_SIZE,
        }
    }
}
