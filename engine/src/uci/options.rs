use crate::options::EngineOptions;
use crate::strategy::KnownStrategy;
use color_eyre::eyre::bail;
use color_eyre::Result;

#[derive(Debug)]
#[allow(unused)]
pub enum UciOptionType {
    Check {
        default: bool,
    },
    Spin {
        default: usize,
        min: usize,
        max: usize,
    },
    Combo {
        default: &'static str,
        values: Vec<&'static str>,
    },
    String {
        default: &'static str,
    },
    Button,
}

pub trait UciOption {
    const NAME: &'static str;
    const DEF: UciOptionType;

    fn set(options: &mut EngineOptions, value: &str) -> Result<()>;
}

pub struct StrategyOption;

impl UciOption for StrategyOption {
    const NAME: &'static str = "Strategy";
    const DEF: UciOptionType = UciOptionType::String {
        default: crate::options::defaults::STRATEGY.to_string(),
    };

    fn set(options: &mut EngineOptions, value: &str) -> Result<()> {
        let strategy = match value {
            "Default" | "Main" => KnownStrategy::Main,
            "Random" => KnownStrategy::Random,
            "TopEval" => KnownStrategy::TopEval,
            _ => {
                bail!("Invalid strategy name: {}", value);
            }
        };

        options.strategy = strategy;
        Ok(())
    }
}

pub struct HashOption;

impl UciOption for HashOption {
    const NAME: &'static str = "Hash";
    const DEF: UciOptionType = UciOptionType::Spin {
        default: crate::options::defaults::HASH_SIZE,
        min: 0,
        max: 1024,
    };

    fn set(options: &mut EngineOptions, value: &str) -> Result<()> {
        let hash_size = value.parse::<usize>()?;
        options.hash_size = hash_size;
        Ok(())
    }
}
