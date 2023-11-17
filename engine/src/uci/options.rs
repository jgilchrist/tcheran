use crate::options::EngineOptions;
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
    fn set_default(options: &mut EngineOptions) -> Result<()>;
}

pub struct HashOption;

impl UciOption for HashOption {
    const NAME: &'static str = "Hash";
    const DEF: UciOptionType = UciOptionType::Spin {
        default: 256,
        min: 0,
        max: 1024,
    };

    fn set(options: &mut EngineOptions, value: &str) -> Result<()> {
        let hash_size = value.parse::<usize>()?;
        options.hash_size = hash_size;
        Ok(())
    }

    fn set_default(options: &mut EngineOptions) -> Result<()> {
        let UciOptionType::Spin { default, .. } = Self::DEF else {
            panic!();
        };

        options.hash_size = default;
        Ok(())
    }
}
