use std::time::{Duration, Instant};

use chess::{game::Game, moves::Move};

use crate::options::EngineOptions;
use crate::{
    eval::Eval,
    strategy::{Reporter, SearchInfo, SearchScore, SearchStats},
};

mod move_ordering;
mod negamax;
mod negamax_eval;

pub struct SearchState {
    start_time: Option<Instant>,
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

pub fn search(game: &Game, options: &EngineOptions, reporter: &impl Reporter) -> (Move, Eval) {
    let mut state = SearchState::new();
    state.start_timer();

    let depth = options.max_search_depth;
    let (best_move, pv, eval) = negamax::negamax(game, depth, &mut state, reporter);

    let score = if let Some(nmoves) = eval.is_mate_in_moves() {
        SearchScore::Mate(nmoves)
    } else {
        SearchScore::Centipawns(eval.0)
    };

    reporter.report_search_progress(SearchInfo {
        depth,
        seldepth: state.max_depth_reached,
        score,
        pv,
        stats: SearchStats {
            time: state.elapsed_time(),
            nodes: state.nodes_visited,
            nodes_per_second: state.nodes_per_second(),
        },
    });

    (best_move, eval.to_eval(game.player))
}
