use anyhow::{bail, Result};
use chess::{
    r#move::Move,
    square::{File, Rank, Square},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, space1, one_of},
    combinator::{eof, map, opt, value},
    multi::{fold_many0, separated_list1},
    sequence::{pair, preceded, tuple},
    IResult, InputTakeAtPosition,
};

use super::commands::{GoCmdArguments, Position, UciCommand};

// FIXME: Don't accept `isreadymorechars` as `IsReady`

fn non_space(input: &str) -> IResult<&str, &str> {
    input.split_at_position_complete(char::is_whitespace)
}

fn boolean(input: &str) -> IResult<&str, bool> {
    alt((value(true, tag("on")), value(false, tag("off"))))(input)
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
    value(UciCommand::Uci, pair(tag("uci"), eof))(input)
}

fn cmd_debug(input: &str) -> IResult<&str, UciCommand> {
    map(
        preceded(
            pair(tag("debug"), space1),
            boolean
        ),
        |on| UciCommand::Debug(on),
    )(input)
}

fn cmd_isready(input: &str) -> IResult<&str, UciCommand> {
    value(UciCommand::IsReady, tag("isready"))(input)
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
    value(UciCommand::UciNewGame, tag("ucinewgame"))(input)
}

fn cmd_position(input: &str) -> IResult<&str, UciCommand> {
    fn position_arg(input: &str) -> IResult<&str, Position> {
        alt((
            value(Position::StartPos, tag("startpos")),
            map(
                preceded(
                    pair(tag("fen"), space1),
                    non_space
                ),
                |fen| Position::Fen(fen.to_string()),
            ),
        ))(input)
    }

    fn moves_arg(input: &str) -> IResult<&str, Vec<Move>> {
        map(
            preceded(
                pair(tag("moves"), space1),
                separated_list1(space1, uci_move),
            ),
            |moves| moves,
        )(input)
    }

    let (input, _) = tag("position")(input)?;

    let (input, _) = space1(input)?;
    let (input, pos) = position_arg(input)?;

    let (input, _) = opt(space1)(input)?;
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
            space1,
            alt((
                // searchmoves
                map(
                    preceded(
                        pair(tag("searchmoves"), space1),
                        separated_list1(space1, uci_move),
                    ),
                    |searchmoves| {
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
                    preceded(
                        pair(tag("wtime"), space1),
                        nom::character::complete::i32,
                    ),
                    |wtime| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.wtime = Some(wtime);
                        })
                    },
                ),
                // btime
                map(
                    preceded(
                        pair(tag("btime"), space1),
                        nom::character::complete::i32,
                    ),
                    |btime| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.btime = Some(btime);
                        })
                    },
                ),
                // wint
                map(
                    preceded(
                        pair(tag("winc"), space1),
                        nom::character::complete::u32,
                    ),
                    |winc| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.winc = Some(winc);
                        })
                    },
                ),
                // binc
                map(
                    preceded(
                        pair(tag("binc"), space1),
                        nom::character::complete::u32,
                    ),
                    |binc| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.binc = Some(binc);
                        })
                    },
                ),
                // movestogo
                map(
                    preceded(
                        pair(tag("movestogo"), space1),
                        nom::character::complete::u32,
                    ),
                    |movestogo| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.movestogo = Some(movestogo);
                        })
                    },
                ),
                // depth
                map(
                    preceded(
                        pair(tag("depth"), space1),
                        nom::character::complete::u32,
                    ),
                    |depth| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.depth = Some(depth);
                        })
                    },
                ),
                // nodes
                map(
                    preceded(
                        pair(tag("nodes"), space1),
                        nom::character::complete::u32,
                    ),
                    |nodes| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.nodes = Some(nodes);
                        })
                    },
                ),
                // mate
                map(
                    preceded(
                        pair(tag("mate"), space1),
                        nom::character::complete::u32,
                    ),
                    |mate| {
                        GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                            acc.mate = Some(mate);
                        })
                    },
                ),
                // movetime
                map(
                    preceded(
                        pair(tag("movetime"), space1),
                        nom::character::complete::u32,
                    ),
                    |movetime| {
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
    value(UciCommand::Stop, tag("stop"))(input)
}

fn cmd_ponderhit(input: &str) -> IResult<&str, UciCommand> {
    value(UciCommand::PonderHit, tag("ponderhit"))(input)
}

fn cmd_quit(input: &str) -> IResult<&str, UciCommand> {
    value(UciCommand::Quit, tag("quit"))(input)
}

pub(super) fn any_uci_command(input: &str) -> IResult<&str, UciCommand> {
    let (input, _) = space0(input)?;

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

    let (input, _) = space0(input)?;
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
