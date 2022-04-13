use anyhow::{bail, Result};

use super::commands::{Position::StartPos, UciCommand};

pub(super) fn parse(input: &str) -> Result<UciCommand> {
    if input == "uci" {
        return Ok(UciCommand::Uci);
    }

    if input == "debug" {
        // TODO: Parse out 'debug' arguments
        return Ok(UciCommand::Debug { on: false });
    }

    if input == "isready" {
        return Ok(UciCommand::IsReady);
    }

    if input.starts_with("setoption") {
        // TODO: Parse out 'setoption' arguments
        return Ok(UciCommand::SetOption {
            name: "".to_string(),
            value: "".to_string(),
        });
    }

    if input.starts_with("register") {
        // TODO: Parse out 'register' arguments
        return Ok(UciCommand::Register {
            later: false,
            name: "".to_string(),
            code: "".to_string(),
        });
    }

    if input == "ucinewgame" {
        return Ok(UciCommand::UciNewGame);
    }

    if input.starts_with("position") {
        // TODO: Parse out 'position' arguments
        return Ok(UciCommand::Position {
            position: StartPos,
            moves: vec![],
        });
    }

    if input.starts_with("go") {
        // TODO: Parse out 'go' arguments
        return Ok(UciCommand::Go {
            searchmoves: vec![],
            ponder: false,
            wtime: (),
            btime: (),
            winc: (),
            binc: (),
            movestogo: (),
            depth: 1,
            nodes: 1,
            mate: 1,
            movetime: (),
            infinite: false,
        });
    }

    if input == "stop" {
        return Ok(UciCommand::Stop);
    }

    if input == "ponderhit" {
        return Ok(UciCommand::PonderHit);
    }

    if input == "quit" {
        return Ok(UciCommand::Quit);
    }

    bail!("Unknown command: {}", input)
}
