//! Implementation of the Universal Chess Interface (UCI) protocol

use color_eyre::eyre::{bail, Context};
use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use chess::moves::Move;
use chess::perft;
use chess::util::nodes_per_second;
use color_eyre::Result;

use crate::game::EngineGame;
use crate::options::EngineOptions;
use crate::strategy::{Clocks, SearchRestrictions, TimeControl};
use crate::uci::commands::DebugCommand;
use crate::uci::options::UciOption;
use crate::util::sync::LockLatch;
use crate::{
    eval,
    strategy::{self, Strategy},
    uci,
    util::log::log,
};

use self::responses::{InfoFields, InfoScore};
use self::{
    commands::{GoCmdArguments, UciCommand},
    responses::{IdParam, UciResponse},
};

pub mod commands;
mod options;
pub mod parser;
#[allow(unused)]
pub mod responses;

// TODO: Use some clearer types in commands/responses, e.g. u32 -> nplies/msec

#[derive(Clone)]
pub struct UciControl {
    stop: Arc<Mutex<bool>>,
    stopped: Arc<LockLatch>,
}

impl strategy::Control for UciControl {
    fn stop(&self) {
        self.stopped.set();
    }

    fn should_stop(&self) -> bool {
        *self.stop.lock().unwrap()
    }
}

#[derive(Clone)]
pub struct UciReporter;

impl strategy::Reporter for UciReporter {
    fn generic_report(&self, s: &str) {
        println!("{s}");
    }

