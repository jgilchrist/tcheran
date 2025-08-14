use crate::chess::board::Board;
use crate::chess::piece::Piece;
use crate::chess::player::Player;
use crate::chess::square::Square;
use crate::engine::eval::Eval;
/// NNUE Implementation
/// Carp uses a (768->1024)x2->1 perspective net architecture, fully trained on self play data.
/// Network is initialized at compile time from the 'net.bin' file in thie bins directory.
/// A new net can be loaded by running the convert_json.py script in the scripts folder.
///
/// Huge thanks to Cosmo, author of Viridithas, for the help. The code here is heavily inspired by
/// his engine.
use std::mem;
use std::ops::{Deref, DerefMut};

// Network Arch
const FEATURES: usize = 768;
const HIDDEN: usize = 1024;

// Clipped ReLu bounds
const CR_MIN: i16 = 0;
const CR_MAX: i16 = 255;

// Quantization factors
const QA: i32 = 255;
const QAB: i32 = 255 * 64;

// Eval scaling factor
const SCALE: i32 = 400;

/// Container for all network parameters
#[repr(C)]
struct NNUEParams {
    feature_weights: Align64<[i16; FEATURES * HIDDEN]>,
    feature_bias: Align64<[i16; HIDDEN]>,
    output_weights: Align64<[i16; HIDDEN * 2]>,
    output_bias: i16,
}

/// NNUE model is initialized from binary values (Viridithas format)
static MODEL: NNUEParams = unsafe { mem::transmute(*include_bytes!("../../../../net.bin")) };

/// Generic wrapper for types aligned to 64B for AVX512 (also a Viridithas trick)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C, align(64))]
struct Align64<T>(pub T);

impl<T, const SIZE: usize> Deref for Align64<[T; SIZE]> {
    type Target = [T; SIZE];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T, const SIZE: usize> DerefMut for Align64<[T; SIZE]> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

type SideAccumulator = Align64<[i16; HIDDEN]>;

/// Accumulators contain the efficiently updated hidden layer values
/// Each accumulator is perspective, hence both contains the white and black pov
#[derive(Clone, Debug)]
pub struct Accumulator {
    white: SideAccumulator,
    black: SideAccumulator,
}

impl Default for Accumulator {
    fn default() -> Self {
        Self {
            white: MODEL.feature_bias,
            black: MODEL.feature_bias,
        }
    }
}

impl Accumulator {
    pub fn from_board(board: &Board) -> Self {
        let mut acc = Self::default();

        for sq in board.occupancy() {
            acc.update_weights::<ON>(board.piece_at(sq).unwrap(), sq);
        }

        acc
    }

    /// Updates weights for a single feature, either turning them on or off
    pub fn update_weights<const ON: bool>(&mut self, piece: Piece, sq: Square) {
        fn update<const ON: bool>(acc: &mut SideAccumulator, idx: usize) {
            let zip = acc
                .iter_mut()
                .zip(&MODEL.feature_weights[idx..idx + HIDDEN]);

            for (acc_val, &weight) in zip {
                if ON {
                    *acc_val += weight;
                } else {
                    *acc_val -= weight;
                }
            }
        }

        let (white_idx, black_idx) = nnue_index(piece, sq);

        update::<ON>(&mut self.white, white_idx);
        update::<ON>(&mut self.black, black_idx);
    }

    /// Update accumulator for a quiet move.
    /// Adds in features for the destination and removes the features of the source
    fn add_sub_weights(&mut self, piece: Piece, from: Square, to: Square) {
        fn add_sub(acc: &mut SideAccumulator, from: usize, to: usize) {
            let zip = acc.iter_mut().zip(
                MODEL.feature_weights[from..from + HIDDEN]
                    .iter()
                    .zip(&MODEL.feature_weights[to..to + HIDDEN]),
            );

            for (acc_val, (&remove_weight, &add_weight)) in zip {
                *acc_val += add_weight - remove_weight;
            }
        }

        let from = nnue_index(piece, from);
        let to = nnue_index(piece, to);

        add_sub(&mut self.white, from.0, to.0);
        add_sub(&mut self.black, from.1, to.1);
    }

