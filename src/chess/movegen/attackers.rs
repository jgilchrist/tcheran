use crate::chess::bitboard::Bitboard;
use crate::chess::board::Board;
use crate::chess::movegen::tables;
use crate::chess::player::Player;
use crate::chess::square::Square;

pub fn generate_attackers_of(board: &Board, player: Player, square: Square) -> Bitboard {
    let mut attackers = Bitboard::EMPTY;
    let our_pieces = board.pieces(player);
    let their_pieces = board.pieces(player.other());
    let all_pieces = our_pieces.all() | their_pieces.all();

    // Pawns: A square is attacked by pawns in the same positions as a pawn could capture if it was on
    // that square
    attackers |= tables::pawn_attacks(square, player) & their_pieces.pawns();

    // Knights: A square is attacked by any squares a knight could reach if it were on that square
    attackers |= tables::knight_attacks(square) & their_pieces.knights();

    // Sliders: A square is attacked by any squares a
    attackers |= tables::bishop_attacks(square, all_pieces)
        & (their_pieces.bishops() | their_pieces.queens());
    attackers |=
        tables::rook_attacks(square, all_pieces) & (their_pieces.rooks() | their_pieces.queens());
    attackers |= tables::king_attacks(square) & their_pieces.king();

    attackers
}

pub fn all_attackers_of(board: &Board, square: Square, occupied: Bitboard) -> Bitboard {
    use Player::*;

    let white_pieces = board.white_pieces();
    let black_pieces = board.black_pieces();

    let mut attackers = Bitboard::EMPTY;

    attackers |= tables::pawn_attacks(square, White) & black_pieces.pawns();
    attackers |= tables::pawn_attacks(square, Black) & white_pieces.pawns();

    let knights = white_pieces.knights() | black_pieces.knights();
    attackers |= tables::knight_attacks(square) & knights;

    let diagonal_attackers = white_pieces.diagonal_sliders() | black_pieces.diagonal_sliders();
    attackers |= tables::bishop_attacks(square, occupied) & diagonal_attackers;

    let orthogonal_attackers =
        white_pieces.orthogonal_sliders() | black_pieces.orthogonal_sliders();
    attackers |= tables::rook_attacks(square, occupied) & orthogonal_attackers;

    let kings = white_pieces.king() | black_pieces.king();
    attackers |= tables::king_attacks(square) & kings;

    attackers
}
