use chess::moves::Move;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Position {
    Fen(String),
    StartPos,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GoCmdArguments {
    pub searchmoves: Option<Vec<Move>>,
    pub ponder: bool,
    pub wtime: Option<i32>,
    pub btime: Option<i32>,
    pub winc: Option<u32>,
    pub binc: Option<u32>,
    pub movestogo: Option<u32>,
    pub depth: Option<u32>,
    pub nodes: Option<u32>,
    pub mate: Option<u32>,
    pub movetime: Option<u32>,
    pub infinite: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DebugCommand {
    Position,
    Perft { depth: u8 },
    PerftDiv { depth: u8 },
    Move { mv: Move },
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
    Register {
        later: bool,
        name: String,
        code: String,
    },
    UciNewGame,
    Position {
        position: Position,
        moves: Vec<Move>,
    },
    Go(GoCmdArguments),
    D(DebugCommand),
    Stop,
    PonderHit,
    Quit,
}
