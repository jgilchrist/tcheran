#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::wildcard_imports)]

pub mod log;
pub mod strategy;
pub mod uci;

mod eval;
mod search;
