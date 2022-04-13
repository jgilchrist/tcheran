use anyhow::{bail, Result};
use chess::{
    r#move::Move,
    square::{File, Rank, Square},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, one_of},
    combinator::{eof, map, opt},
    multi::{fold_many0, separated_list1},
    sequence::{pair, preceded, separated_pair, tuple},
    IResult, InputTakeAtPosition,
};

use super::commands::{GoCmdArguments, Position, UciCommand};

// FIXME: Don't accept `isreadymorechars` as `IsReady`

fn non_space(input: &str) -> IResult<&str, &str> {
    input.split_at_position_complete(char::is_whitespace)
}

fn boolean(input: &str) -> IResult<&str, bool> {
    alt((map(tag("on"), |_| true), map(tag("off"), |_| false)))(input)
}

fn uci_file(input: &str) -> IResult<&str, File> {
    let (input, file) = one_of("abcdefgh")(input)?;

    Ok((
        input,
        match file {
            'a' => File::A,
            'b' => File::B,
            'c' => File::C,
            'd' => File::D,
            'e' => File::E,
            'f' => File::F,
            'g' => File::G,
            'h' => File::H,
            _ => unreachable!(),
        },
    ))
}

fn uci_rank(input: &str) -> IResult<&str, Rank> {
    let (input, rank) = one_of("12345678")(input)?;

    Ok((
        input,
        match rank {
            '1' => Rank::R1,
            '2' => Rank::R2,
            '3' => Rank::R3,
            '4' => Rank::R4,
            '5' => Rank::R5,
            '6' => Rank::R6,
            '7' => Rank::R7,
            '8' => Rank::R8,
            _ => unreachable!(),
        },
    ))
}

fn uci_square(input: &str) -> IResult<&str, Square> {
    map(tuple((uci_file, uci_rank)), |(file, rank)| {
        Square::new(file, rank)
    })(input)
}

fn uci_move(input: &str) -> IResult<&str, Move> {
    map(tuple((uci_square, uci_square)), |(from, to)| {
        Move::new(from, to)
    })(input)
}

fn cmd_uci(input: &str) -> IResult<&str, UciCommand> {
    map(pair(tag("uci"), eof), |_| UciCommand::Uci)(input)
}

fn cmd_debug(input: &str) -> IResult<&str, UciCommand> {
    map(
        separated_pair(tag("debug"), multispace1, boolean),
        |(_, on)| UciCommand::Debug(on),
    )(input)
}

fn cmd_isready(input: &str) -> IResult<&str, UciCommand> {
    map(tag("isready"), |_| UciCommand::IsReady)(input)
}

fn cmd_setoption(input: &str) -> IResult<&str, UciCommand> {
    let (input, _) = tag("setoption")(input)?;

    // TODO: Parse out 'setoption' arguments
    Ok((
        input,
        UciCommand::SetOption {
            name: "".to_string(),
            value: "".to_string(),
        },
    ))
}

fn cmd_register(input: &str) -> IResult<&str, UciCommand> {
    let (input, _) = tag("register")(input)?;

    // TODO: Parse out 'register' arguments
    Ok((
        input,
        UciCommand::Register {
            later: false,
            name: "".to_string(),
            code: "".to_string(),
        },
    ))
}

fn cmd_ucinewgame(input: &str) -> IResult<&str, UciCommand> {
    map(tag("ucinewgame"), |_| UciCommand::UciNewGame)(input)
}

fn cmd_position(input: &str) -> IResult<&str, UciCommand> {
    fn position_arg(input: &str) -> IResult<&str, Position> {
        alt((
            map(tag("startpos"), |_| Position::StartPos),
            map(
                separated_pair(tag("fen"), multispace1, non_space),
                |(_, fen)| Position::Fen(fen.to_string()),
            ),
        ))(input)
    }

    fn moves_arg(input: &str) -> IResult<&str, Vec<Move>> {
        map(
            separated_pair(
                tag("moves"),
                multispace1,
                separated_list1(multispace1, uci_move),
            ),
            |(_, moves)| moves,
        )(input)
    }

    let (input, _) = tag("position")(input)?;

    let (input, _) = multispace1(input)?;
    let (input, pos) = position_arg(input)?;

    let (input, _) = opt(multispace1)(input)?;
    let (input, moves) = opt(moves_arg)(input)?;

    Ok((
        input,
        UciCommand::Position {
            position: pos,
            moves: moves.unwrap_or_default(),
        },
    ))
}

