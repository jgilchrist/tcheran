use crate::chess::bitboard::Bitboard;
use crate::chess::{direction::Direction, player::Player, square::Square};

pub fn generate_pawn_attacks(square: Square, player: Player) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;
    let sq = square.bb();

    attacks |= sq.forward(player).west();
    attacks |= sq.forward(player).east();

    attacks
}

pub fn generate_knight_attacks(square: Square) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;
    let sq = square.bb();

    // Going clockwise, starting at 12
    attacks |= sq.north().north_east();
    attacks |= sq.east().north_east();
    attacks |= sq.east().south_east();
    attacks |= sq.south().south_east();
    attacks |= sq.south().south_west();
    attacks |= sq.west().south_west();
    attacks |= sq.west().north_west();
    attacks |= sq.north().north_west();

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
        let mut current_square = square.bb();

        // Until we're off the board
        while current_square.any() {
            current_square = current_square.in_direction(*direction);
            attacks |= current_square;

            // Future squares blocked
            if (pieces & current_square).any() {
                break;
            }
        }
    }

    attacks
}

pub fn generate_king_attacks(square: Square) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;
    let sq = square.bb();

    for direction in Direction::ALL {
        attacks |= sq.in_direction(*direction);
    }

    attacks
}
