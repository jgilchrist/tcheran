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

#[derive(Debug, Default)]
pub(crate) struct InfoFields {
    pub(super) depth: Option<u32>,
    pub(super) seldepth: Option<u32>,
    pub(super) time: Option<Duration>,
    pub(super) nodes: Option<u32>,
    pub(super) pv: Option<Vec<Move>>,
    pub(super) multipv: Option<u32>,
    pub(super) score: Option<InfoScore>,
    pub(super) currmove: Option<Move>,
    pub(super) currmovenumber: Option<u32>,
    pub(super) hashfull: Option<u32>,
    pub(super) nps: Option<u32>,
    pub(super) tbhits: Option<u32>,
    pub(super) cpuload: Option<u32>,
    pub(super) string: Option<String>,
    pub(super) refutation: Option<(Move, Option<Move>)>,
    pub(super) currline: Option<Vec<Move>>,
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
    Info(InfoFields),
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
            Self::Info(InfoFields {
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
            }) => {
                let mut response = "info".to_owned();

                // TODO: Some of these fields are not implemented

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
                            response.push_str(&format!(" score cp {centipawns}"));
                        }
                        InfoScore::Mate(turns) => {
                            response.push_str(&format!(" score mate {turns}"));
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
