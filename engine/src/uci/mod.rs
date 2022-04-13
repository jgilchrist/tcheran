use anyhow::Result;
use chess::{r#move::Move, square::{File, Rank}};
use std::io::{BufRead, Write};

use self::{
    commands::UciCommand,
    responses::{IdParam, UciResponse},
};

pub mod commands;
pub mod parser;
pub mod responses;

/// Implementation of the Universal Chess Interface (UCI) protocol

// TODO: Use some clearer types in commands/responses, e.g. u32 -> nplies/msec

struct UciState {
    debug: bool,
}

#[derive(Debug, PartialEq)]
enum ExecuteResult {
    KeepGoing,
    Exit,
}

const LOG_PATH: &str = "/tmp/chess_engine";

// FIXME: It's not ideal to open a handle to the file every time we want to write a line
fn log(s: &str) {
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(LOG_PATH)
        .unwrap();

    writeln!(f, "{}", s).unwrap();
    f.flush().unwrap()
}

fn send_response(response: &UciResponse) {
    println!("{}", response.as_string());

    log(&format!("\t\t{}", response.as_string()));
    log(&format!("\t\t  {:?}", response));
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
        UciCommand::Debug { on } => state.debug = *on,
        UciCommand::IsReady => send_response(&UciResponse::ReadyOk),
        UciCommand::SetOption { name, value } => {}
        UciCommand::Register { later, name, code } => {}
        UciCommand::UciNewGame => {}
        UciCommand::Position { position, moves } => {}
        UciCommand::Go {
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
        } => send_response(&UciResponse::BestMove {
            r#move: Move::new(
                chess::square::Square(File::E, Rank::R7),
                chess::square::Square(File::E, Rank::R5),
            ),
            ponder: None,
        }),
        UciCommand::Stop => send_response(&UciResponse::BestMove {
            r#move: Move::new(
                chess::square::Square(File::E, Rank::R7),
                chess::square::Square(File::E, Rank::R5),
            ),
            ponder: None,
        }),
        UciCommand::PonderHit => {}
        UciCommand::Quit => return Ok(ExecuteResult::Exit),
    }

    Ok(ExecuteResult::KeepGoing)
}

pub fn uci() -> Result<()> {
    println!("Welcome!");
    println!("In UCI mode.");

    let mut state = UciState { debug: false };

    let stdin = std::io::stdin();

    log("\n\n============== Engine ============");

    for line in stdin.lock().lines() {
        let line = line?;
        log(&line);
        let command = parser::parse(&line);

        match command {
            Ok(ref c) => {
                log(&format!("  {:?}", c));

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
