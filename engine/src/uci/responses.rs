#[derive(Debug)]
pub(super) struct InfoScore {
    cp: f32,
    mate: u32,
    lowerbound: f32,
    upperbound: f32,
}

#[derive(Debug)]
pub(super) enum OptionType {
    Check,
    Spin,
    Combo,
    Button,
    String,
}

#[derive(Debug)]
pub(super) enum IdParam {
    Name(String),
    Author(&'static str),
}

// TODO: Tighten up these types. BestMove.move can be Move, for example
#[derive(Debug)]
pub(super) enum UciResponse {
    Id(IdParam),
    UciOk,
    ReadyOk,
    BestMove {
        r#move: String,
        ponder: Option<String>,
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

impl UciResponse {
    pub(super) fn as_string(&self) -> String {
        match self {
            UciResponse::Id(i) => match i {
                IdParam::Name(name) => format!("id name {}", name),
                IdParam::Author(author) => format!("id author {}", author),
            },
            UciResponse::UciOk => "uciok".to_string(),
            UciResponse::ReadyOk => "readyok".to_string(),
            // TODO: Account for 'ponder'
            UciResponse::BestMove { r#move, ponder } => format!("bestmove {}", r#move),
            UciResponse::CopyProtection => todo!(),
            UciResponse::Registration => todo!(),
            UciResponse::Info {
                depth,
                seldepth,
                time,
                nodes,
                pv,
                multipv,
                score,
                currmove,
                currmovenumber,
                hashfull,
                nps,
                tbhits,
                cpuload,
                string,
                refutation,
                currline,
            } => todo!(),
            UciResponse::Option {
                name,
                r#type,
                default,
                min,
                max,
                var,
            } => todo!(),
        }
    }
}
