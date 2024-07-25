use crate::chess::{
    piece::PromotionPieceKind,
    square::{File, Rank, Square},
};
use crate::engine::uci::UciMove;
use crate::uci::commands::{DebugCommand, Position};
use nom::bytes::complete::take_until;
use nom::character::complete::alpha1;
use nom::combinator::rest;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{one_of, space0, space1},
    combinator::{eof, map, opt, value},
    error::ParseError,
    multi::{fold_many0, separated_list1},
    sequence::{pair, preceded, tuple},
    IResult, Parser,
};
use std::time::Duration;

use super::commands::{GoCmdArguments, UciCommand};

// FIXME: Don't accept `isreadymorechars` as `IsReady`

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

fn uci_move(input: &str) -> IResult<&str, UciMove> {
    map(
        tuple((uci_square, uci_square, opt(uci_promotion))),
        |(src, dst, promotion)| UciMove {
            src,
            dst,
            promotion,
        },
    )(input)
}

pub fn uci_moves(input: &str) -> IResult<&str, Vec<UciMove>> {
    separated_list1(space1, uci_move)(input)
}

fn command_without_arguments<'a, G, O, E: ParseError<&'a str>>(
    cmd: &'a str,
    map_argument_fn: G,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    G: FnMut(&'a str) -> O,
{
    map(tag(cmd), map_argument_fn)
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

    let (input, _) = space1(input)?;

    let (input, name) = command_with_argument("name", take_until(" value"), |name| name)(input)?;

    let (input, _) = space1(input)?;
    let (input, _) = tag("value")(input)?;
    let (input, _) = space1(input)?;

    let (input, value) = rest(input)?;

    Ok((
        input,
        UciCommand::SetOption {
            name: name.to_string(),
            value: value.to_string(),
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
            command_with_argument("fen", alt((take_until(" moves"), rest)), |fen| {
                Position::Fen(fen.to_string())
            }),
        ))(input)
    }

    fn moves_arg(input: &str) -> IResult<&str, Vec<UciMove>> {
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

fn parse_duration(n: i64) -> Duration {
    Duration::from_millis(n.max(0).try_into().unwrap())
}

fn cmd_go(input: &str) -> IResult<&str, UciCommand> {
    let (input, _) = tag("go")(input)?;

    // Parse each potential argument. For each argument, return a function which sets
    // the relevant field in GoCmdArguments.
    let (input, args) = fold_many0(
        preceded(
            space1,
            alt((
                command_without_arguments("ponder", |_| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.ponder = true;
                    })
                }),
                command_with_argument("wtime", nom::character::complete::i64, |wtime| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.wtime = Some(parse_duration(wtime));
                    })
                }),
                command_with_argument("btime", nom::character::complete::i64, |btime| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.btime = Some(parse_duration(btime));
                    })
                }),
                command_with_argument("winc", nom::character::complete::i64, |winc| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.winc = Some(parse_duration(winc));
                    })
                }),
                command_with_argument("binc", nom::character::complete::i64, |binc| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.binc = Some(parse_duration(binc));
                    })
                }),
                command_with_argument("movestogo", nom::character::complete::u32, |movestogo| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.movestogo = Some(movestogo);
                    })
                }),
                command_with_argument("depth", nom::character::complete::u8, |depth| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.depth = Some(depth);
                    })
                }),
                command_with_argument("nodes", nom::character::complete::u32, |nodes| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.nodes = Some(nodes);
                    })
                }),
                command_with_argument("movetime", nom::character::complete::i64, |movetime| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.movetime = Some(parse_duration(movetime));
                    })
                }),
                command_without_arguments("infinite", |_| {
                    GoCmdArgumentsModifyFn::new(move |acc: &mut GoCmdArguments| {
                        acc.infinite = true;
                    })
                }),
            )),
        ),
        || GoCmdArguments {
            ponder: false,
            wtime: None,
            btime: None,
            winc: None,
            binc: None,
            movestogo: None,
            depth: None,
            nodes: None,
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

fn cmd_d_fen(input: &str) -> IResult<&str, UciCommand> {
    let (input, _) = tag("fen")(input)?;
    Ok((input, UciCommand::D(DebugCommand::PrintPosition)))
}

fn cmd_d_position(input: &str) -> IResult<&str, UciCommand> {
    let (input, _) = tag("position")(input)?;
    let (input, _) = space1(input)?;
    let (input, pos) = alpha1(input)?;
    Ok((
        input,
        UciCommand::D(DebugCommand::SetPosition {
            position: pos.to_owned(),
        }),
    ))
}

fn cmd_d_move(input: &str) -> IResult<&str, UciCommand> {
    let (input, _) = tag("move")(input)?;
    let (input, _) = space1(input)?;
    let (input, moves) = uci_moves(input)?;

    Ok((input, UciCommand::D(DebugCommand::Move { moves })))
}

fn cmd_d_perft(input: &str) -> IResult<&str, UciCommand> {
    let (input, _) = tag("perft")(input)?;

    let (input, _) = space1(input)?;
    let (input, depth) = nom::character::complete::u8(input)?;

    Ok((input, UciCommand::D(DebugCommand::Perft { depth })))
}

fn cmd_d_perft_div(input: &str) -> IResult<&str, UciCommand> {
    let (input, _) = tag("perftdiv")(input)?;

    let (input, _) = space1(input)?;
    let (input, depth) = nom::character::complete::u8(input)?;

    Ok((input, UciCommand::D(DebugCommand::PerftDiv { depth })))
}

fn cmd_d_eval(input: &str) -> IResult<&str, UciCommand> {
    let (input, _) = tag("eval")(input)?;
    Ok((input, UciCommand::D(DebugCommand::Eval)))
}

fn cmd_d(input: &str) -> IResult<&str, UciCommand> {
    let (input, _) = tag("d")(input)?;
    let (input, _) = space0(input)?;

    alt((
        cmd_d_fen,
        cmd_d_position,
        cmd_d_move,
        cmd_d_perft,
        cmd_d_perft_div,
        cmd_d_eval,
    ))(input)
}

fn cmd_ponderhit(input: &str) -> IResult<&str, UciCommand> {
    value(UciCommand::PonderHit, tag("ponderhit"))(input)
}

fn cmd_bench(input: &str) -> IResult<&str, UciCommand> {
    value(UciCommand::Bench, tag("bench"))(input)
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
        cmd_ucinewgame,
        cmd_position,
        cmd_go,
        cmd_stop,
        cmd_ponderhit,
        cmd_bench,
        cmd_d,
        cmd_quit,
    ))(input)?;

    let (input, _) = space0(input)?;
    let (input, _) = eof(input)?;

    Ok((input, cmd))
}

