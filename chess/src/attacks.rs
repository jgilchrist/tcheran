use crate::{
    board::Board, move_tables, piece::PieceKind, player::Player, square::Square, squares::Squares,
};

// PERF: There are less naive approaches which would improve performance here

pub fn generate_all_attacks(board: &Board, player: Player) -> Squares {
    let mut attacks = Squares::none();

    let our_pieces = board.player_pieces(player).all();
    let their_pieces = board.player_pieces(player.other()).all();
    let all_pieces = our_pieces | their_pieces;

    for square in our_pieces {
        let piece_kind = board
            .player_piece_at(player, square)
            .expect("Unable to find piece on square it should be on.");
        attacks |= generate_piece_attacks(piece_kind, player, square, all_pieces);
    }

    attacks
}

fn generate_piece_attacks(
    piece_kind: PieceKind,
    player: Player,
    square: Square,
    pieces: Squares,
) -> Squares {
    match piece_kind {
        PieceKind::Pawn => move_tables::pawn_attacks(square, player),
        PieceKind::Knight => move_tables::knight_attacks(square),
        PieceKind::Bishop => move_tables::bishop_attacks(square, pieces),
        PieceKind::Rook => move_tables::rook_attacks(square, pieces),
        PieceKind::Queen => move_tables::queen_attacks(square, pieces),
        PieceKind::King => move_tables::king_attacks(square),
    }
}
