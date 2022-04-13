use chess::r#move::Move;

#[derive(Debug)]
pub(super) enum Position {
    Fen(String),
    StartPos,
}

#[derive(Debug)]
pub(super) enum UciCommand {
    Uci,
    Debug {
        on: bool,
    },
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
    Go {
        searchmoves: Vec<Move>,
        ponder: bool,
        wtime: u32,
        btime: u32,
        winc: u32,
        binc: u32,
        movestogo: u32,
        depth: u32,
        nodes: u32,
        mate: u32,
        movetime: u32,
        infinite: bool,
    },
    Stop,
    PonderHit,
    Quit,
}
