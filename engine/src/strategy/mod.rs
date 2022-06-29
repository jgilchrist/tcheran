use chess::{game::Game, moves::Move};

pub use self::{
    main::MainStrategy, out_of_process::out_of_process_engine::run as run_out_of_process_engine,
    out_of_process::OutOfProcessEngineStrategy, random::RandomMoveStrategy,
    top_eval::TopEvalStrategy,
};

pub trait Strategy {
    fn next_move(&mut self, game: &Game) -> Move;
}

mod main;
mod out_of_process;
mod random;
mod top_eval;

pub enum KnownStrategy {
    Main,
    Random,
    TopEval,
    OutOfProcess,
}

impl KnownStrategy {
    pub fn create(&self) -> Box<dyn Strategy + Send + Sync> {
        match self {
            KnownStrategy::Main => Box::new(MainStrategy::default()),
            KnownStrategy::Random => Box::new(RandomMoveStrategy::default()),
            KnownStrategy::TopEval => Box::new(TopEvalStrategy::default()),
            KnownStrategy::OutOfProcess => Box::new(OutOfProcessEngineStrategy::default()),
        }
    }
}
