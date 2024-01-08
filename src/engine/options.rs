pub mod defaults {
    pub const HASH_SIZE: usize = 256;
    pub const ENABLE_LOGGING: bool = false;
}

#[derive(Debug, Clone)]
pub struct EngineOptions {
    pub hash_size: usize,
    pub enable_logging: bool,
}

impl Default for EngineOptions {
    fn default() -> Self {
        Self {
            hash_size: defaults::HASH_SIZE,
            enable_logging: defaults::ENABLE_LOGGING,
        }
    }
}
