use crate::bitboard::Bitboard;
use crate::{direction::Direction, player::Player, square::Square};

pub fn generate_pawn_attacks(square: Square, player: Player) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

    let pawn_move_direction = match player {
        Player::White => Direction::North,
        Player::Black => Direction::South,
    };

    let forward_one = square.in_direction_maybe(pawn_move_direction);

    // Capture
    let capture_left = forward_one.and_then(|s| s.west_maybe());

    if let Some(dst) = capture_left {
        attacks |= dst;
    }

    let capture_right = forward_one.and_then(|s| s.east_maybe());

    if let Some(dst) = capture_right {
        attacks |= dst;
    }

    attacks
}

pub fn generate_knight_attacks(square: Square) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

    // Going clockwise, starting at 12
    if let Some(nne) = square.north_maybe().and_then(|s| s.north_east_maybe()) {
        attacks |= nne;
    }

    if let Some(een) = square.east_maybe().and_then(|s| s.north_east_maybe()) {
        attacks |= een;
    }

    if let Some(ees) = square.east_maybe().and_then(|s| s.south_east_maybe()) {
        attacks |= ees;
    }

    if let Some(sse) = square.south_maybe().and_then(|s| s.south_east_maybe()) {
        attacks |= sse;
    }

    if let Some(ssw) = square.south_maybe().and_then(|s| s.south_west_maybe()) {
        attacks |= ssw;
    }

    if let Some(wws) = square.west_maybe().and_then(|s| s.south_west_maybe()) {
        attacks |= wws;
    }

    if let Some(wwn) = square.west_maybe().and_then(|s| s.north_west_maybe()) {
        attacks |= wwn;
    }

    if let Some(nnw) = square.north_maybe().and_then(|s| s.north_west_maybe()) {
        attacks |= nnw;
    }

    attacks
}

pub fn generate_bishop_attacks(square: Square, pieces: Bitboard) -> Bitboard {
    generate_sliding_attacks(square, Direction::DIAGONAL, pieces)
}

pub fn generate_rook_attacks(square: Square, pieces: Bitboard) -> Bitboard {
    generate_sliding_attacks(square, Direction::CARDINAL, pieces)
}

fn generate_sliding_attacks(
    square: Square,
    directions: &[Direction],
    pieces: Bitboard,
) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

    for direction in directions {
        let mut current_square = square;

        // Until we're off the board
        while let Some(dst) = current_square.in_direction_maybe(*direction) {
            current_square = dst;
            attacks |= dst;

            // Future squares blocked
            if pieces.contains(dst) {
                break;
            }
        }
    }

    attacks
}

pub fn generate_king_attacks(square: Square) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

    for direction in Direction::ALL {
        if let Some(dst) = square.in_direction_maybe(*direction) {
            attacks |= dst;
        }
    }

    attacks
}
