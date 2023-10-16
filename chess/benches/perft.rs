use chess::game::Game;
use engine::perft::perft;
use std::time::Duration;

use criterion::{black_box, Criterion};

pub fn bench_perft_startpos_5(c: &mut Criterion) {
    engine::init();

    c.bench_function("perft_startpos_5", |b| {
        b.iter(|| assert_eq!(black_box(perft(5, &Game::new())), 4_865_609))
    });
}