    fn report_search_progress(&self, progress: strategy::SearchInfo) {
        let score = match progress.score {
            strategy::SearchScore::Centipawns(cp) => InfoScore::Centipawns(cp),
            strategy::SearchScore::Mate(moves) => InfoScore::Mate(moves),
        };

        send_response(&UciResponse::Info(InfoFields {
            depth: Some(progress.depth),
            seldepth: Some(progress.seldepth),
            score: Some(score),
            pv: Some(progress.pv),
            time: Some(progress.stats.time),
            nodes: Some(progress.stats.nodes),
            nps: Some(progress.stats.nodes_per_second),
            hashfull: Some(progress.hashfull),
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

    fn best_move(&self, mv: Move) {
        send_response(&UciResponse::BestMove { mv, ponder: None });
    }
}

pub struct Uci {
    strategy: Option<Arc<Mutex<Box<dyn Strategy<UciControl, UciReporter>>>>>,
    control: UciControl,
    reporter: UciReporter,
    debug: bool,
    game: EngineGame,
    options: EngineOptions,
}

impl Uci {
    // Both of these clippy lints can be ignored as there is more to implement here.
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::match_same_arms)]
    fn execute(&mut self, cmd: &UciCommand) -> Result<ExecuteResult> {
        match cmd {
            UciCommand::Uci => {
                self.game = EngineGame::new();

                let version = crate::engine_version();
                send_response(&UciResponse::Id(IdParam::Name(format!(
                    "engine ({version})"
                ))));
                send_response(&UciResponse::Id(IdParam::Author("Jonathan Gilchrist")));

                // Options
                send_response(&UciResponse::option::<uci::options::HashOption>());
                send_response(&UciResponse::option::<uci::options::StrategyOption>());

                send_response(&UciResponse::UciOk);
            }
            UciCommand::Debug(on) => {
                self.debug = *on;
            }
            UciCommand::IsReady => send_response(&UciResponse::ReadyOk),
            UciCommand::SetOption { name, value } => {
                match name.as_str() {
                    options::HashOption::NAME => options::HashOption::set(&mut self.options, value),
                    options::StrategyOption::NAME => {
                        let result = options::StrategyOption::set(&mut self.options, value);
                        self.set_strategy();
                        result
                    }
                    _ => {
                        bail!("Unknown option: {name}")
                    }
                }?;
            }
            UciCommand::UciNewGame => {
                self.game = EngineGame::new();
                log(format!("{:?}", self.game.game.board));
            }
            UciCommand::Position { position, moves } => {
                let mut game = match position {
                    commands::Position::StartPos => EngineGame::new(),
                    commands::Position::Fen(fen) => EngineGame::from_fen(fen)?,
                };

                for mv in moves {
                    game.make_move(mv);
                }

                self.game = game;
                log(format!("{:?}", self.game.game.board));
            }
            UciCommand::Go(GoCmdArguments {
                searchmoves: _,
                ponder: _,
                wtime,
                btime,
                winc,
                binc,
                movestogo,
                depth,
                nodes: _,
                mate: _,
                movetime,
                infinite: _,
            }) => {
                let strategy = self.strategy.clone();
                let mut game = self.game.clone();
                let options = self.options.clone();
                let control = self.control.clone();
                let reporter = self.reporter.clone();

                let clocks = Clocks {
                    white_clock: wtime.map(|t| Duration::from_millis(t.try_into().unwrap())),
                    black_clock: btime.map(|t| Duration::from_millis(t.try_into().unwrap())),
                    white_increment: winc.map(|t| Duration::from_millis(t.try_into().unwrap())),
                    black_increment: binc.map(|t| Duration::from_millis(t.try_into().unwrap())),
                    moves_to_go: *movestogo,
                };

                let move_time = movetime.map(|t| Duration::from_millis(t.try_into().unwrap()));

                // TODO: Improve errors if we get conflicting time control messaging here (e.g. movetime 100 infinite)
                let mut time_control = TimeControl::Infinite;

                if let Some(move_time) = move_time {
                    time_control = TimeControl::ExactTime(move_time);
                }

                if wtime.is_some() && btime.is_some() {
                    time_control = TimeControl::Clocks(clocks);
                }

                let search_restrictions = SearchRestrictions { depth: *depth };

                std::thread::spawn(move || {
                    let strategy_binding = strategy.unwrap();
                    let mut s = strategy_binding.lock().unwrap();
                    s.go(
                        &mut game,
                        &time_control,
                        &search_restrictions,
                        &options,
                        control,
                        reporter,
                    );
                });
            }
            UciCommand::Stop => {
                {
                    let mut stop = self.control.stop.lock().unwrap();
                    *stop = true;
                }
                self.control.stopped.wait();
                self.control.stopped = Arc::new(LockLatch::new());
                {
                    let mut stop = self.control.stop.lock().unwrap();
                    *stop = false;
                }
            }
            UciCommand::D(debug_cmd) => match debug_cmd {
                DebugCommand::Position => {
                    println!("{:?}", self.game.game.board);
                    println!("FEN: {}", chess::fen::write(&self.game.game));
                    println!();
                }
                DebugCommand::Move { mv } => {
                    self.game.make_move(mv);
                    println!("{:?}", self.game.game.board);
                    println!("FEN: {}", chess::fen::write(&self.game.game));
                    println!();
                }
                DebugCommand::Perft { depth } => {
                    let started_at = Instant::now();
                    let result = perft::perft(*depth, &mut self.game.game);
                    let time_taken = started_at.elapsed();

                    let nodes_per_second =
                        nodes_per_second(u32::try_from(result).unwrap(), time_taken);

                    println!("positions: {result}");
                    println!("time taken: {time_taken:?}");
                    println!("nps: {nodes_per_second:?}");
                    println!();
                }
                DebugCommand::PerftDiv { depth } => {
                    let result = perft::perft_div(*depth, &mut self.game.game);
                    let mut total = 0;

                    for (mv, number_for_mv) in result {
                        println!("{mv:?}: {number_for_mv}");
                        total += number_for_mv;
                    }

                    println!("total: {total}");
                    println!();
                }
                DebugCommand::Eval => {
                    let eval_components = eval::eval_components(&self.game.game);

                    println!("Eval: {}", eval_components.eval);
                    println!("Components:");
                    println!("  Material: {}", eval_components.material);
                    println!(
                        "  Piece square tables: {}",
                        eval_components.piece_square_tables
                    );
                    println!("    White: {}", eval_components.piece_square_tables_white);
                    println!("    Black: {}", eval_components.piece_square_tables_black);
                }
            },
            UciCommand::PonderHit => {}
            UciCommand::Quit => return Ok(ExecuteResult::Exit),
        }

        Ok(ExecuteResult::KeepGoing)
    }

    fn set_strategy(&mut self) {
        let strategy = self.options.strategy.create();
        self.strategy = Some(Arc::new(Mutex::new(strategy)));
    }

    fn main_loop(&mut self) -> Result<()> {
        let stdin_lines = std::io::stdin().lock().lines();

        for line in stdin_lines {
            let line = line?;

            log(format!("< {}", &line));
            let command = parser::parse(&line);

            match command {
                Ok(ref c) => {
                    let execute_result = self
                        .execute(c)
                        .wrap_err_with(|| format!("Failed to run UCI command: {line}"))?;

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

pub fn uci() -> Result<()> {
    let mut uci = Uci {
        strategy: None,
        control: UciControl {
            stop: Arc::new(Mutex::new(false)),
            stopped: Arc::new(LockLatch::new()),
        },
        reporter: UciReporter {},
        debug: false,
        game: EngineGame::new(),
        options: EngineOptions::default(),
    };

    uci.set_strategy();
    uci.main_loop()
}
