use crate::chess::game::Game;
use crate::chess::moves::Move;
use crate::chess::piece::{PieceKind, PromotionPieceKind};
use crate::chess::player::Player;
use crate::chess::san;
use crate::chess::square::{squares, File, Rank, Square};
use color_eyre::eyre::{bail, eyre};
use color_eyre::Result;
use std::collections::HashSet;

enum AmbiguityResolution {
    None,
    File(File),
    Rank(Rank),
    Exact(File, Rank),
}

impl AmbiguityResolution {
    fn satisfied_by(&self, mv: Move) -> bool {
        match self {
            Self::None => true,
            Self::File(file) => mv.src.file() == *file,
            Self::Rank(rank) => mv.src.rank() == *rank,
            Self::Exact(file, rank) => mv.src.file() == *file && mv.src.rank() == *rank,
        }
    }
}

fn parse_ambiguity_resolution(chars: &[char]) -> Result<AmbiguityResolution> {
    match chars.len() {
        0 => Ok(AmbiguityResolution::None),
        1 => {
            let c = chars[0];

            if let Ok(file) = parse_file(c) {
                return Ok(AmbiguityResolution::File(file));
            }

            if let Ok(rank) = parse_rank(c) {
                return Ok(AmbiguityResolution::Rank(rank));
            }

            bail!("Invalid ambiguity resolution");
        }
        2 => {
            let file = chars[0];
            let rank = chars[1];

            let file = parse_file(file)?;
            let rank = parse_rank(rank)?;

            Ok(AmbiguityResolution::Exact(file, rank))
        }
        _ => bail!("Invalid ambiguity resolution"),
    }
}

fn parse_file(c: char) -> Result<File> {
    Ok(match c {
        'a' => File::A,
        'b' => File::B,
        'c' => File::C,
        'd' => File::D,
        'e' => File::E,
        'f' => File::F,
        'g' => File::G,
        'h' => File::H,
        _ => bail!("Invalid file"),
    })
}

fn parse_rank(c: char) -> Result<Rank> {
    Ok(match c {
        '1' => Rank::R1,
        '2' => Rank::R2,
        '3' => Rank::R3,
        '4' => Rank::R4,
        '5' => Rank::R5,
        '6' => Rank::R6,
        '7' => Rank::R7,
        '8' => Rank::R8,
        _ => bail!("Invalid rank"),
    })
}

fn parse_promotion_piece(piece: &str) -> Result<PromotionPieceKind> {
    if piece.len() != 1 {
        bail!("Invalid promotion piece");
    }

    let char = piece.chars().next().unwrap();
    Ok(match char {
        'Q' => PromotionPieceKind::Queen,
        'R' => PromotionPieceKind::Rook,
        'N' => PromotionPieceKind::Knight,
        'B' => PromotionPieceKind::Bishop,
        _ => bail!("Invalid promotion piece"),
    })
}

fn parse_piece(c: char) -> Option<PieceKind> {
    match c {
        'K' => Some(PieceKind::King),
        'Q' => Some(PieceKind::Queen),
        'R' => Some(PieceKind::Rook),
        'B' => Some(PieceKind::Bishop),
        'N' => Some(PieceKind::Knight),
        'P' => Some(PieceKind::Pawn),
        _ => None,
    }
}

fn parse_source_square(game: &Game, src: &str, dst: Square) -> Result<Square> {
    let piece_moves: Vec<(PieceKind, Move)> = game
        .moves()
        .into_iter()
        .map(|mv| (game.board.piece_at(mv.src).unwrap().kind, mv))
        .collect();

    // Pawn move
    if src.is_empty() {
        let matching_source_squares: HashSet<Square> = piece_moves
            .into_iter()
            .filter(|&(piece, mv)| piece == PieceKind::Pawn && mv.dst == dst)
            .map(|(_, mv)| mv.src)
            .collect();

        assert_eq!(matching_source_squares.len(), 1);
        return Ok(*matching_source_squares.iter().next().unwrap());
    }

    let src_chars: Vec<char> = src.chars().collect();
    let (first_char, rest) = src_chars.split_first().unwrap();

    if let Some(moved_piece) = parse_piece(*first_char) {
        let ambiguity_resolution = parse_ambiguity_resolution(rest)?;

        let matching_source_squares: Vec<Square> = piece_moves
            .into_iter()
            .filter(|&(piece, mv)| {
                piece == moved_piece && mv.dst == dst && ambiguity_resolution.satisfied_by(mv)
            })
            .map(|(_, mv)| mv.src)
            .collect();

        assert_eq!(matching_source_squares.len(), 1);
        return Ok(*matching_source_squares.first().unwrap());
    }

    let ambiguity_resolution = parse_ambiguity_resolution(&src_chars)?;

    let matching_source_squares: Vec<Square> = piece_moves
        .into_iter()
        .filter(|&(_, mv)| mv.dst == dst && ambiguity_resolution.satisfied_by(mv))
        .map(|(_, mv)| mv.src)
        .collect();

    assert_eq!(matching_source_squares.len(), 1);
    Ok(*matching_source_squares.first().unwrap())
}

fn parse_destination_square(sq: &str) -> Result<Square> {
    assert_eq!(sq.len(), 2);

    let mut chars = sq.chars();
    let file = parse_file(chars.next().unwrap())?;
    let rank = parse_rank(chars.next().unwrap())?;

    Ok(Square::from_file_and_rank(file, rank))
}

