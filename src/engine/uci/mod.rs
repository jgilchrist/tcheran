//! Implementation of the Universal Chess Interface (UCI) protocol

use color_eyre::eyre::{bail, Context};
use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::chess::moves::Move;
use crate::chess::perft;
use color_eyre::Result;

use crate::engine::options::EngineOptions;
use crate::engine::strategy::{Clocks, SearchRestrictions, TimeControl};
use crate::engine::util::sync::LockLatch;
use crate::engine::{
    eval,
    strategy::{self, Strategy},
    uci, util,
    util::log::log,
};
use crate::uci::commands::DebugCommand;
use crate::uci::options::UciOption;
use crate::ENGINE_NAME;

use self::responses::{InfoFields, InfoScore};
use self::{
    commands::{GoCmdArguments, UciCommand},
    responses::{IdParam, UciResponse},
};

pub mod commands;
mod r#move;
mod options;
pub mod parser;
pub mod responses;

use crate::chess::game::Game;
pub use r#move::UciMove;

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
            pv: Some(progress.pv.into_iter().map(Into::into).collect()),
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
        send_response(&UciResponse::BestMove {
            mv: mv.into(),
            ponder: None,
        });
    }
}

pub struct Uci {
    strategy: Option<Arc<Mutex<Box<dyn Strategy<UciControl, UciReporter>>>>>,
    control: UciControl,
    reporter: UciReporter,
    debug: bool,
    game: Game,
    options: EngineOptions,

