use chess::moves::Move;

#[derive(Debug)]
pub(super) struct InfoScore {
    cp: i32,
    mate: u32,
    lowerbound: i32,
    upperbound: i32,
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

#[derive(Debug)]
pub(super) enum CopyProtectionStatus {
    Checking,
    Ok,
    Error,
}

#[derive(Debug)]
pub(super) enum RegistrationStatus {
    Checking,
    Ok,
    Error,
}

#[derive(Debug)]
pub(super) enum UciResponse {
    Id(IdParam),
    UciOk,
    ReadyOk,
    BestMove {
        mv: Move,
        ponder: Option<Move>,
    },
    CopyProtection(CopyProtectionStatus),
    Registration(RegistrationStatus),
    // TODO: Make this enum variant reflect actual info messages that would be sent
    // Typically certain info messages are sent together, e.g.:
    // info depth 1 seldepth 0
    // info nps 15937
    // info score cp 20  depth 3 nodes 423 time 15 pv f1c4 g8f6 b1c3
    Info {
        depth: Option<u32>,
        seldepth: Option<u32>,
        time: Option<u32>,
        nodes: Option<u32>,
        pv: Option<Vec<Move>>,
        multipv: Option<u32>,
        score: Option<InfoScore>,
        currmove: Option<Move>,
        currmovenumber: Option<u32>,
        hashfull: Option<u32>,
        nps: Option<u32>,
        tbhits: Option<u32>,
        cpuload: Option<u32>,
        string: Option<String>,
        refutation: Option<(Move, Option<Move>)>,
        currline: Option<Vec<Move>>,
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
            Self::Id(i) => match i {
                IdParam::Name(name) => format!("id name {name}"),
                IdParam::Author(author) => format!("id author {author}"),
            },
            Self::UciOk => "uciok".to_string(),
            Self::ReadyOk => "readyok".to_string(),
            // TODO: Account for 'ponder'
            Self::BestMove { mv, ponder } => format!("bestmove {}", mv.notation()),
            Self::CopyProtection(status) => todo!(),
            Self::Registration(status) => todo!(),
            Self::Info {
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
            Self::Option {
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
