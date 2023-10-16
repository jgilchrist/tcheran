use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

pub mod perft;

criterion_group!(
    name = perft;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(100))
        .sample_size(10);
    targets = perft::bench_perft_startpos_5
);

criterion_main!(perft);
