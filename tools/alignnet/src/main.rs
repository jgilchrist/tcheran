use std::{fs, process::ExitCode, slice};

const FEATURES: usize = 768;
const HIDDEN_SIZE: usize = 256;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct UnalignedAccumulator([i16; HIDDEN_SIZE]);

#[repr(C)]
struct UnalignedNetwork {
    feature_weights: [UnalignedAccumulator; FEATURES],
    feature_bias: UnalignedAccumulator,
    output_weights: [i16; HIDDEN_SIZE * 2],
    output_bias: i16,
}

static UNALIGNED_NETWORK: UnalignedNetwork =
    unsafe { std::mem::transmute(*include_bytes!("../../../data/network.bin")) };

#[derive(Clone)]
#[repr(C, align(64))]
pub struct Accumulator([i16; HIDDEN_SIZE]);

#[repr(C, align(64))]
struct Network {
    feature_weights: [Accumulator; FEATURES],
    feature_bias: Accumulator,
    output_weights: [i16; HIDDEN_SIZE * 2],
    output_bias: i16,
}

fn main() -> ExitCode {
    println!("size before: {}", size_of::<UnalignedAccumulator>());
    println!("size after: {}", size_of::<Accumulator>());

    let network = Network {
        feature_weights: UNALIGNED_NETWORK.feature_weights.map(|l| Accumulator(l.0)),
        feature_bias: Accumulator(UNALIGNED_NETWORK.feature_bias.0),
        output_weights: UNALIGNED_NETWORK.output_weights,
        output_bias: UNALIGNED_NETWORK.output_bias,
    };

    let d: &[u8] =
        unsafe { slice::from_raw_parts(&network as *const _ as *const u8, size_of::<Network>()) };

    fs::write("aligned.bin", d).unwrap();

    0.into()
}
