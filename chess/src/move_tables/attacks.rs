use crate::{direction::Direction, player::Player, square::Square, squares::Squares};

pub fn generate_pawn_attacks(square: Square, player: Player) -> Squares {
    let mut attacks = Squares::NONE;

    let pawn_move_direction = match player {
        Player::White => Direction::North,
        Player::Black => Direction::South,
    };

    let forward_one = square.in_direction(pawn_move_direction);

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

pub fn generate_knight_attacks(square: Square) -> Squares {
    let mut attacks = Squares::NONE;

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

pub fn generate_bishop_attacks(square: Square, pieces: Squares) -> Squares {
    generate_sliding_attacks(square, Direction::DIAGONAL, pieces)
}

pub fn generate_rook_attacks(square: Square, pieces: Squares) -> Squares {
    generate_sliding_attacks(square, Direction::CARDINAL, pieces)
}

fn generate_sliding_attacks(square: Square, directions: &[Direction], pieces: Squares) -> Squares {
    let mut attacks = Squares::NONE;

    for direction in directions {
        let mut current_square = square;

        // Until we're off the board
        while let Some(dst) = current_square.in_direction(*direction) {
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

pub fn generate_king_attacks(square: Square) -> Squares {
    let mut attacks = Squares::NONE;

    for direction in Direction::ALL {
        if let Some(dst) = square.in_direction(*direction) {
            attacks |= dst;
        }
    }

    attacks
}