    // If we're running without using stdin (i.e. passing the UCI commands as command line
    // args) then we need to block on anything taking place on other threads, otherwise we'll
    // exit immediately as the search takes place on another thread.
    block_on_threads: bool,
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
                    "{ENGINE_NAME} ({version})"
                ))));
                send_response(&UciResponse::Id(IdParam::Author("Jonathan Gilchrist")));

                // Options
                send_response(&UciResponse::option::<uci::options::HashOption>());
                send_response(&UciResponse::option::<uci::options::StrategyOption>());
                send_response(&UciResponse::option::<uci::options::LogOption>());

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
                    options::LogOption::NAME => options::LogOption::set(&mut self.options, value),
                    _ => {
                        bail!("Unknown option: {name}")
                    }
                }?;
            }
            UciCommand::UciNewGame => {
                self.game = Game::new();
                log(self.game.to_fen());
            }
            UciCommand::Position { position, moves } => {
                let mut game = match position {
                    commands::Position::StartPos => Game::new(),
                    commands::Position::Fen(fen) => Game::from_fen(fen)?,
                };

                for mv in moves {
                    game.make_move(*mv);
                }

                self.game = game;
                log(self.game.to_fen());
            }
            UciCommand::Go(GoCmdArguments {
                ponder: _,
                wtime,
                btime,
                winc,
                binc,
                movestogo,
                depth,
                nodes: _,
                movetime,
                infinite: _,
            }) => {
                let strategy = self.strategy.clone();
                let mut game = self.game.clone();
                let options = self.options.clone();
                let control = self.control.clone();
                let reporter = self.reporter.clone();

                let clocks = Clocks {
                    white_clock: *wtime,
                    black_clock: *btime,
                    white_increment: *winc,
                    black_increment: *binc,
                    moves_to_go: *movestogo,
                };

                let mut time_control = TimeControl::Infinite;

                if let Some(move_time) = movetime {
                    time_control = TimeControl::ExactTime(*move_time);
                }

                if wtime.is_some() && btime.is_some() {
                    time_control = TimeControl::Clocks(clocks);
                }

                let search_restrictions = SearchRestrictions { depth: *depth };

                let join_handle = std::thread::spawn(move || {
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

                if self.block_on_threads {
                    join_handle.join().unwrap();
                }
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
                DebugCommand::PrintPosition => {
                    println!("{:?}", self.game.board);
                    println!("FEN: {}", self.game.to_fen());
                    println!();
                }
                DebugCommand::SetPosition { position } => match position.as_str() {
                    "kiwipete" => {
                        self.game = Game::from_fen(
                            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
                        )
                        .unwrap();

                        println!("{:?}", self.game.board);
                    }
                    _ => {
                        bail!("Unknown debug position")
                    }
                },
                DebugCommand::Move { mv } => {
                    self.game.make_move(*mv);
                    println!("{:?}", self.game.board);
                    println!("FEN: {}", crate::chess::fen::write(&self.game));
                    println!();
                }
                DebugCommand::Perft { depth } => {
                    let started_at = Instant::now();
                    let result = perft::perft(*depth, &mut self.game);
                    let time_taken = started_at.elapsed();

                    let nodes_per_second =
                        util::metrics::nodes_per_second(u64::try_from(result).unwrap(), time_taken);

                    println!("positions: {result}");
                    println!("time taken: {time_taken:?}");
                    println!("nps: {nodes_per_second:?}");
                    println!();
                }
                DebugCommand::PerftDiv { depth } => {
                    let result = perft::perft_div(*depth, &mut self.game);
                    let mut total = 0;

                    for (mv, number_for_mv) in result {
                        println!("{mv:?}: {number_for_mv}");
                        total += number_for_mv;
                    }

                    println!("total: {total}");
                    println!();
                }
                DebugCommand::Eval => {
                    let eval_components = eval::eval_components(&self.game);

                    println!("Eval: {}", eval_components.eval);
                    println!(
                        "  Piece square tables: {}",
                        eval_components.piece_square_tables
                    );
                    println!("    Midgame: {}", eval_components.piece_square_midgame);
                    println!("    Endgame: {}", eval_components.piece_square_endgame);
                    println!("    Phase value: {}", eval_components.phase_value);
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

    fn run_line(&mut self, line: &str) -> Result<bool> {
        log(format!("< {}", &line));
        let command = parser::parse(line);

        match command {
            Ok(ref c) => {
                let execute_result = self
                    .execute(c)
                    .wrap_err_with(|| format!("Failed to run UCI command: {line}"))?;

                if execute_result == ExecuteResult::Exit {
                    return Ok(false);
                }
            }
            Err(e) => {
                eprintln!("{e}");
                log("? Unknown command\n");
            }
        }

        Ok(true)
    }

    fn main_loop_stdin(&mut self) -> Result<()> {
        let stdin_lines = std::io::stdin().lock().lines();

        for line in stdin_lines {
            let line = line?;
            let should_continue = self.run_line(&line)?;

            if !should_continue {
                break;
            }
        }

        Ok(())
    }

    fn main_loop_args(&mut self, lines: Vec<String>) -> Result<()> {
        for line in lines {
            let should_continue = self.run_line(&line)?;

            if !should_continue {
                break;
            }
        }

        Ok(())
    }

    fn main_loop(&mut self, uci_input_mode: UciInputMode) -> Result<()> {
        match uci_input_mode {
            UciInputMode::Stdin => self.main_loop_stdin(),
            UciInputMode::Commands(cmds) => self.main_loop_args(cmds),
        }
    }
}

#[derive(Debug, PartialEq)]
enum ExecuteResult {
    KeepGoing,
    Exit,
}

fn send_response(response: &UciResponse) {
    println!("{response}");
    log(format!(" > {response}"));
}

pub enum UciInputMode {
    Commands(Vec<String>),
    Stdin,
}

pub fn uci(uci_input_mode: UciInputMode) -> Result<()> {
    let mut uci = Uci {
        strategy: None,
        control: UciControl {
            stop: Arc::new(Mutex::new(false)),
            stopped: Arc::new(LockLatch::new()),
        },
        reporter: UciReporter {},
        debug: false,
        game: Game::new(),
        options: EngineOptions::default(),

        block_on_threads: match uci_input_mode {
            UciInputMode::Stdin => false,
            UciInputMode::Commands(_) => true,
        },
    };

    uci.set_strategy();
    uci.main_loop(uci_input_mode)
}
