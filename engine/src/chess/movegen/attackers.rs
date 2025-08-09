use crate::chess::bitboard::Bitboard;
use crate::chess::board::Board;
use crate::chess::movegen::tables;
use crate::chess::player::Player;
use crate::chess::square::Square;

pub fn generate_attackers_of(board: &Board, player: Player, square: Square) -> Bitboard {
    let mut attackers = Bitboard::EMPTY;
    let them = player.other();
    let all_pieces = board.occupancy();

    // Pawns: A square is attacked by pawns in the same positions as a pawn could capture if it was on
    // that square
    attackers |= tables::pawn_attacks(square, player) & board.pawns(them);

    // Knights: A square is attacked by any squares a knight could reach if it were on that square
    attackers |= tables::knight_attacks(square) & board.knights(them);

    // Sliders: A square is attacked by any squares sliding pieces can reach
    attackers |= tables::bishop_attacks(square, all_pieces) & board.diagonal_sliders(them);
    attackers |= tables::rook_attacks(square, all_pieces) & board.orthogonal_sliders(them);

    attackers |= tables::king_attacks(square) & board.king(them);

    attackers
}

pub fn all_attackers_of(board: &Board, square: Square, occupied: Bitboard) -> Bitboard {
    use Player::*;

    let mut attackers = Bitboard::EMPTY;

    attackers |= tables::pawn_attacks(square, White) & board.pawns(Black);
    attackers |= tables::pawn_attacks(square, Black) & board.pawns(White);

    attackers |= tables::knight_attacks(square) & board.all_knights();
    attackers |= tables::bishop_attacks(square, occupied) & board.all_diagonal_sliders();
    attackers |= tables::rook_attacks(square, occupied) & board.all_orthogonal_sliders();
    attackers |= tables::king_attacks(square) & board.all_kings();

    attackers
}
