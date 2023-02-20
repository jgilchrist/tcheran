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
    #[must_use]
    pub fn create(&self) -> Box<dyn Strategy + Send + Sync> {
        match self {
            Self::Main => Box::<MainStrategy>::default(),
            Self::Random => Box::<RandomMoveStrategy>::default(),
            Self::TopEval => Box::<TopEvalStrategy>::default(),
            Self::OutOfProcess => Box::<OutOfProcessEngineStrategy>::default(),
        }
    }
}
