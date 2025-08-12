static mut LMR_TABLE: [[u8; 64]; 64] = [[0; 64]; 64];

const LMR_BASE: f32 = 0.75;
const LMR_FACTOR: f32 = 2.25;

pub fn lmr_reduction(depth: u8, move_count: usize) -> u8 {
    let depth = depth as usize;
    unsafe { LMR_TABLE[depth.min(63)][move_count.min(63)] }
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    reason = "Calculation is intentionally approximate"
)]
pub fn init() {
    unsafe {
        for (depth, table) in LMR_TABLE.iter_mut().enumerate().skip(1) {
            for (move_count, reduction) in table.iter_mut().enumerate().skip(1) {
                *reduction = (LMR_BASE
                    + f32::ln(depth as f32) * f32::ln(move_count as f32) / LMR_FACTOR)
                    as u8;
            }
        }
    }
}
