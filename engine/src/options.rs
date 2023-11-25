use crate::strategy::KnownStrategy;

pub mod defaults {
    use crate::strategy::KnownStrategy;

    pub const STRATEGY: KnownStrategy = KnownStrategy::Main;
    pub const HASH_SIZE: usize = 256;
}

#[derive(Debug, Clone)]
pub struct EngineOptions {
    pub strategy: KnownStrategy,
    pub hash_size: usize,
}

impl Default for EngineOptions {
    fn default() -> Self {
        Self {
            strategy: defaults::STRATEGY,
            hash_size: defaults::HASH_SIZE,
        }
    }
}
