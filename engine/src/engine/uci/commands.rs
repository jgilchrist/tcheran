use crate::engine::uci::UciMove;
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Position {
    Fen(String),
    StartPos,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GoCmdArguments {
    pub ponder: bool,
    pub wtime: Option<Duration>,
    pub btime: Option<Duration>,
    pub winc: Option<Duration>,
    pub binc: Option<Duration>,
    pub movestogo: Option<u32>,
    pub depth: Option<u8>,
    pub nodes: Option<u32>,
    pub movetime: Option<Duration>,
    pub infinite: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DebugCommand {
    PrintPosition,
    SetPosition { position: String },
    Perft { depth: u8 },
    PerftDiv { depth: u8 },
    Move { moves: Vec<UciMove> },
    Eval,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UciCommand {
    Uci,
    Debug(bool),
    IsReady,
    SetOption {
        name: String,
        value: String,
    },
    UciNewGame,
    Position {
        position: Position,
        moves: Vec<UciMove>,
    },
    Go(GoCmdArguments),
    D(DebugCommand),
    Stop,
    PonderHit,
    Bench,
    Quit,
}
