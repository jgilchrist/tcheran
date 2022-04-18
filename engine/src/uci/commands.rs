use chess::r#move::Move;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum Position {
    Fen(String),
    StartPos,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) struct GoCmdArguments {
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
pub(super) enum UciCommand {
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
    Stop,
    PonderHit,
    Quit,
}
