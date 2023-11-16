use crate::bitboard::Bitboard;
use crate::board::Board;
use crate::movegen::tables;
use crate::player::Player;
use crate::square::Square;

#[must_use]
pub fn generate_all_attacks(board: &Board, player: Player) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

    let our_pieces = board.player_pieces(player);
    let their_pieces = board.player_pieces(player.other()).all();
    let all_pieces = our_pieces.all() | their_pieces;

    for pawn in our_pieces.pawns {
        attacks |= tables::pawn_attacks(pawn, player);
    }

    for knight in our_pieces.knights {
        attacks |= tables::knight_attacks(knight);
    }

    for bishop in our_pieces.bishops {
        attacks |= tables::bishop_attacks(bishop, all_pieces);
    }

    for rook in our_pieces.rooks {
        attacks |= tables::rook_attacks(rook, all_pieces);
    }

    for queen in our_pieces.queens {
        attacks |= tables::queen_attacks(queen, all_pieces);
    }

    for king in our_pieces.king {
        attacks |= tables::king_attacks(king);
    }

    attacks
}

#[must_use]
pub fn generate_attackers_of(board: &Board, player: Player, square: Square) -> Bitboard {
    let mut attackers = Bitboard::EMPTY;
    let our_pieces = board.player_pieces(player);
    let their_pieces = board.player_pieces(player.other());
    let all_pieces = our_pieces.all() | their_pieces.all();

    // Pawns: A square is attacked by pawns in the same positions as a pawn could capture if it was on
    // that square
    attackers |= tables::pawn_attacks(square, player) & their_pieces.pawns;

    // Knights: A square is attacked by any squares a knight could reach if it were on that square
    attackers |= tables::knight_attacks(square) & their_pieces.knights;

    // Sliders: A square is attacked by any squares a
    attackers |= tables::bishop_attacks(square, all_pieces) & their_pieces.bishops;
    attackers |= tables::rook_attacks(square, all_pieces) & their_pieces.rooks;
    attackers |= tables::queen_attacks(square, all_pieces) & their_pieces.queens;
    attackers |= tables::king_attacks(square) & their_pieces.king;

    attackers
}
