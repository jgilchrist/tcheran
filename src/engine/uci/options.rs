use crate::engine::options::EngineOptions;
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

pub struct HashOption;

impl UciOption for HashOption {
    const NAME: &'static str = "Hash";
    const DEF: UciOptionType = UciOptionType::Spin {
        default: crate::engine::options::defaults::HASH_SIZE,
        min: 0,
        max: 1024,
    };

    fn set(options: &mut EngineOptions, value: &str) -> Result<()> {
        let hash_size = value.parse::<usize>()?;
        options.hash_size = hash_size;
        Ok(())
    }
}

pub struct LogOption;

impl UciOption for LogOption {
    const NAME: &'static str = "Log";

    const DEF: UciOptionType = UciOptionType::Check {
        default: crate::engine::options::defaults::ENABLE_LOGGING,
    };

    fn set(options: &mut EngineOptions, value: &str) -> Result<()> {
        let should_enable_logging = value.parse::<bool>()?;
        options.enable_logging = should_enable_logging;
        crate::engine::util::log::set_logging_enabled(options.enable_logging);
        Ok(())
    }
}
