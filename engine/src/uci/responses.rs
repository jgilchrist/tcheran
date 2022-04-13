pub struct InfoScore {
    cp: f32,
    mate: u32,
    lowerbound: f32,
    upperbound: f32,
}

pub enum OptionType {
    Check,
    Spin,
    Combo,
    Button,
    String,
}

// TODO: Tighten up these types. BestMove.move can be Move, for example
pub enum UciResponse {
    Id {
        name: String,
        author: String,
    },
    UciOk,
    ReadyOk,
    BestMove {
        r#move: String,
    },
    // TODO
    CopyProtection,
    // TODO
    Registration,
    // TODO: Make this enum variant reflect actual info messages that would be sent
    // Typically certain info messages are sent together, e.g.:
    // info depth 1 seldepth 0
    // info nps 15937
    // info score cp 20  depth 3 nodes 423 time 15 pv f1c4 g8f6 b1c3
    Info {
        depth: u32,
        seldepth: u32,
        // TODO
        time: (),
        nodes: u32,
        pv: Vec<String>,
        // TODO
        multipv: (),
        score: InfoScore,
        currmove: String,
        currmovenumber: u32,
        // TODO
        hashfull: (),
        nps: u32,
        tbhits: u32,
        // TODO
        cpuload: (),
        string: Option<String>,
        // TODO
        refutation: (),
        // TODO
        currline: (),
    },
    Option {
        name: String,
        r#type: OptionType,
        default: String,
        min: String,
        max: String,
        var: String,
    },
}
