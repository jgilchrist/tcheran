use chess::game::Game;
use chess::perft::perft;

use criterion::{black_box, Criterion};
pub fn bench_perft_startpos_5(c: &mut Criterion) {
    chess::init();

    c.bench_function("perft_startpos_5", |b| {
        b.iter(|| assert_eq!(black_box(perft(5, &Game::new())), 4_865_609))
    });
}
