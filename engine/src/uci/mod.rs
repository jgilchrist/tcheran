use anyhow::Result;
use chess::board::Board;
use chess::debug;
use std::io::BufRead;

use self::{
    commands::{GoCmdArguments, UciCommand},
    responses::{IdParam, UciResponse},
};

pub mod commands;
pub mod parser;
pub mod responses;

/// Implementation of the Universal Chess Interface (UCI) protocol

// TODO: Use some clearer types in commands/responses, e.g. u32 -> nplies/msec

#[derive(Debug)]
struct UciState {
    debug: bool,
    board: Board,
}

#[derive(Debug, PartialEq)]
enum ExecuteResult {
    KeepGoing,
    Exit,
}

fn send_response(response: &UciResponse) {
    println!("{}", response.as_string());

    debug::log(&format!("\t\t{}", response.as_string()));
    debug::log(&format!("\t\t  {:?}", response));
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
        UciCommand::SetOption { name, value } => {}
        UciCommand::Register { later, name, code } => {}
        UciCommand::UciNewGame => {
            state.board = Board::start();
        }
        UciCommand::Position { position, moves } => {
            match position {
                commands::Position::StartPos => {
                    let mut board = Board::start();

                    for r#move in moves {
                        // TODO: Error handling for invalid moves
                        let (new_board, _) = board.make_move(r#move).unwrap();
                        board = new_board;
                    }

                    state.board = board;
                    debug::log(&format!("{:?}", state.board));
                }
                // TODO: Get board from FEN
                commands::Position::Fen(_) => state.board = Board::start(),
            }
        }
        UciCommand::Go(GoCmdArguments {
            searchmoves,
            ponder,
            wtime,
            btime,
            winc,
            binc,
            movestogo,
            depth,
            nodes,
            mate,
            movetime,
            infinite,
        }) => {
            let best_move = crate::run(&state.board);

            send_response(&UciResponse::BestMove {
                r#move: best_move,
                ponder: None,
            });
        }
        UciCommand::Stop => {
            let best_move = crate::run(&state.board);

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
        board: Board::start(),
    };

    let stdin = std::io::stdin();

    debug::log("\n\n============== Engine ============");

    for line in stdin.lock().lines() {
        let line = line?;
        debug::log(&line);
        let command = parser::parse(&line);

        match command {
            Ok(ref c) => {
                debug::log(&format!("  {:?}", c));

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
