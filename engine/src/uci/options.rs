use crate::options::EngineOptions;
use anyhow::Result;

#[derive(Debug)]
#[allow(unused)]
pub enum UciOptionType {
    Check,
    Spin,
    Combo,
    String,
    Button,
}

pub trait UciOption {
    const NAME: &'static str;
    const TYPE: UciOptionType;
    const DEFAULT_VALUE: &'static str;

    fn set(options: &mut EngineOptions, value: &str) -> Result<()>;
}
