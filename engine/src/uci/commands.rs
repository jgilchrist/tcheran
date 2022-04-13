#[derive(Debug)]
pub(super) enum Position {
    Fen(String),
    StartPos,
}

// TODO: Tighten up these types. Position.moves can be Vec<Move> for example.
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
        moves: Vec<String>,
    },
    Go {
        searchmoves: Vec<String>,
        ponder: bool,
        // TODO
        wtime: (),
        btime: (),
        // TODO
        winc: (),
        binc: (),
        // TODO
        movestogo: (),
        depth: u32,
        nodes: u32,
        mate: u32,
        // TODO
        movetime: (),
        infinite: bool,
    },
    Stop,
    PonderHit,
    Quit,
}
