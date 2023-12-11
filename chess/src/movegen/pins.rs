use crate::bitboard::Bitboard;
use crate::board::Board;
use crate::movegen::tables;
use crate::player::Player;
use crate::square::Square;

pub fn get_pins_and_checkers(
    board: &Board,
    player: Player,
    king_square: Square,
) -> (Bitboard, Bitboard) {
    let mut checkers = Bitboard::EMPTY;
    let mut pinned = Bitboard::EMPTY;

    let our_pieces = board.player_pieces(player).all();
    let their_pieces = board.player_pieces(player.other());
    let all_their_pieces = their_pieces.all();

    // Any sliding piece on a ray around the King could be attacking the king, depending on if
    // there are any blocking pieces or not.
    let potential_pinners = tables::rook_attacks(king_square, Bitboard::EMPTY)
        & (their_pieces.rooks | their_pieces.queens)
        | tables::bishop_attacks(king_square, Bitboard::EMPTY)
            & (their_pieces.bishops | their_pieces.queens);

    for potential_pinner in potential_pinners {
        let squares_between = tables::between(king_square, potential_pinner);
        let their_pieces_on_ray = squares_between & all_their_pieces;

        // If there are any other enemy pieces between, they're blocking this attack.
        if their_pieces_on_ray.any() {
            continue;
        }

        let our_pieces_between = squares_between & our_pieces;

        if our_pieces_between.is_empty() {
            checkers |= potential_pinner.bb();
        } else if our_pieces_between.count() == 1 {
            pinned |= our_pieces_between;
        }
    }

    checkers |= tables::pawn_attacks(king_square, player) & their_pieces.pawns;
    checkers |= tables::knight_attacks(king_square) & their_pieces.knights;

    (checkers, pinned)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;
    use crate::square::squares::all::*;

    fn pin_test(fen: &'static str, expected_pinned: Bitboard) {
        crate::init();
        let game = Game::from_fen(fen).unwrap();

        let king_square = game.board.player_pieces(game.player).king.single();
        let (_, pinned) = get_pins_and_checkers(&game.board, game.player, king_square);

        assert_eq!(pinned, expected_pinned);
    }

    #[test]
    fn test_single_pinned_piece() {
        pin_test("8/8/8/1kq2PK1/8/8/8/8 w - - 0 1", F5.bb());
    }

    #[test]
    fn test_pins_all_around() {
        pin_test(
            "K7/2Q1Q1Q1/3ppp2/2QpkpQ1/3ppp2/2Q1Q1Q1/8/8 b - - 0 1",
            D6 | E6 | F6 | D5 | F5 | D4 | E4 | F4,
        );
    }
}
