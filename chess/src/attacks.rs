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
        attacks |= Bitboard::from_square(&dst)
    }

    let capture_right = forward_one.and_then(|s| s.east());

    if let Some(dst) = capture_right {
        attacks |= Bitboard::from_square(&dst)
    }

    attacks
}

fn generate_knight_attacks(square: &Square) -> Bitboard {
    let mut attacks = Bitboard::empty();

    // Going clockwise, starting at 12
    let nne = Some(square)
        .and_then(|s| s.north())
        .and_then(|s| s.north())
        .and_then(|s| s.east());

    let een = Some(square)
        .and_then(|s| s.east())
        .and_then(|s| s.east())
        .and_then(|s| s.north());

    let ees = Some(square)
        .and_then(|s| s.east())
        .and_then(|s| s.east())
        .and_then(|s| s.south());

    let sse = Some(square)
        .and_then(|s| s.south())
        .and_then(|s| s.south())
        .and_then(|s| s.east());

    let ssw = Some(square)
        .and_then(|s| s.south())
        .and_then(|s| s.south())
        .and_then(|s| s.west());

    let wws = Some(square)
        .and_then(|s| s.west())
        .and_then(|s| s.west())
        .and_then(|s| s.south());

    let wwn = Some(square)
        .and_then(|s| s.west())
        .and_then(|s| s.west())
        .and_then(|s| s.north());

    let nnw = Some(square)
        .and_then(|s| s.north())
        .and_then(|s| s.north())
        .and_then(|s| s.west());

    if let Some(s) = nne { attacks |= Bitboard::from_square(&s); }
    if let Some(s) = een { attacks |= Bitboard::from_square(&s); }
    if let Some(s) = ees { attacks |= Bitboard::from_square(&s); }
    if let Some(s) = sse { attacks |= Bitboard::from_square(&s); }
    if let Some(s) = ssw { attacks |= Bitboard::from_square(&s); }
    if let Some(s) = wws { attacks |= Bitboard::from_square(&s); }
    if let Some(s) = wwn { attacks |= Bitboard::from_square(&s); }
    if let Some(s) = nnw { attacks |= Bitboard::from_square(&s); }

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
            attacks |= Bitboard::from_square(&dst);

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
            attacks |= Bitboard::from_square(&dst)
        }
    }

    attacks
}
