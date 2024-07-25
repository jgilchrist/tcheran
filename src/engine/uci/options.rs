use crate::engine::options::EngineOptions;

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

    fn set(options: &mut EngineOptions, value: &str) -> Result<(), String>;
}

pub struct HashOption;

impl UciOption for HashOption {
    const NAME: &'static str = "Hash";
    const DEF: UciOptionType = UciOptionType::Spin {
        default: crate::engine::options::defaults::HASH_SIZE,
        min: 0,
        max: 1024,
    };

    fn set(options: &mut EngineOptions, value: &str) -> Result<(), String> {
        let hash_size = value.parse::<usize>().map_err(|_| "Invalid value")?;

        options.hash_size = hash_size;
        Ok(())
    }
}

pub struct ThreadsOption;

impl UciOption for ThreadsOption {
    const NAME: &'static str = "Threads";
    const DEF: UciOptionType = UciOptionType::Spin {
        default: crate::engine::options::defaults::THREADS,
        min: 1,
        max: 1,
    };

    fn set(options: &mut EngineOptions, value: &str) -> Result<(), String> {
        let threads = value.parse::<usize>().map_err(|_| "Invalid value")?;

        options.threads = threads;
        Ok(())
    }
}

pub struct MoveOverheadOption;

impl UciOption for MoveOverheadOption {
    const NAME: &'static str = "Move Overhead";
    const DEF: UciOptionType = UciOptionType::Spin {
        default: crate::engine::options::defaults::MOVE_OVERHEAD,
        min: 0,
        max: 1000,
    };

    fn set(options: &mut EngineOptions, value: &str) -> Result<(), String> {
        let move_overhead = value.parse::<usize>().map_err(|_| "Invalid value")?;

        options.move_overhead = move_overhead;
        Ok(())
    }
}
