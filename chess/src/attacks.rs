use crate::{
    bitboard::Bitboard, board::Board, direction::Direction, piece::PieceKind, player::Player,
    square::Square,
};

// PERF: There are less naive approaches which would improve performance here

pub fn generate_all_attacks(board: &Board, player: &Player) -> Bitboard {
    let mut attacks = Bitboard::empty();

    let our_pieces = board.player_pieces(player).all();
    let their_pieces = board.player_pieces(&player.other()).all();
    let all_pieces = our_pieces | their_pieces;

    for square in our_pieces.squares() {
        let piece_kind = board
            .player_piece_at(player, &square)
            .expect("Piece bitboard had a piece on this square");
        attacks |= generate_piece_attacks(piece_kind, player, &square, &all_pieces);
    }

    attacks
}

fn generate_piece_attacks(
    piece_kind: PieceKind,
    player: &Player,
    square: &Square,
    pieces: &Bitboard,
) -> Bitboard {
    match piece_kind {
        PieceKind::Pawn => generate_pawn_attacks(square, player),
        PieceKind::Knight => generate_knight_attacks(square),
        PieceKind::Bishop => generate_bishop_attacks(square, pieces),
        PieceKind::Rook => generate_rook_attacks(square, pieces),
        PieceKind::Queen => generate_queen_attacks(square, pieces),
        PieceKind::King => generate_king_attacks(square),
    }
}

fn generate_pawn_attacks(square: &Square, player: &Player) -> Bitboard {
    let mut attacks = Bitboard::empty();

    let pawn_move_direction = match player {
        Player::White => Direction::North,
        Player::Black => Direction::South,
    };

    let forward_one = square.in_direction(&pawn_move_direction);

    // Capture
    let capture_left = forward_one.and_then(|s| s.west());

    if let Some(dst) = capture_left {
        attacks |= dst.bitboard();
    }

    let capture_right = forward_one.and_then(|s| s.east());

    if let Some(dst) = capture_right {
        attacks |= dst.bitboard();
    }

    attacks
}

fn generate_knight_attacks(square: &Square) -> Bitboard {
    let mut attacks = Bitboard::empty();

    // Going clockwise, starting at 12
    if let Some(nne) = square.north().and_then(|s| s.north_east()) {
        attacks |= nne.bitboard();
    }

    if let Some(een) = square.east().and_then(|s| s.north_east()) {
        attacks |= een.bitboard();
    }

    if let Some(ees) = square.east().and_then(|s| s.south_east()) {
        attacks |= ees.bitboard();
    }

    if let Some(sse) = square.south().and_then(|s| s.south_east()) {
        attacks |= sse.bitboard();
    }

    if let Some(ssw) = square.south().and_then(|s| s.south_west()) {
        attacks |= ssw.bitboard();
    }

    if let Some(wws) = square.west().and_then(|s| s.south_west()) {
        attacks |= wws.bitboard();
    }

    if let Some(wwn) = square.west().and_then(|s| s.north_west()) {
        attacks |= wwn.bitboard();
    }

    if let Some(nnw) = square.north().and_then(|s| s.north_west()) {
        attacks |= nnw.bitboard();
    }

    attacks
}

fn generate_bishop_attacks(square: &Square, pieces: &Bitboard) -> Bitboard {
    generate_sliding_attacks(square, Direction::DIAGONAL, pieces)
}

fn generate_rook_attacks(square: &Square, pieces: &Bitboard) -> Bitboard {
    generate_sliding_attacks(square, Direction::NON_DIAGONAL, pieces)
}

fn generate_queen_attacks(square: &Square, pieces: &Bitboard) -> Bitboard {
    generate_sliding_attacks(square, Direction::ALL, pieces)
}

fn generate_sliding_attacks(
    square: &Square,
    directions: &[Direction],
    pieces: &Bitboard,
) -> Bitboard {
    let mut attacks = Bitboard::empty();

    for direction in directions {
        let mut current_square = *square;

        // Until we're off the board
        while let Some(dst) = current_square.in_direction(direction) {
            current_square = dst;
            attacks |= dst.bitboard();

            // Future squares blocked
            if pieces.has_square(&dst) {
                break;
            }
        }
    }

    attacks
}

fn generate_king_attacks(square: &Square) -> Bitboard {
    let mut attacks = Bitboard::empty();

    for direction in Direction::ALL {
        if let Some(dst) = square.in_direction(direction) {
            attacks |= dst.bitboard();
        }
    }

    attacks
}
