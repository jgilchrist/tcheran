use chess::{game::Game, moves::Move};

use crate::log::log;

use super::Strategy;

use std::time::Duration;

use tokio::runtime::{Builder, Runtime};
use tonic::Status;

use self::rpc::{Empty, GoRequest};

pub const BIND_ADDRESS: &str = "[::1]:1359";

pub mod out_of_process_engine;

mod rpc {
    tonic::include_proto!("out_of_process");
}

#[derive(Default)]
pub struct OutOfProcessEngineStrategy {
    conn: OutOfProcessConnectionManager,
}

impl Strategy for OutOfProcessEngineStrategy {
    fn next_move(&mut self, game: &Game) -> Move {
        self.conn
            .with_connected_client(|client| client.go(game.clone()).unwrap())
    }
}

#[derive(Default)]
struct OutOfProcessConnectionManager {
    client: Option<OutOfProcessEngineClient>,
}

impl OutOfProcessConnectionManager {
    // TODO: Check if connected, and if not, re-initialise the connection
    pub fn with_connected_client<Ret>(
        &mut self,
        run_fn: impl Fn(&mut OutOfProcessEngineClient) -> Ret,
    ) -> Ret {
        let maybe_client = self.get_connected_client();

        match maybe_client {
            Some(c) => run_fn(c),
            None => loop {
                let client_result =
                    OutOfProcessEngineClient::connect(format!("http://{}", BIND_ADDRESS));

                if let Ok(mut c) = client_result {
                    c.init().unwrap();
                    self.client = Some(c);
                    return run_fn(self.client.as_mut().unwrap());
                } else {
                    log("Unable to connect");
                    std::thread::sleep(Duration::from_secs(1));
                }
            },
        }
    }

    fn get_connected_client(&mut self) -> Option<&mut OutOfProcessEngineClient> {
        match self.client {
            Some(ref mut client_impl) => match client_impl.ping() {
                Ok(_) => Some(client_impl),
                Err(_) => None,
            },
            None => None,
        }
    }
}

// TODO: How can we ensure only a single connection is made at once?
// Multiple games being played with the same engine will fight for the state.

struct OutOfProcessEngineClient {
    runtime: Runtime,
    client: self::rpc::out_of_process_engine_client::OutOfProcessEngineClient<
        tonic::transport::Channel,
    >,
}

impl OutOfProcessEngineClient {
    pub fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
        D: std::convert::TryInto<tonic::transport::Endpoint>,
        D::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    {
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();

        let client = runtime.block_on(
            self::rpc::out_of_process_engine_client::OutOfProcessEngineClient::connect(dst),
        )?;

        Ok(Self { runtime, client })
    }

    pub fn init(&mut self) -> Result<(), Status> {
        self.runtime
            .block_on(self.client.init(tonic::Request::new(Empty {})))
            .map(|_| ())
    }

    pub fn ping(&mut self) -> Result<(), Status> {
        self.runtime
            .block_on(self.client.ping(tonic::Request::new(Empty {})))
            .map(|_| ())
    }

    pub fn go(&mut self, game: Game) -> Result<Move, Status> {
        self.runtime
            .block_on(
                self.client
                    .go(tonic::Request::new(GoRequest { fen: game.to_fen() })),
            )
            .map(|r| r.into_inner().r#move)
            // TODO
            .map(|m| crate::uci::parser::parse_move(&m).unwrap())
    }
}
