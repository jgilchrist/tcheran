use std::io::BufRead;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use chess::{game::Game, moves::Move};

use crate::sync::LockLatch;
use crate::{
    log::log,
    strategy::{self, Strategy},
};

use self::{
    commands::{GoCmdArguments, UciCommand},
    responses::{IdParam, UciResponse},
};

pub mod commands;
pub mod parser;
#[allow(unused)]
pub mod responses;

/// Implementation of the Universal Chess Interface (UCI) protocol

// TODO: Use some clearer types in commands/responses, e.g. u32 -> nplies/msec

#[derive(Clone)]
pub struct UciReporter {
    stop: Arc<Mutex<bool>>,
    stopped: Arc<LockLatch>,
}

impl strategy::Reporter for UciReporter {
    fn should_stop(&self) -> bool {
        *self.stop.lock().unwrap()
    }

    fn report_progress(&self, s: &str) {
        println!("{s}");
    }

    fn best_move(&self, mv: Move) {
        send_response(&UciResponse::BestMove { mv, ponder: None });
        self.stopped.set();
    }
}

pub struct UciState {
    strategy: Arc<Mutex<Box<dyn Strategy<UciReporter>>>>,
    reporter: UciReporter,
    debug: bool,
    game: Game,
}

impl UciState {
    fn reset(&mut self) {
        self.game = Game::new();
    }

    fn set_debug(&mut self, on: bool) {
        self.debug = on;
    }

    fn set_game_state(&mut self, game: Game) {
        self.game = game;
        log(format!("{:?}", self.game.board));
    }

    fn go(&mut self) {
        let strategy = self.strategy.clone();
        let game = self.game.clone();
        let reporter = self.reporter.clone();

        std::thread::spawn(move || {
            let mut s = strategy.lock().unwrap();
            s.go(&game, reporter);
        });
    }

    fn stop(&mut self) {
        {
            let mut stop = self.reporter.stop.lock().unwrap();
            *stop = true;
        }
        self.reporter.stopped.wait();
    }
}

#[derive(Debug, PartialEq)]
enum ExecuteResult {
    KeepGoing,
    Exit,
}

fn send_response(response: &UciResponse) {
    println!("{}", &response.as_string());
    log(format!(" > {}", response.as_string()));
}

// Both of these clippy lints can be ignored as there is more to implement here.
#[allow(clippy::unnecessary_wraps)]
#[allow(clippy::match_same_arms)]
fn execute(cmd: &UciCommand, state: &mut UciState) -> Result<ExecuteResult> {
    match cmd {
        UciCommand::Uci => {
            state.reset();
            let version = crate::engine_version();
            send_response(&UciResponse::Id(IdParam::Name(format!(
                "engine ({version})"
            ))));
            send_response(&UciResponse::Id(IdParam::Author("Jonathan Gilchrist")));
            send_response(&UciResponse::UciOk);
        }
        UciCommand::Debug(on) => state.set_debug(*on),
        UciCommand::IsReady => send_response(&UciResponse::ReadyOk),
        UciCommand::SetOption { name: _, value: _ } => {}
        UciCommand::Register {
            later: _,
            name: _,
            code: _,
        } => {}
        UciCommand::UciNewGame => state.set_game_state(Game::new()),
        // TODO: Error handling for invalid positions/moves
        UciCommand::Position { position, moves } => {
            let mut game = match position {
                commands::Position::StartPos => Game::new(),
                commands::Position::Fen(fen) => Game::from_fen(fen).unwrap(),
            };

            for mv in moves {
                game = game.make_move(mv).unwrap();
            }

            state.set_game_state(game);
        }
        UciCommand::Go(GoCmdArguments {
            searchmoves: _,
            ponder: _,
            wtime: _,
            btime: _,
            winc: _,
            binc: _,
            movestogo: _,
            depth: _,
            nodes: _,
            mate: _,
            movetime: _,
            infinite: _,
        }) => {
            state.go();
        }
        UciCommand::Stop => {
            state.stop();
        }
        UciCommand::PonderHit => {}
        UciCommand::Quit => return Ok(ExecuteResult::Exit),
    }

    Ok(ExecuteResult::KeepGoing)
}

pub fn uci(strategy: Box<dyn Strategy<UciReporter>>) -> Result<()> {
    let strategy_arc = Arc::new(Mutex::new(strategy));

    let mut state = UciState {
        strategy: strategy_arc,
        reporter: UciReporter {
            stop: Arc::new(Mutex::new(false)),
            stopped: Arc::new(LockLatch::new()),
        },
        debug: false,
        game: Game::new(),
    };

    let stdin = std::io::stdin().lock();
    let stdin_lines = stdin.lines();

    for line in stdin_lines {
        let line = line?;

        log(format!("< {}", &line));
        let command = parser::parse(&line);

        match command {
            Ok(ref c) => {
                let execute_result = execute(c, &mut state)?;
                if execute_result == ExecuteResult::Exit {
                    break;
                }
            }
            Err(e) => {
                eprintln!("{e}");
                log("? Unknown command\n");
            }
        }
    }

    Ok(())
}
