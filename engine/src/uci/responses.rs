use std::time::Duration;

use chess::moves::Move;

#[derive(Debug)]
pub(super) enum InfoScore {
    Centipawns(i32),
    Mate(u32),
    // lowerbound: i32,
    // upperbound: i32,
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
        time: Option<Duration>,
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
            } => {
                let mut response = "info".to_owned();

                if let Some(depth) = depth {
                    response.push_str(&format!(" depth {depth}"));
                }

                if let Some(seldepth) = seldepth {
                    response.push_str(&format!(" seldepth {seldepth}"));
                }

                if let Some(time) = time {
                    response.push_str(&format!(" time {}", time.as_millis()));
                }

                if let Some(nodes) = nodes {
                    response.push_str(&format!(" nodes {nodes}"));
                }

                if let Some(nps) = nps {
                    response.push_str(&format!(" nps {nps}"));
                }

                if let Some(currmove) = currmove {
                    response.push_str(&format!(" currmove {}", currmove.notation()));
                }

                if let Some(score) = score {
                    match score {
                        InfoScore::Centipawns(centipawns) => {
                            response.push_str(&format!(" score cp {centipawns}"))
                        }
                        InfoScore::Mate(turns) => {
                            response.push_str(&format!(" score mate {turns}"))
                        }
                    }
                }

                response
            }
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
