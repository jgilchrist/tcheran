use anyhow::Result;
use chess::{debug, game::Game};
use std::io::BufRead;

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

#[derive(Debug)]
struct UciState {
    debug: bool,
    game: Game,
}

#[derive(Debug, PartialEq)]
enum ExecuteResult {
    KeepGoing,
    Exit,
}

fn send_response(response: &UciResponse) {
    println!("{}", response.as_string());

    debug::log(&format!("\t\t{}", response.as_string()));
}

fn execute(cmd: &UciCommand, state: &mut UciState) -> Result<ExecuteResult> {
    match cmd {
        UciCommand::Uci => {
            let version = crate::engine_version();
            send_response(&UciResponse::Id(IdParam::Name(format!(
                "engine ({version})"
            ))));
            send_response(&UciResponse::Id(IdParam::Author("Jonathan Gilchrist")));
            send_response(&UciResponse::UciOk);
        }
        UciCommand::Debug(on) => state.debug = *on,
        UciCommand::IsReady => send_response(&UciResponse::ReadyOk),
        UciCommand::SetOption { name: _, value: _ } => {}
        UciCommand::Register {
            later: _,
            name: _,
            code: _,
        } => {}
        UciCommand::UciNewGame => {
            state.game = Game::new();
        }
        UciCommand::Position { position, moves } => {
            match position {
                commands::Position::StartPos => {
                    let mut game = Game::new();

                    for r#move in moves {
                        // TODO: Error handling for invalid moves
                        game = game.make_move(r#move).unwrap();
                    }

                    state.game = game;
                    debug::log(&format!("{:?}", state.game));
                }
                // TODO: Get board from FEN
                commands::Position::Fen(_) => state.game = Game::new(),
            }
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
            let best_move = crate::run(&state.game.board);

            send_response(&UciResponse::BestMove {
                r#move: best_move,
                ponder: None,
            });
        }
        UciCommand::Stop => {
            let best_move = crate::run(&state.game.board);

            send_response(&UciResponse::BestMove {
                r#move: best_move,
                ponder: None,
            });
        }
        UciCommand::PonderHit => {}
        UciCommand::Quit => return Ok(ExecuteResult::Exit),
    }

    Ok(ExecuteResult::KeepGoing)
}

pub fn uci() -> Result<()> {
    println!("Welcome!");
    println!("In UCI mode.");

    let mut state = UciState {
        debug: false,
        game: Game::new(),
    };

    let stdin = std::io::stdin();

    debug::log("\n\n============== Engine ============");

    for line in stdin.lock().lines() {
        let line = line?;
        debug::log(&line);
        let command = parser::parse(&line);

        match command {
            Ok(ref c) => {
                let execute_result = execute(c, &mut state)?;
                if execute_result == ExecuteResult::Exit {
                    break;
                }

                debug::log("");
            }
            Err(e) => {
                eprintln!("{}", e);
                debug::log("? Unknown command\n");
            }
        }
    }

    Ok(())
}
