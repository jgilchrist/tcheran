use nom::Parser;
use std::collections::HashSet;

use crate::chess::{
    board::Board,
    game::{CastleRights, Game},
    piece::Piece,
    player::Player,
    square::{File, Rank, Square},
};

use crate::chess::player::ByPlayer;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::sequence::terminated;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of, space1},
    combinator::{eof, map, value},
    multi::many1,
    sequence::{pair, preceded},
    IResult,
};

#[derive(Debug)]
pub struct FenRank(Vec<Option<Piece>>);

fn fen_piece(input: &str) -> IResult<&str, Piece> {
    let (input, piece) = one_of("RNBQKPrnbqkp").parse(input)?;

    Ok((
        input,
        match piece {
            'R' => Piece::WHITE_ROOK,
            'N' => Piece::WHITE_KNIGHT,
            'B' => Piece::WHITE_BISHOP,
            'Q' => Piece::WHITE_QUEEN,
            'K' => Piece::WHITE_KING,
            'P' => Piece::WHITE_PAWN,
            'r' => Piece::BLACK_ROOK,
            'n' => Piece::BLACK_KNIGHT,
            'b' => Piece::BLACK_BISHOP,
            'q' => Piece::BLACK_QUEEN,
            'k' => Piece::BLACK_KING,
            'p' => Piece::BLACK_PAWN,
            _ => unreachable!(),
        },
    ))
}

fn fen_empty_squares(input: &str) -> IResult<&str, Vec<Option<Piece>>> {
    map(one_of("12345678"), |digit| {
        let sq: Option<Piece> = None;
        vec![sq; digit.to_string().parse::<usize>().unwrap()]
    })
    .parse(input)
}

fn fen_line(input: &str) -> IResult<&str, FenRank> {
    let (input, squares) = many1(alt((
        map(fen_piece, |p| vec![Some(p); 1]),
        fen_empty_squares,
    )))
    .parse(input)?;

    Ok((input, FenRank(squares.concat())))
}

fn fen_position(input: &str) -> IResult<&str, Board> {
    let (input, board) = map(
        (
            fen_line,
            preceded(char('/'), fen_line),
            preceded(char('/'), fen_line),
            preceded(char('/'), fen_line),
            preceded(char('/'), fen_line),
            preceded(char('/'), fen_line),
            preceded(char('/'), fen_line),
            preceded(char('/'), fen_line),
        ),
        |(line8, line7, line6, line5, line4, line3, line2, line1)| {
            let mut all_pieces: Vec<Option<Piece>> = Vec::new();
            all_pieces.extend(line1.0);
            all_pieces.extend(line2.0);
            all_pieces.extend(line3.0);
            all_pieces.extend(line4.0);
            all_pieces.extend(line5.0);
            all_pieces.extend(line6.0);
            all_pieces.extend(line7.0);
            all_pieces.extend(line8.0);

            assert_eq!(all_pieces.len(), Square::N);

            let pieces_array: [Option<Piece>; Square::N] = all_pieces.try_into().unwrap();

            pieces_array.try_into().unwrap()
        },
    )
    .parse(input)?;

    Ok((input, board))
}

