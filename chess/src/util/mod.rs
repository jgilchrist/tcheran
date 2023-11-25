use std::time::Duration;

// This is an approximate calculations so ignoring all of the possible issues around
// precision loss here
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
pub fn nodes_per_second(nodes: u32, elapsed_time: Duration) -> u32 {
    (nodes as f32 / elapsed_time.as_secs_f32()) as u32
}