// Wraps a closure which modifies `GoCmdArguments`.
struct GoCmdArgumentsModifyFn(Box<dyn FnOnce(&mut GoCmdArguments)>);
impl GoCmdArgumentsModifyFn {
    fn new(f: impl FnOnce(&mut GoCmdArguments) + 'static) -> Self {
        Self(Box::new(f))
    }
}

fn cmd_go(input: &str) -> IResult<&str, UciCommand> {
    let (input, _) = tag("go")(input)?;

    // Parse each potential argument. For each argument, return a function which sets
    // the relevant field in GoCmdArguments.
    let (input, args) = fold_many0(
        preceded(
            multispace1,
            alt((
                // searchmoves
                map(
                    separated_pair(
                        tag("searchmoves"),
                        multispace1,
                        separated_list1(multispace1, uci_move),
                    ),
                    |(_, searchmoves)| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.searchmoves = Some(searchmoves);
                        })
                    },
                ),
                // ponder
                map(tag("ponder"), |_| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.ponder = true;
                    })
                }),
                // wtime
                map(
                    separated_pair(tag("wtime"), multispace1, nom::character::complete::i32),
                    |(_, wtime)| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.wtime = Some(wtime);
                        })
                    },
                ),
                // btime
                map(
                    separated_pair(tag("btime"), multispace1, nom::character::complete::i32),
                    |(_, btime)| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.btime = Some(btime);
                        })
                    },
                ),
                // wint
                map(
                    separated_pair(tag("winc"), multispace1, nom::character::complete::u32),
                    |(_, winc)| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.winc = Some(winc);
                        })
                    },
                ),
                // binc
                map(
                    separated_pair(tag("binc"), multispace1, nom::character::complete::u32),
                    |(_, binc)| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.binc = Some(binc);
                        })
                    },
                ),
                // movestogo
                map(
                    separated_pair(tag("movestogo"), multispace1, nom::character::complete::u32),
                    |(_, movestogo)| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.movestogo = Some(movestogo);
                        })
                    },
                ),
                // depth
                map(
                    separated_pair(tag("depth"), multispace1, nom::character::complete::u32),
                    |(_, depth)| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.depth = Some(depth);
                        })
                    },
                ),
                // nodes
                map(
                    separated_pair(tag("nodes"), multispace1, nom::character::complete::u32),
                    |(_, nodes)| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.nodes = Some(nodes);
                        })
                    },
                ),
                // mate
                map(
                    separated_pair(tag("mate"), multispace1, nom::character::complete::u32),
                    |(_, mate)| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.mate = Some(mate);
                        })
                    },
                ),
                // movetime
                map(
                    separated_pair(tag("movetime"), multispace1, nom::character::complete::u32),
                    |(_, movetime)| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.movetime = Some(movetime);
                        })
                    },
                ),
                // infinite
                map(tag("infinite"), |_| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.infinite = true;
                    })
                }),
            )),
        ),
        || GoCmdArguments {
            searchmoves: None,
            ponder: false,
            wtime: None,
            btime: None,
            winc: None,
            binc: None,
            movestogo: None,
            depth: None,
            nodes: None,
            mate: None,
            movetime: None,
            infinite: false,
        },
        |mut acc, GoCmdArgumentsModifyFn(f)| {
            f(&mut acc);
            acc
        },
    )(input)?;

    Ok((input, UciCommand::Go(args)))
}

fn cmd_stop(input: &str) -> IResult<&str, UciCommand> {
    map(tag("stop"), |_| UciCommand::Stop)(input)
}

fn cmd_ponderhit(input: &str) -> IResult<&str, UciCommand> {
    map(tag("ponderhit"), |_| UciCommand::PonderHit)(input)
}

fn cmd_quit(input: &str) -> IResult<&str, UciCommand> {
    map(tag("quit"), |_| UciCommand::Quit)(input)
}

pub(super) fn any_uci_command(input: &str) -> IResult<&str, UciCommand> {
    let (input, _) = multispace0(input)?;

    let (input, cmd) = alt((
        cmd_uci,
        cmd_debug,
        cmd_isready,
        cmd_setoption,
        cmd_register,
        cmd_ucinewgame,
        cmd_position,
        cmd_go,
        cmd_stop,
        cmd_ponderhit,
        cmd_quit,
    ))(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = eof(input)?;

    Ok((input, cmd))
}

pub(super) fn parse(input: &str) -> Result<UciCommand> {
    let result = any_uci_command(input);

    match result {
        Ok((_, cmd)) => Ok(cmd),
        Err(e) => bail!("Unknown command: {} ({})", input, e),
    }
}
