use std::sync::{Arc, Mutex};

use anyhow::Result;
use chess::game::Game;
use tonic::{transport::Server, Request, Response, Status};

use crate::strategy::Strategy;

use self::rpc::{
    out_of_process_engine_server::{self, OutOfProcessEngineServer},
    Empty, GoRequest, GoResponse,
};

use super::BIND_ADDRESS;

pub mod rpc {
    tonic::include_proto!("out_of_process");
}

pub struct State {
    strategy: Box<dyn Strategy + Send + Sync>,
}

pub struct OutOfProcessEngine {
    state: Arc<Mutex<State>>,
}

#[tonic::async_trait]
impl out_of_process_engine_server::OutOfProcessEngine for OutOfProcessEngine {
    async fn ping(&self, _request: Request<Empty>) -> Result<Response<Empty>, Status> {
        Ok(Response::new(Empty {}))
    }

    async fn init(&self, _request: Request<Empty>) -> Result<Response<Empty>, Status> {
        println!("INIT");
        Ok(Response::new(Empty {}))
    }

    async fn go(&self, request: Request<GoRequest>) -> Result<Response<GoResponse>, Status> {
        let fen = request.into_inner().fen;
        println!("GO: {fen}");

        let game = Game::from_fen(&fen).unwrap();

        let mut state = self.state.lock().unwrap();
        let best_move = state.strategy.next_move(&game);
        let new_board_state = game.make_move(&best_move).unwrap();

        println!("{:?}", new_board_state.board);

        let response = GoResponse {
            r#move: best_move.notation(),
        };

        println!("Best move: {}", best_move.notation());
        println!();

        Ok(Response::new(response))
    }
}

// TODO: Print debug information when new connections are made
pub fn run(strategy: Box<dyn Strategy + Send + Sync>) -> Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create the async runtime");

    let relay = OutOfProcessEngine {
        state: Arc::new(Mutex::new(State { strategy })),
    };

    let server = Server::builder()
        .add_service(OutOfProcessEngineServer::new(relay))
        .serve(BIND_ADDRESS.parse()?);

    runtime
        .block_on(server)
        .expect("Could not run the server on the runtime");

    Ok(())
}
