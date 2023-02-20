use anyhow::Result;
use chess::{game::Game, moves::Move};
use std::io::BufRead;

use crate::{log::log, strategy::Strategy};

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

pub struct UciState {
    strategy: Box<dyn Strategy>,
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

    fn go(&mut self) -> Move {
        let best_move = self.strategy.next_move(&self.game);
        log(format!("{:?}", &best_move));

        let new_game_state = self.game.make_move(&best_move).unwrap();
        log(format!("{:?}", new_game_state.board));

        best_move
    }
}

#[derive(Debug, PartialEq)]
enum ExecuteResult {
    KeepGoing,
    Exit,
}

fn send_response(response: &UciResponse) {
    println!("{}", response.as_string());

    log(format!("\t\t{}", response.as_string()));
}

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

            state.set_game_state(game)
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
            let best_move = state.go();

            send_response(&UciResponse::BestMove {
                mv: best_move,
                ponder: None,
            });
        }
        UciCommand::Stop => {
            let best_move = state.go();

            send_response(&UciResponse::BestMove {
                mv: best_move,
                ponder: None,
            });
        }
        UciCommand::PonderHit => {}
        UciCommand::Quit => return Ok(ExecuteResult::Exit),
    }

    Ok(ExecuteResult::KeepGoing)
}

pub fn uci(strategy: Box<dyn Strategy>) -> Result<()> {
    println!("Welcome!");
    println!("In UCI mode.");

    let mut state = UciState {
        strategy,
        debug: false,
        game: Game::new(),
    };

    let stdin = std::io::stdin();

    log("\n\n============== Engine ============");

    for line in stdin.lock().lines() {
        let line = line?;
        log(&line);
        let command = parser::parse(&line);

        match command {
            Ok(ref c) => {
                let execute_result = execute(c, &mut state)?;
                if execute_result == ExecuteResult::Exit {
                    break;
                }

                log("");
            }
            Err(e) => {
                eprintln!("{}", e);
                log("? Unknown command\n");
            }
        }
    }

    Ok(())
}
