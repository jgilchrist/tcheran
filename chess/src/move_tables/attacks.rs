use crate::{direction::Direction, square::Square, squares::Squares};

pub fn generate_bishop_attacks(square: Square, pieces: Squares) -> Squares {
    generate_sliding_attacks(square, Direction::DIAGONAL, pieces)
}

pub fn generate_rook_attacks(square: Square, pieces: Squares) -> Squares {
    generate_sliding_attacks(square, Direction::CARDINAL, pieces)
}

fn generate_sliding_attacks(square: Square, directions: &[Direction], pieces: Squares) -> Squares {
    let mut attacks = Squares::none();

    for direction in directions {
        let mut current_square = square;

        // Until we're off the board
        while let Some(dst) = current_square.in_direction(direction) {
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
