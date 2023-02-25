use std::collections::HashSet;

use anyhow::{bail, Result};

use crate::{
    board::Board,
    game::{CastleRights, Game},
    piece::Piece,
    player::Player,
    square::{File, Rank, Square},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of, space1},
    combinator::{eof, map, opt, value},
    multi::many1,
    sequence::{pair, preceded, tuple},
    IResult,
};

#[derive(Debug)]
pub struct FenRank(Vec<Option<Piece>>);

fn fen_piece(input: &str) -> IResult<&str, Piece> {
    let (input, piece) = one_of("RNBQKPrnbqkp")(input)?;

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
    })(input)
}

fn fen_line(input: &str) -> IResult<&str, FenRank> {
    let (input, squares) = many1(alt((
        map(fen_piece, |p| vec![Some(p); 1]),
        fen_empty_squares,
    )))(input)?;

    Ok((input, FenRank(squares.concat())))
}

fn fen_position(input: &str) -> IResult<&str, Board> {
    let (input, board) = map(
        tuple((
            fen_line,
            preceded(char('/'), fen_line),
            preceded(char('/'), fen_line),
            preceded(char('/'), fen_line),
            preceded(char('/'), fen_line),
            preceded(char('/'), fen_line),
            preceded(char('/'), fen_line),
            preceded(char('/'), fen_line),
        )),
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

            assert!(all_pieces.len() == 64);

            // TODO: Error handling
            let pieces_array: [Option<Piece>; 64] = all_pieces.try_into().unwrap();

            Board::from_array(pieces_array)
        },
    )(input)?;

    Ok((input, board))
}

fn fen_color(input: &str) -> IResult<&str, Player> {
    alt((
        value(Player::White, tag("w")),
        value(Player::Black, tag("b")),
    ))(input)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum FenCastleRight {
    WhiteKingside,
    WhiteQueenside,
    BlackKingside,
    BlackQueenside,
}

fn fen_castle_right(input: &str) -> IResult<&str, FenCastleRight> {
    let (input, piece) = one_of("KQkq")(input)?;

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

fn fen_castling(input: &str) -> IResult<&str, (CastleRights, CastleRights)> {
    alt((
        value((CastleRights::none(), CastleRights::none()), tag("-")),
        map(many1(fen_castle_right), |rs| {
            let rights: HashSet<FenCastleRight> = rs.iter().copied().collect();

            (
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
    ))(input)
}

fn fen_file(input: &str) -> IResult<&str, File> {
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

fn fen_rank(input: &str) -> IResult<&str, Rank> {
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

fn fen_square(input: &str) -> IResult<&str, Square> {
    map(pair(fen_file, fen_rank), |(file, rank)| {
        Square::from_file_and_rank(file, rank)
    })(input)
}

fn fen_en_passant_target(input: &str) -> IResult<&str, Option<Square>> {
    alt((value(None, tag("-")), map(fen_square, Some)))(input)
}

fn fen_halfmove_clock(input: &str) -> IResult<&str, u32> {
    nom::character::complete::u32(input)
}

fn fen_fullmove_number(input: &str) -> IResult<&str, u32> {
    nom::character::complete::u32(input)
}

fn fen(input: &str) -> IResult<&str, Game> {
    let (
        input,
        (board, player, castle_rights, en_passant_target, _halfmove_clock, _fullmove_number, _),
    ) = tuple((
        fen_position,
        preceded(space1, fen_color),
        preceded(space1, fen_castling),
        preceded(space1, fen_en_passant_target),
        opt(preceded(space1, fen_halfmove_clock)),
        opt(preceded(space1, fen_fullmove_number)),
        eof,
    ))(input)?;

    let (white_castle_rights, black_castle_rights) = castle_rights;

    Ok((
        input,
        Game {
            player,
            board,
            white_castle_rights,
            black_castle_rights,
            en_passant_target,
        },
    ))
}

pub fn parse(input: &str) -> Result<Game> {
    let result = fen(input);

    match result {
        Ok((_, game)) => Ok(game),
        Err(e) => bail!("Invalid FEN: {} ({})", input, e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_startpos() {
        let game_result = parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert!(game_result.is_ok());

        let game = game_result.unwrap();
        let default_game = Game::default();

        dbg!(&game);
        dbg!(&default_game);

        assert!(game.board.white_pieces.pawns == default_game.board.white_pieces.pawns);
        assert!(game.board.white_pieces.knights == default_game.board.white_pieces.knights);
        assert!(game.board.white_pieces.bishops == default_game.board.white_pieces.bishops);
        assert!(game.board.white_pieces.rooks == default_game.board.white_pieces.rooks);
        assert!(game.board.white_pieces.queens == default_game.board.white_pieces.queens);
        assert!(game.board.white_pieces.king == default_game.board.white_pieces.king);

        assert!(game.board.black_pieces.pawns == default_game.board.black_pieces.pawns);
        assert!(game.board.black_pieces.knights == default_game.board.black_pieces.knights);
        assert!(game.board.black_pieces.bishops == default_game.board.black_pieces.bishops);
        assert!(game.board.black_pieces.rooks == default_game.board.black_pieces.rooks);
        assert!(game.board.black_pieces.queens == default_game.board.black_pieces.queens);
        assert!(game.board.black_pieces.king == default_game.board.black_pieces.king);
    }

    #[test]
    fn parse_kiwipete() {
        assert!(parse("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -").is_ok());
    }
}
