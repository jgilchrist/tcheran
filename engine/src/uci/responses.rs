use std::fmt::{Formatter, Write};
use std::time::Duration;

use crate::uci::options::{UciOption, UciOptionType};
use chess::moves::Move;

#[derive(Debug)]
pub(super) enum InfoScore {
    Centipawns(i16),
    Mate(i16),
    // lowerbound: i32,
    // upperbound: i32,
}

#[derive(Debug)]
pub(super) enum IdParam {
    Name(String),
    Author(&'static str),
}

#[derive(Debug, Default)]
pub(crate) struct InfoFields {
    pub(super) depth: Option<u8>,
    pub(super) seldepth: Option<u8>,
    pub(super) time: Option<Duration>,
    pub(super) nodes: Option<u32>,
    pub(super) pv: Option<Vec<Move>>,
    pub(super) multipv: Option<u32>,
    pub(super) score: Option<InfoScore>,
    pub(super) currmove: Option<Move>,
    pub(super) currmovenumber: Option<u32>,
    pub(super) hashfull: Option<usize>,
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
    Info(InfoFields),
    Option {
        name: &'static str,
        def: UciOptionType,
    },
}

impl UciResponse {
    pub(super) fn option<T: UciOption>() -> Self {
        Self::Option {
            name: T::NAME,
            def: T::DEF,
        }
    }
}

impl std::fmt::Display for UciResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(i) => match i {
                IdParam::Name(name) => write!(f, "id name {name}")?,
                IdParam::Author(author) => write!(f, "id author {author}")?,
            },
            Self::UciOk => write!(f, "uciok")?,
            Self::ReadyOk => write!(f, "readyok")?,
            // TODO: Account for 'ponder'
            Self::BestMove {
                mv,
                ponder: _ponder,
            } => write!(f, "bestmove {}", mv.notation())?,
            Self::Info(InfoFields {
                depth,
                seldepth,
                time,
                nodes,
                pv,
                multipv: _multipv,
                score,
                currmove,
                currmovenumber: _currmovenumber,
                hashfull,
                nps,
                tbhits: _tbhits,
                cpuload: _cpuload,
                string: _string,
                refutation: _refutation,
                currline: _currline,
            }) => {
                write!(f, "info")?;

                // TODO: Some of these fields are not implemented

                if let Some(depth) = depth {
                    write!(f, " depth {depth}")?;
                }

                if let Some(seldepth) = seldepth {
                    write!(f, " seldepth {seldepth}")?;
                }

                if let Some(score) = score {
                    match score {
                        InfoScore::Centipawns(centipawns) => {
                            write!(f, " score cp {centipawns}")?;
                        }
                        InfoScore::Mate(turns) => {
                            write!(f, " score mate {turns}")?;
                        }
                    }
                }

                if let Some(time) = time {
                    write!(f, " time {}", time.as_millis())?;
                }

                if let Some(nodes) = nodes {
                    write!(f, " nodes {nodes}")?;
                }

                if let Some(nps) = nps {
                    write!(f, " nps {nps}")?;
                }

                if let Some(hashfull) = hashfull {
                    write!(f, " hashfull {hashfull}")?;
                }

                if let Some(currmove) = currmove {
                    write!(f, " currmove {}", currmove.notation())?;
                }

                if let Some(pv) = pv {
                    write!(f, " pv")?;

                    for mv in pv {
                        write!(f, " {}", mv.notation())?;
                    }
                }
            }
            Self::Option { name, def } => {
                write!(f, "option name {name}")?;

                write!(
                    f,
                    " {}",
                    match def {
                        UciOptionType::Check { .. } => "check",
                        UciOptionType::Spin { .. } => "spin",
                        UciOptionType::Combo { .. } => "combo",
                        UciOptionType::String { .. } => "string",
                        UciOptionType::Button => "button",
                    }
                )?;

                match def {
                    UciOptionType::Check { default } => write!(f, " default {default}")?,
                    UciOptionType::Spin { default, .. } => write!(f, " default {default}")?,
                    UciOptionType::Combo { default, .. } | UciOptionType::String { default } => {
                        write!(f, " default {default}")?;
                    }
                    UciOptionType::Button => {}
                };

                match def {
                    UciOptionType::Spin { min, max, .. } => write!(f, " min {min} max {max}")?,
                    UciOptionType::Combo { ref values, .. } => {
                        for v in values {
                            write!(f, " var {v}")?;
                        }
                    }
                    UciOptionType::Check { .. }
                    | UciOptionType::String { .. }
                    | UciOptionType::Button => {}
                }
            }
        }

        Ok(())
    }
}