fn fen_color(input: &str) -> IResult<&str, Player> {
    alt((
        value(Player::White, tag("w")),
        value(Player::Black, tag("b")),
    ))
    .parse(input)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum FenCastleRight {
    WhiteKingside,
    WhiteQueenside,
    BlackKingside,
    BlackQueenside,
}

fn fen_castle_right(input: &str) -> IResult<&str, FenCastleRight> {
    let (input, piece) = one_of("KQkq").parse(input)?;

    Ok((
        input,
        match piece {
            'K' => FenCastleRight::WhiteKingside,
            'Q' => FenCastleRight::WhiteQueenside,
            'k' => FenCastleRight::BlackKingside,
            'q' => FenCastleRight::BlackQueenside,
            _ => unreachable!(),
        },
    ))
}

fn fen_castling(input: &str) -> IResult<&str, ByPlayer<CastleRights>> {
    alt((
        value(
            ByPlayer::new(CastleRights::none(), CastleRights::none()),
            tag("-"),
        ),
        map(many1(fen_castle_right), |rs| {
            let rights: HashSet<FenCastleRight> = rs.iter().copied().collect();

            ByPlayer::new(
                CastleRights {
                    king_side: rights.contains(&FenCastleRight::WhiteKingside),
                    queen_side: rights.contains(&FenCastleRight::WhiteQueenside),
                },
                CastleRights {
                    king_side: rights.contains(&FenCastleRight::BlackKingside),
                    queen_side: rights.contains(&FenCastleRight::BlackQueenside),
                },
            )
        }),
    ))
    .parse(input)
}

fn fen_file(input: &str) -> IResult<&str, File> {
    let (input, file) = one_of("abcdefgh").parse(input)?;

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

fn fen_rank(input: &str) -> IResult<&str, Rank> {
    let (input, rank) = one_of("12345678").parse(input)?;

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

fn fen_square(input: &str) -> IResult<&str, Square> {
    map(pair(fen_file, fen_rank), |(file, rank)| {
        Square::from_file_and_rank(file, rank)
    })
    .parse(input)
}

fn fen_en_passant_target(input: &str) -> IResult<&str, Option<Square>> {
    alt((value(None, tag("-")), map(fen_square, Some))).parse(input)
}

fn fen_halfmove_clock(input: &str) -> IResult<&str, u32> {
    nom::character::complete::u32(input)
}

fn fen_fullmove_number(input: &str) -> IResult<&str, u32> {
    nom::character::complete::u32(input)
}

fn fen_parser(input: &str) -> IResult<&str, Game> {
    let (input, (board, player, castle_rights, en_passant_target, halfmove_clock, fullmove_number)) =
        terminated(
            (
                fen_position,
                preceded(space1, fen_color),
                preceded(space1, fen_castling),
                preceded(space1, fen_en_passant_target),
                opt(preceded(space1, fen_halfmove_clock)),
                opt(preceded(space1, fen_fullmove_number)),
            ),
            (space0, eof),
        )
        .parse(input)?;

    let halfmove_clock = halfmove_clock.unwrap_or(0);
    let fullmove_number = fullmove_number.unwrap_or(1);

    let plies = plies_from_fullmove_number(fullmove_number, player);

    Ok((
        input,
        Game::from_state(
            board,
            player,
            castle_rights,
            en_passant_target,
            halfmove_clock,
            plies,
        ),
    ))
}

#[inline(always)]
fn plies_from_fullmove_number(fullmove_number: u32, player: Player) -> u32 {
    (fullmove_number - 1) * 2 + u32::from(player == Player::Black)
}

pub fn parse(input: &str) -> Result<Game, String> {
    let result = fen_parser(input);

    match result {
        Ok((_, game)) => Ok(game),
        Err(e) => Err(format!("Invalid FEN: {input} ({e})")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Player::*;

    #[test]
    fn parse_startpos() {
        crate::init();

        let game_result = parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert!(game_result.is_ok());

        let game = game_result.unwrap();
        let default_game = Game::default();

        assert_eq!(game.board.pawns(White), default_game.board.pawns(White));
        assert_eq!(game.board.knights(White), default_game.board.knights(White));
        assert_eq!(game.board.bishops(White), default_game.board.bishops(White));
        assert_eq!(game.board.rooks(White), default_game.board.rooks(White));
        assert_eq!(game.board.queens(White), default_game.board.queens(White));
        assert_eq!(game.board.king(White), default_game.board.king(White));

        assert_eq!(game.board.pawns(Black), default_game.board.pawns(Black));
        assert_eq!(game.board.knights(Black), default_game.board.knights(Black));
        assert_eq!(game.board.bishops(Black), default_game.board.bishops(Black));
        assert_eq!(game.board.rooks(Black), default_game.board.rooks(Black));
        assert_eq!(game.board.queens(Black), default_game.board.queens(Black));
        assert_eq!(game.board.king(Black), default_game.board.king(Black));

        assert_eq!(game.plies, 0);
    }

    #[test]
    fn parse_kiwipete() {
        crate::init();

        assert!(
            parse("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").is_ok()
        );
    }

    #[test]
    fn parse_kiwipete_without_halfmove_and_fullmove() {
        crate::init();

        assert!(parse("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ").is_ok());
    }

    #[test]
    fn plies_from_fullmove_number() {
        assert_eq!(super::plies_from_fullmove_number(1, Player::White), 0);
        assert_eq!(super::plies_from_fullmove_number(1, Player::Black), 1);
        assert_eq!(super::plies_from_fullmove_number(2, Player::White), 2);
        assert_eq!(super::plies_from_fullmove_number(2, Player::Black), 3);
    }
}
