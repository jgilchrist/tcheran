use crate::chess::bitboard::Bitboard;
use crate::chess::board::Board;
use crate::chess::movegen::tables;
use crate::chess::movegen::tables::{bishop_attacks, rook_attacks};
use crate::chess::square::Square;

pub fn get_pins<const PLAYER: bool>(board: &Board, king_square: Square) -> (Bitboard, Bitboard) {
    let our_pieces = board.player_pieces::<PLAYER>().all();
    let their_pieces = board.opponent_pieces::<PLAYER>();
    let all_their_pieces = their_pieces.all();
    let all_pieces = our_pieces | all_their_pieces;

    let mut orthogonal_pins = Bitboard::EMPTY;
    let potential_orthogonal_pinned = rook_attacks(king_square, all_pieces) & our_pieces;
    let without_potential_orthogonal_pinned_pieces = all_pieces & !potential_orthogonal_pinned;
    let orthogonal_pinners = rook_attacks(king_square, without_potential_orthogonal_pinned_pieces)
        & (their_pieces.rooks() | their_pieces.queens());

    let mut diagonal_pins = Bitboard::EMPTY;
    let potential_diagonal_pinned = bishop_attacks(king_square, all_pieces) & our_pieces;
    let without_potential_diagonal_pinned_pieces = all_pieces & !potential_diagonal_pinned;
    let diagonal_pinners = bishop_attacks(king_square, without_potential_diagonal_pinned_pieces)
        & (their_pieces.bishops() | their_pieces.queens());

    for pinner in orthogonal_pinners {
        orthogonal_pins |= pinner.bb();
        orthogonal_pins |= tables::between(king_square, pinner);
    }

    for pinner in diagonal_pinners {
        diagonal_pins |= pinner.bb();
        diagonal_pins |= tables::between(king_square, pinner);
    }

    (orthogonal_pins, diagonal_pins)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::bitboard::bitboards::*;
    use crate::chess::game::Game;
    use crate::chess::player::Player;

    fn pin_test(
        fen: &'static str,
        expected_orthogonal_pins: Bitboard,
        expected_diagonal_pins: Bitboard,
    ) {
        crate::init();
        let game = Game::from_fen(fen).unwrap();

        let king_square = game.board.player_pieces_ref(game.player).king().single();
        let (orthogonal_pins, diagonal_pins) = match game.player {
            Player::White => get_pins::<true>(&game.board, king_square),
            Player::Black => get_pins::<false>(&game.board, king_square),
        };

        assert_eq!(orthogonal_pins, expected_orthogonal_pins);
        assert_eq!(diagonal_pins, expected_diagonal_pins);
    }

    #[test]
    fn test_pin_in_gist_8_depth_3() {
        pin_test(
            "rnbq1k1r/pp1P1ppp/2p5/8/1bB5/8/PPPNNnPP/R1BQK2R w KQ - 3 9",
            Bitboard::EMPTY,
            B4_BB | C3_BB | D2_BB,
        );
    }
}
