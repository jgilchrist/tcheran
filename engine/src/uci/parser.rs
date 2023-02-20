use anyhow::{bail, Result};
use chess::{
    moves::Move,
    piece::PromotionPieceKind,
    square::{File, Rank, Square},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{one_of, space0, space1},
    combinator::{eof, map, opt, success, value},
    error::ParseError,
    multi::{fold_many0, separated_list1},
    sequence::{pair, preceded, tuple},
    IResult, InputTakeAtPosition, Parser,
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
    map(pair(uci_file, uci_rank), |(file, rank)| {
        Square::from_file_and_rank(file, rank)
    })(input)
}

fn uci_promotion(input: &str) -> IResult<&str, PromotionPieceKind> {
    let (input, rank) = one_of("nbrq")(input)?;

    Ok((
        input,
        match rank {
            'n' => PromotionPieceKind::Knight,
            'b' => PromotionPieceKind::Bishop,
            'r' => PromotionPieceKind::Rook,
            'q' => PromotionPieceKind::Queen,
            _ => unreachable!(),
        },
    ))
}

fn uci_move(input: &str) -> IResult<&str, Move> {
    map(
        tuple((uci_square, uci_square, opt(uci_promotion))),
        |(from, to, promotion)| match promotion {
            None => Move::new(from, to),
            Some(p) => Move::new_with_promotion(from, to, p),
        },
    )(input)
}

pub fn maybe_uci_moves(input: &str) -> IResult<&str, Option<Vec<Move>>> {
    nom::combinator::opt(uci_moves)(input)
}

pub fn uci_moves(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(space1, uci_move)(input)
}

fn command_with_argument<'a, F, G, OInner, O, E: ParseError<&'a str>>(
    cmd: &'static str,
    argument_combinator: F,
    map_argument_fn: G,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, OInner, E>,
    G: FnMut(OInner) -> O,
{
    map(
        preceded(pair(tag(cmd), space1), argument_combinator),
        map_argument_fn,
    )
}

fn cmd_uci(input: &str) -> IResult<&str, UciCommand> {
    value(UciCommand::Uci, pair(tag("uci"), eof))(input)
}

fn cmd_debug(input: &str) -> IResult<&str, UciCommand> {
    command_with_argument("debug", boolean, UciCommand::Debug)(input)
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
            name: String::new(),
            value: String::new(),
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
            name: String::new(),
            code: String::new(),
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
            command_with_argument("fen", non_space, |fen| Position::Fen(fen.to_string())),
        ))(input)
    }

    fn moves_arg(input: &str) -> IResult<&str, Vec<Move>> {
        command_with_argument("moves", uci_moves, |moves| moves)(input)
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
                command_with_argument("searchmoves", uci_moves, |searchmoves| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.searchmoves = Some(searchmoves);
                    })
                }),
                command_with_argument("ponder", success(()), |_| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.ponder = true;
                    })
                }),
                command_with_argument("wtime", nom::character::complete::i32, |wtime| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.wtime = Some(wtime);
                    })
                }),
                command_with_argument("btime", nom::character::complete::i32, |btime| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.btime = Some(btime);
                    })
                }),
                command_with_argument("winc", nom::character::complete::u32, |winc| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.winc = Some(winc);
                    })
                }),
                command_with_argument("binc", nom::character::complete::u32, |binc| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.binc = Some(binc);
                    })
                }),
                command_with_argument("movestogo", nom::character::complete::u32, |movestogo| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.movestogo = Some(movestogo);
                    })
                }),
                command_with_argument("depth", nom::character::complete::u32, |depth| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.depth = Some(depth);
                    })
                }),
                command_with_argument("nodes", nom::character::complete::u32, |nodes| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.nodes = Some(nodes);
                    })
                }),
                command_with_argument("mate", nom::character::complete::u32, |mate| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.mate = Some(mate);
                    })
                }),
                command_with_argument("movetime", nom::character::complete::u32, |movetime| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.movetime = Some(movetime);
                    })
                }),
                command_with_argument("infinite", success(()), |_| {
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

pub fn parse_move(input: &str) -> Result<Move> {
    let result = uci_move(input);

    match result {
        Ok((_, mv)) => Ok(mv),
        Err(e) => bail!("Unknown move: {} ({})", input, e),
    }
}

pub fn parse(input: &str) -> Result<UciCommand> {
    let result = any_uci_command(input);

    match result {
        Ok((_, cmd)) => Ok(cmd),
        Err(e) => bail!("Unknown command: {} ({})", input, e),
    }
}
