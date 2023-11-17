use crate::bitboard::Bitboard;
use crate::board::Board;
use crate::movegen::tables;
use crate::player::Player;
use crate::square::Square;

pub fn get_pins(board: &Board, player: Player, king_square: Square) -> (Bitboard, Bitboard) {
    let mut pinned = Bitboard::EMPTY;
    let mut pinners = Bitboard::EMPTY;

    let our_pieces = board.player_pieces(player).all();
    let their_pieces = board.player_pieces(player.other());

    // Any sliding piece on a ray around the King could be attacking the king, depending on if
    // there are any blocking pieces or not.
    let potential_pinners = tables::rook_attacks(king_square, Bitboard::EMPTY)
        & (their_pieces.rooks | their_pieces.queens)
        | tables::bishop_attacks(king_square, Bitboard::EMPTY)
            & (their_pieces.bishops | their_pieces.queens);

    for potential_pinner in potential_pinners {
        let squares_between = tables::between(king_square, potential_pinner);
        let their_pieces_on_ray = squares_between & their_pieces.all();

        // If there are any other enemy pieces between, they're blocking this attack.
        if their_pieces_on_ray.any() {
            continue;
        }

        let our_pieces_between = squares_between & our_pieces;

        if our_pieces_between.count() == 1 {
            pinned |= our_pieces_between;
            pinners |= potential_pinner;
        }
    }

    (pinned, pinners)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;
    use crate::square::squares::all::*;
    use spectral::assert_that;

    fn pin_test(fen: &'static str, expected_pinned: Bitboard, expected_pinners: Bitboard) {
        crate::init();
        let game = Game::from_fen(fen).unwrap();

        let king_square = game.board.player_pieces(game.player).king.single();
        let (pinned, pinners) = get_pins(&game.board, game.player, king_square);

        assert_that(&pinned).is_equal_to(expected_pinned);
        assert_that(&pinners).is_equal_to(expected_pinners);
    }

    #[test]
    fn test_single_pinned_piece() {
        pin_test("8/8/8/1kq2PK1/8/8/8/8 w - - 0 1", F5.0, C5.0);
    }

    #[test]
    fn test_pins_all_around() {
        pin_test(
            "K7/2Q1Q1Q1/3ppp2/2QpkpQ1/3ppp2/2Q1Q1Q1/8/8 b - - 0 1",
            D6 | E6 | F6 | D5 | F5 | D4 | E4 | F4,
            C7 | E7 | G7 | C5 | G5 | C3 | E3 | G3,
        );
    }
}
