use crate::engine::strategy::KnownStrategy;

pub mod defaults {
    use crate::engine::strategy::KnownStrategy;

    pub const STRATEGY: KnownStrategy = KnownStrategy::Main;
    pub const HASH_SIZE: usize = 256;
    pub const ENABLE_LOGGING: bool = false;
}

#[derive(Debug, Clone)]
pub struct EngineOptions {
    pub strategy: KnownStrategy,
    pub hash_size: usize,
    pub enable_logging: bool,
}

impl Default for EngineOptions {
    fn default() -> Self {
        Self {
            strategy: defaults::STRATEGY,
            hash_size: defaults::HASH_SIZE,
            enable_logging: defaults::ENABLE_LOGGING,
        }
    }
}
