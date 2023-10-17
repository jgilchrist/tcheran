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

pub struct MaxSearchDepthOption;

impl UciOption for MaxSearchDepthOption {
    const NAME: &'static str = "MaxSearchDepth";
    const TYPE: UciOptionType = UciOptionType::Spin;
    const DEFAULT_VALUE: &'static str = "6";

    fn set(options: &mut EngineOptions, value: &str) -> Result<()> {
        let new_value = value.parse::<u8>()?;
        options.max_search_depth = new_value;
        Ok(())
    }
}