    /// Evaluate the nn from the current accumulator
    /// Concatenates the accumulators based on the side to move, computes the activation function
    /// with Squared CReLu and multiplies activation by weight. The result is the sum of all these
    /// with the bias.
    /// Since we are squaring activations, we need an extra quantization pass with QA.
    pub fn evaluate(&self, side: Player) -> Eval {
        let (us, them) = match side {
            Player::White => (self.white.iter(), self.black.iter()),
            Player::Black => (self.black.iter(), self.white.iter()),
        };

        let mut out = 0;
        for (&value, &weight) in us.zip(&MODEL.output_weights[..HIDDEN]) {
            out += squared_crelu(value) * (weight as i32);
        }
        for (&value, &weight) in them.zip(&MODEL.output_weights[HIDDEN..]) {
            out += squared_crelu(value) * (weight as i32);
        }

        Eval((out / QA + MODEL.output_bias as i32) * SCALE / QAB)
    }
}

// /// NNUEState is simply a stack of accumulators, updated along the search tree
// #[derive(Debug, Clone)]
// pub struct NNUEState {
//     accumulator_stack: [Accumulator; 256],
//     current_acc: usize,
// }

// used for turning on/off features
pub const ON: bool = true;
pub const OFF: bool = false;

// impl NNUEState {
//     /// Inits nnue state from a board
//     /// To be able to run debug builds, heap is allocated manually
//     pub fn from_board(board: &Board) -> Box<Self> {
//         let mut boxed: Box<Self> = unsafe {
//             let layout = alloc::Layout::new::<Self>();
//             let ptr = alloc::alloc_zeroed(layout);
//             if ptr.is_null() {
//                 alloc::handle_alloc_error(layout);
//             }
//             Box::from_raw(ptr.cast())
//         };
//
//         // init with feature biases and add in all features of the board
//         boxed.accumulator_stack[0] = Accumulator::from_board(board);
//         for sq in board.occupancy() {
//             boxed.manual_update::<ON>(board.piece_at(sq).unwrap(), sq);
//         }
//
//         boxed
//     }
//
//     /// Refresh the accumulator stack to the given board
//     // pub fn refresh(&mut self, board: &Board) {
//     //     // reset the accumulator stack
//     //     self.current_acc = 0;
//     //     self.accumulator_stack[self.current_acc] = Accumulator::default();
//     //
//     //     // update the first accumulator
//     //     for piece in PieceKind::ALL {
//     //         for sq in board.piece_occupancy(piece) {
//     //             self.manual_update::<ON>(piece, sq);
//     //         }
//     //     }
//     // }
//
//     /// Add a new accumulator to the stack by copying the previous top
//     pub fn push(&mut self) {
//         self.accumulator_stack[self.current_acc + 1] = self.accumulator_stack[self.current_acc];
//         self.current_acc += 1;
//     }
//
//     /// Pop the top off the accumulator stack
//     pub fn pop(&mut self) {
//         self.current_acc -= 1;
//     }
//
//     /// Manually turn on or off the single given feature
//     pub fn manual_update<const ON: bool>(&mut self, piece: Piece, sq: Square) {
//         self.accumulator_stack[self.current_acc].update_weights::<ON>(piece, sq);
//     }
//
//     /// Efficiently update accumulator for a quiet move (that is, only changes from/to features)
//     pub fn move_update(&mut self, piece: Piece, from: Square, to: Square) {
//         self.accumulator_stack[self.current_acc].add_sub_weights(piece, from, to);
//     }
//
//     /// Evaluate the nn from the current accumulator
//     /// Concatenates the accumulators based on the side to move, computes the activation function
//     /// with Squared CReLu and multiplies activation by weight. The result is the sum of all these
//     /// with the bias.
//     /// Since we are squaring activations, we need an extra quantization pass with QA.
//     pub fn evaluate(&self, side: Player) -> Eval {
//         let acc = &self.accumulator_stack[self.current_acc];
//         acc.evaluate(side)
//     }
// }

/// Returns white and black feature weight index for given feature
const fn nnue_index(piece: Piece, sq: Square) -> (usize, usize) {
    const COLOR_STRIDE: usize = 64 * 6;
    const PIECE_STRIDE: usize = 64;
    let p = piece.kind.array_idx();
    let c = piece.player.array_idx();

    let white_idx =
        c * COLOR_STRIDE + p * PIECE_STRIDE + sq.relative_for(Player::White).array_idx();
    let black_idx =
        (1 ^ c) * COLOR_STRIDE + p * PIECE_STRIDE + sq.relative_for(Player::Black).array_idx();

    (white_idx * HIDDEN, black_idx * HIDDEN)
}

/// Squared Clipped ReLu activation function
fn squared_crelu(value: i16) -> i32 {
    let v = value.clamp(CR_MIN, CR_MAX) as i32;

    v * v
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::game::Game;
    use crate::chess::moves::Move;
    use crate::chess::square::squares::all::*;

    #[test]
    fn test_nnue_index() {
        crate::init();

        let idx1 = nnue_index(Piece::WHITE_PAWN, A8);
        let idx2 = nnue_index(Piece::WHITE_PAWN, H1);
        let idx3 = nnue_index(Piece::BLACK_PAWN, A1);
        let idx4 = nnue_index(Piece::WHITE_KING, E1);

        assert_eq!(idx1, (HIDDEN * 56, HIDDEN * 384));
        assert_eq!(idx2, (HIDDEN * 7, HIDDEN * 447));
        assert_eq!(idx3, (HIDDEN * 384, HIDDEN * 56));
        assert_eq!(idx4, (HIDDEN * 324, HIDDEN * 764));
    }
}
