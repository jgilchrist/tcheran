use std::io::BufRead;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use chess::{game::Game, moves::Move};

use crate::sync::LockLatch;
use crate::{
    log::log,
    strategy::{self, Strategy},
};

use self::responses::{InfoScore, InfoFields};
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

// TODO: Split reporting from control
#[derive(Clone)]
pub struct UciReporter {
    stop: Arc<Mutex<bool>>,
    stopped: Arc<LockLatch>,
}

impl strategy::Reporter for UciReporter {
    fn should_stop(&self) -> bool {
        *self.stop.lock().unwrap()
    }

    fn generic_report(&self, s: &str) {
        println!("{s}");
    }

    fn best_move(&self, mv: Move) {
        send_response(&UciResponse::BestMove { mv, ponder: None });
        self.stopped.set();
    }

    fn report_search_progress(&self, progress: strategy::SearchInfo) {
        let score = match progress.score {
            strategy::SearchScore::Centipawns(cp) => InfoScore::Centipawns(cp),
            strategy::SearchScore::Mate(moves) => InfoScore::Mate(moves),
        };

        send_response(&UciResponse::Info(InfoFields {
            depth: Some(progress.depth),
            score: Some(score),
            time: Some(progress.stats.time),
            nodes: Some(progress.stats.nodes),
            nps: Some(progress.stats.nodes_per_second),
            ..Default::default()
        }));
    }

    fn report_search_stats(&self, stats: strategy::SearchStats) {
        send_response(&UciResponse::Info(InfoFields {
            time: Some(stats.time),
            nodes: Some(stats.nodes),
            nps: Some(stats.nodes_per_second),
            ..Default::default()
        }));
    }
}

pub struct Uci {
    strategy: Arc<Mutex<Box<dyn Strategy<UciReporter>>>>,
    reporter: UciReporter,
    debug: bool,
    game: Game,
}

impl Uci {
    // Both of these clippy lints can be ignored as there is more to implement here.
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::match_same_arms)]
    fn execute(&mut self, cmd: &UciCommand) -> Result<ExecuteResult> {
        match cmd {
            UciCommand::Uci => {
                self.game = Game::new();
                let version = crate::engine_version();
                send_response(&UciResponse::Id(IdParam::Name(format!(
                    "engine ({version})"
                ))));
                send_response(&UciResponse::Id(IdParam::Author("Jonathan Gilchrist")));
                send_response(&UciResponse::UciOk);
            }
            UciCommand::Debug(on) => {
                self.debug = *on;
            }
            UciCommand::IsReady => send_response(&UciResponse::ReadyOk),
            UciCommand::SetOption { name: _, value: _ } => {}
            UciCommand::Register {
                later: _,
                name: _,
                code: _,
            } => {}
            UciCommand::UciNewGame => {
                self.game = Game::new();
                log(format!("{:?}", self.game.board));
            }
            // TODO: Error handling for invalid positions/moves
            UciCommand::Position { position, moves } => {
                let mut game = match position {
                    commands::Position::StartPos => Game::new(),
                    commands::Position::Fen(fen) => Game::from_fen(fen).unwrap(),
                };

                for mv in moves {
                    game = game.make_move(mv).unwrap();
                }

                self.game = game;
                log(format!("{:?}", self.game.board));
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
                let strategy = self.strategy.clone();
                let game = self.game.clone();
                let reporter = self.reporter.clone();

                std::thread::spawn(move || {
                    let mut s = strategy.lock().unwrap();
                    s.go(&game, reporter);
                });
            }
            UciCommand::Stop => {
                {
                    let mut stop = self.reporter.stop.lock().unwrap();
                    *stop = true;
                }
                self.reporter.stopped.wait();
            }
            UciCommand::PonderHit => {}
            UciCommand::Quit => return Ok(ExecuteResult::Exit),
        }

        Ok(ExecuteResult::KeepGoing)
    }

    fn main_loop(&mut self) -> Result<()> {
        let stdin_lines = std::io::stdin().lock().lines();

        for line in stdin_lines {
            let line = line?;

            log(format!("< {}", &line));
            let command = parser::parse(&line);

            match command {
                Ok(ref c) => {
                    let execute_result = self.execute(c)?;
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

pub fn uci(strategy: Box<dyn Strategy<UciReporter>>) -> Result<()> {
    let mut uci = Uci {
        strategy: Arc::new(Mutex::new(strategy)),
        reporter: UciReporter {
            stop: Arc::new(Mutex::new(false)),
            stopped: Arc::new(LockLatch::new()),
        },
        debug: false,
        game: Game::new(),
    };

    uci.main_loop()
}