fn parse_move_squares(game: &Game, mv: &str) -> Result<(Square, Square)> {
    let (src, dst) = mv.split_at(mv.len() - 2);

    let dst = parse_destination_square(dst)?;
    let src = parse_source_square(game, src, dst)?;

    Ok((src, dst))
}

fn parse_capture_squares(game: &Game, mv: &str) -> Result<(Square, Square)> {
    let (src, dst) = mv
        .split_once(san::CAPTURE)
        .ok_or(eyre!("No x in capture move"))?;

    let dst = parse_destination_square(dst)?;
    let src = parse_source_square(game, src, dst)?;

    Ok((src, dst))
}

fn parse_squares(game: &Game, mv: &str) -> Result<(Square, Square)> {
    let is_capture_move = mv.contains(san::CAPTURE);
    if is_capture_move {
        return parse_capture_squares(game, mv);
    }

    parse_move_squares(game, mv)
}

fn parse_move_t<const PLAYER: bool>(game: &Game, mv: &str) -> Result<Move> {
    if mv == san::KINGSIDE_CASTLE {
        return Ok(Move::new(
            squares::king_start::<PLAYER>(),
            squares::kingside_castle_dest::<PLAYER>(),
        ));
    }

    if mv == san::QUEENSIDE_CASTLE {
        return Ok(Move::new(
            squares::king_start::<PLAYER>(),
            squares::queenside_castle_dest::<PLAYER>(),
        ));
    }

    let mv = mv
        .trim_end_matches(san::CHECK)
        .trim_end_matches(san::CHECKMATE);

    let (mv, promotion) = if mv.contains(san::PROMOTION) {
        let (rest, promotion_piece) = mv
            .split_once(san::PROMOTION)
            .ok_or(eyre!("Invalid promotion piece"))?;
        let promoted_to = parse_promotion_piece(promotion_piece)?;
        (rest, Some(promoted_to))
    } else {
        (mv, None)
    };

    let (src, dst) = parse_squares(game, mv)?;

    Ok(match promotion {
        None => Move::new(src, dst),
        Some(promoted_to) => Move::promotion(src, dst, promoted_to),
    })
}

#[allow(unused)]
pub fn parse_move(game: &Game, mv: &str) -> Result<Move> {
    match game.player {
        Player::White => parse_move_t::<true>(game, mv),
        Player::Black => parse_move_t::<false>(game, mv),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::fen;
    use crate::chess::game::Game;
    use crate::chess::piece::PromotionPieceKind;
    use crate::chess::square::squares::all::*;

    fn test_parse_san(fen: &'static str, expected_mv: Move, san: &'static str) {
        crate::init();

        let game = Game::from_fen(fen).unwrap();
        let mv = parse_move(&game, san).unwrap();

        assert_eq!(mv, expected_mv);
    }

    #[test]
    fn san_simple_pawn_move() {
        test_parse_san(fen::START_POS, Move::new(E2, E4), "e4");
    }

    #[test]
    fn san_simple_pawn_capture() {
        test_parse_san(
            "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2",
            Move::new(E4, D5),
            "exd5",
        );
    }

    #[test]
    fn san_simple_knight_move() {
        test_parse_san(fen::START_POS, Move::new(B1, C3), "Nc3");
    }

    #[test]
    fn san_test_en_passant() {
        test_parse_san(
            "rnbqkbnr/ppp2ppp/4p3/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3",
            Move::new(E5, D6),
            "exd6",
        );
    }

    #[test]
    fn san_promotion() {
        let promotion_fen = "k7/7P/8/8/8/8/8/7K w - - 0 1";

        test_parse_san(
            promotion_fen,
            Move::promotion(H7, H8, PromotionPieceKind::Knight),
            "h8=N",
        );

        test_parse_san(
            promotion_fen,
            Move::promotion(H7, H8, PromotionPieceKind::Bishop),
            "h8=B",
        );

        test_parse_san(
            promotion_fen,
            Move::promotion(H7, H8, PromotionPieceKind::Rook),
            "h8=R+",
        );

        test_parse_san(
            promotion_fen,
            Move::promotion(H7, H8, PromotionPieceKind::Queen),
            "h8=Q+",
        );
    }

    #[test]
    fn san_file_ambiguity() {
        test_parse_san(
            "R6R/8/8/8/8/8/8/1k4K1 w - - 0 1",
            Move::new(A8, B8),
            "Rab8+",
        );
    }

    #[test]
    fn san_rank_ambiguity() {
        test_parse_san(
            "R7/8/8/8/8/1k4K1/8/R7 w - - 0 1",
            Move::new(A1, A3),
            "R1a3+",
        );
    }

    #[test]
    fn san_exact_ambiguity() {
        test_parse_san(
            "1k1K4/8/8/8/4Q2Q/8/8/7Q w - - 0 1",
            Move::new(H4, E1),
            "Qh4e1",
        );
    }

    #[test]
    fn san_castling() {
        let fen = "1k6/8/8/8/8/8/8/R3K2R w KQ - 0 1";

        test_parse_san(fen, Move::new(E1, G1), "O-O");
        test_parse_san(fen, Move::new(E1, C1), "O-O-O");
    }

    #[test]
    fn san_plus_for_check() {
        test_parse_san(
            "k7/6P1/8/8/8/8/8/K7 w - - 0 1",
            Move::promotion(G7, G8, PromotionPieceKind::Queen),
            "g8=Q+",
        );
    }
}
