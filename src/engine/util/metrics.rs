use std::time::Duration;

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    reason = "This is intended to be approximate so we don't care about this being lossy"
)]
pub fn nodes_per_second(nodes: u64, elapsed_time: Duration) -> u64 {
    (nodes as f64 / elapsed_time.as_secs_f64()) as u64
}
