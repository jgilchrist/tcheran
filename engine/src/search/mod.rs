use std::time::{Duration, Instant};

use chess::{game::Game, moves::Move};

use crate::options::EngineOptions;
use crate::search::time_control::TimeControl;
use crate::strategy::{Control, GoArgs};
use crate::{eval::Eval, strategy::Reporter};

pub use negamax_eval::NegamaxEval;

mod iterative_deepening;
mod move_ordering;
mod negamax;
mod negamax_eval;
mod time_control;

pub struct SearchState {
    start_time: Option<Instant>,
    best_pv: Option<Vec<Move>>,
    nodes_visited: u32,
    max_depth_reached: u8,
    beta_cutoffs: u32,
}

impl SearchState {
    const fn new() -> Self {
        Self {
            start_time: None,
            max_depth_reached: 0,
            nodes_visited: 0,
            beta_cutoffs: 0,
            best_pv: None,
        }
    }

    pub fn start_timer(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn elapsed_time(&self) -> Duration {
        let Some(t) = self.start_time else {
            panic!("Tried to fetch search's elapsed time without a start time.")
        };

        t.elapsed()
    }

    // This is an approximate calculations so ignoring all of the possible issues around
    // precision loss here
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    pub fn nodes_per_second(&self) -> u32 {
        let elapsed_time = self.elapsed_time();
        (self.nodes_visited as f32 / elapsed_time.as_secs_f32()) as u32
    }
}

pub fn search(
    game: &Game,
    args: &GoArgs,
    options: &EngineOptions,
    control: &impl Control,
    reporter: &impl Reporter,
) -> (Move, Eval) {
    let mut state = SearchState::new();
    state.start_timer();

    let mut time_control = TimeControl::new(game, &args.clocks);
    time_control.init();

    let (best_move, eval) =
        iterative_deepening::search(game, options, &mut state, &time_control, control, reporter);

    (best_move, eval.to_eval(game.player))
}
