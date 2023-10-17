#[derive(Debug, Clone)]
pub struct EngineOptions {
    pub max_search_depth: u8,
}

impl Default for EngineOptions {
    fn default() -> Self {
        Self {
            max_search_depth: 6,
        }
    }
}
