use crate::bitboard::Bitboard;
use crate::board::Board;
use crate::movegen::tables;
use crate::player::Player;
use crate::square::Square;

pub fn generate_attackers_of(board: &Board, player: Player, square: Square) -> Bitboard {
    let mut attackers = Bitboard::EMPTY;
    let our_pieces = board.player_pieces(player);
    let their_pieces = board.player_pieces(player.other());
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
