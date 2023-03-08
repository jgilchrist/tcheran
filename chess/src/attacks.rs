use crate::{board::Board, move_tables, player::Player, squares::Squares};

pub fn generate_all_attacks(board: &Board, player: Player) -> Squares {
    let mut attacks = Squares::none();

    let our_pieces = board.player_pieces(player);
    let their_pieces = board.player_pieces(player.other()).all();
    let all_pieces = our_pieces.all() | their_pieces;

    for pawn in our_pieces.pawns {
        attacks |= move_tables::pawn_attacks(pawn, player);
    }

    for knight in our_pieces.knights {
        attacks |= move_tables::knight_attacks(knight);
    }

    for bishop in our_pieces.bishops {
        attacks |= move_tables::bishop_attacks(bishop, all_pieces);
    }

    for rook in our_pieces.rooks {
        attacks |= move_tables::rook_attacks(rook, all_pieces);
    }

    for queen in our_pieces.queens {
        attacks |= move_tables::queen_attacks(queen, all_pieces);
    }

    for king in our_pieces.king {
        attacks |= move_tables::king_attacks(king);
    }

    attacks
}
