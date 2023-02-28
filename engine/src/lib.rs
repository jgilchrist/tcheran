#![feature(return_position_impl_trait_in_trait)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    incomplete_features,
    clippy::wildcard_imports,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
    clippy::inline_always
)]

pub mod log;
pub mod perft;
pub mod strategy;
pub mod uci;

mod eval;
mod search;
mod sync;