pub fn parse(input: &str) -> Result<UciCommand, String> {
    let result = any_uci_command(input);

    match result {
        Ok((_, cmd)) => Ok(cmd),
        Err(e) => Err(format!("Unknown command: {input} ({e})")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_go_infinite() {
        assert!(parse("go infinite").is_ok());
    }

    #[test]
    fn test_uci() {
        let ml = parse("uci").unwrap();
        assert_eq!(ml, UciCommand::Uci);
    }

    #[test]
    fn test_debug_on() {
        let ml = parse("debug    on").unwrap();
        assert_eq!(ml, UciCommand::Debug(true));
    }

    #[test]
    fn test_debug_off() {
        let ml = parse("debug off").unwrap();
        assert_eq!(ml, UciCommand::Debug(false));
    }

    #[test]
    fn test_debugon() {
        let _unused = parse("debugon").expect_err("Should not parse 'debugon'");
    }

    #[test]
    fn test_debug_wrong_param() {
        let ml = parse("debug abc");
        assert!(ml.is_err());
    }

    #[test]
    fn test_debug_cutoff() {
        let _unused = parse("debug    ontario").expect_err("Should not parse");
    }

    #[test]
    fn test_isready() {
        let ml = parse(" \tisready  ").unwrap();
        assert_eq!(ml, UciCommand::IsReady);
    }

    #[test]
    fn test_position_fen() {
        let ml = parse("position fen 6r1/p2p4/3Ppk2/p1R2p2/8/3b4/1r6/4K3 b - - 5 45");
        assert!(ml.is_ok());
    }

    #[test]
    fn test_position_fen_then_moves() {
        let ml =
            parse("position fen 6r1/p2p4/3Ppk2/p1R2p2/8/3b4/1r6/4K3 b - - 5 45 moves a7a6 c1d1");
        assert!(ml.is_ok());
    }
}
