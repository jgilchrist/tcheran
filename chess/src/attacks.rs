use crate::{
    board::Board, direction::Direction, move_tables, piece::PieceKind, player::Player,
    square::Square, squares::Squares,
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
        PieceKind::Pawn => generate_pawn_attacks(square, player),
        PieceKind::Knight => generate_knight_attacks(square),
        PieceKind::Bishop => move_tables::bishop_attacks(square, pieces),
        PieceKind::Rook => move_tables::rook_attacks(square, pieces),
        PieceKind::Queen => move_tables::queen_attacks(square, pieces),
        PieceKind::King => generate_king_attacks(square),
    }
}

pub fn generate_pawn_attacks(square: Square, player: Player) -> Squares {
    let mut attacks = Squares::none();

    let pawn_move_direction = match player {
        Player::White => Direction::North,
        Player::Black => Direction::South,
    };

    let forward_one = square.in_direction(&pawn_move_direction);

    // Capture
    let capture_left = forward_one.and_then(|s| s.west());

    if let Some(dst) = capture_left {
        attacks |= dst;
    }

    let capture_right = forward_one.and_then(|s| s.east());

    if let Some(dst) = capture_right {
        attacks |= dst;
    }

    attacks
}

fn generate_knight_attacks(square: Square) -> Squares {
    let mut attacks = Squares::none();

    // Going clockwise, starting at 12
    if let Some(nne) = square.north().and_then(|s| s.north_east()) {
        attacks |= nne;
    }

    if let Some(een) = square.east().and_then(|s| s.north_east()) {
        attacks |= een;
    }

    if let Some(ees) = square.east().and_then(|s| s.south_east()) {
        attacks |= ees;
    }

    if let Some(sse) = square.south().and_then(|s| s.south_east()) {
        attacks |= sse;
    }

    if let Some(ssw) = square.south().and_then(|s| s.south_west()) {
        attacks |= ssw;
    }

    if let Some(wws) = square.west().and_then(|s| s.south_west()) {
        attacks |= wws;
    }

    if let Some(wwn) = square.west().and_then(|s| s.north_west()) {
        attacks |= wwn;
    }

    if let Some(nnw) = square.north().and_then(|s| s.north_west()) {
        attacks |= nnw;
    }

    attacks
}

fn generate_king_attacks(square: Square) -> Squares {
    let mut attacks = Squares::none();

    for direction in Direction::ALL {
        if let Some(dst) = square.in_direction(direction) {
            attacks |= dst;
        }
    }

    attacks
}
