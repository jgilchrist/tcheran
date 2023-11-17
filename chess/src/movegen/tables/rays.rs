use crate::bitboard::{bitboards, Bitboard};
use crate::direction::Direction;
use crate::square::{File, Rank, Square};

static mut RAYS_TABLE: [[Option<Bitboard>; Square::N]; Square::N] = [[None; Square::N]; Square::N];

pub fn ray(s1: Square, s2: Square) -> Bitboard {
    unsafe { RAYS_TABLE[s1.array_idx()][s2.array_idx()].unwrap() }
}

// TODO: Tidy this up with more utility methods in Rank and File
fn generate_ray_from(s1: Square, s2: Square) -> Option<Bitboard> {
    // Same rank
    if s1.rank() == s2.rank() {
        let shared_rank = s1.rank();

        return Some(match shared_rank {
            Rank::R1 => bitboards::RANK_1,
            Rank::R2 => bitboards::RANK_2,
            Rank::R3 => bitboards::RANK_3,
            Rank::R4 => bitboards::RANK_4,
            Rank::R5 => bitboards::RANK_5,
            Rank::R6 => bitboards::RANK_6,
            Rank::R7 => bitboards::RANK_7,
            Rank::R8 => bitboards::RANK_8,
        });
    }

    // Same file
    if s1.file() == s2.file() {
        let shared_file = s1.file();

        return Some(match shared_file {
            File::A => bitboards::A_FILE,
            File::B => bitboards::B_FILE,
            File::C => bitboards::C_FILE,
            File::D => bitboards::D_FILE,
            File::E => bitboards::E_FILE,
            File::F => bitboards::F_FILE,
            File::G => bitboards::G_FILE,
            File::H => bitboards::H_FILE,
        });
    }

    // Diagonal
    // TODO
    if s1.file().idx().abs_diff(s2.file().idx()) == s1.rank().idx().abs_diff(s2.rank().idx()) {
        let mut squares = Bitboard::EMPTY;

        let leftmost_square = std::cmp::min_by_key(s1, s2, |s| s.file());
        let rightmost_square = std::cmp::max_by_key(s1, s2, |s| s.file());

        // We're starting with the leftmost of the two squares.
        // If that square is below our end square, we need to move up and to the right.
        // If that square is above our end square, we need to move below and to the right.
        let (to_end_direction, to_start_direction) =
            if leftmost_square.rank() < rightmost_square.rank() {
                (Direction::NorthEast, Direction::SouthWest)
            } else {
                (Direction::SouthEast, Direction::NorthWest)
            };

        let mut start_square = leftmost_square;
        let mut end_square = rightmost_square;

        // First, walk back from the start square along the diagonal until we hit the edge of the board
        while !start_square.on_edge() {
            start_square = start_square.in_direction(to_start_direction);
        }

        while !end_square.on_edge() {
            end_square = end_square.in_direction(to_end_direction);
        }

        let mut current_square = start_square;

        while current_square != end_square {
            squares.set_inplace(current_square);
            current_square = current_square.in_direction(to_end_direction);
        }

        squares.set_inplace(end_square);

        return Some(squares);
    }

    // No path between these two squares
    None
}

pub fn init() {
    for s1 in Bitboard::FULL {
        for s2 in Bitboard::FULL {
            let line = generate_ray_from(s1, s2);

            unsafe {
                RAYS_TABLE[s1.array_idx()][s2.array_idx()] = line;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::squares::all::*;

    #[test]
    fn test_ray_on_rank() {
        init();
        assert_eq!(ray(B4, G4), bitboards::RANK_4);
    }

    #[test]
    fn test_ray_on_rank_for_full_rank() {
        init();
        assert_eq!(ray(A1, H1), bitboards::RANK_1);
    }

    #[test]
    fn test_ray_on_file() {
        init();
        assert_eq!(ray(C2, C7), bitboards::C_FILE);
    }

    #[test]
    fn test_ray_on_file_for_full_file() {
        init();
        assert_eq!(ray(H1, H8), bitboards::H_FILE);
    }

    #[test]
    fn test_ray_on_upwards_diagonal() {
        init();
        assert_eq!(ray(B2, G7), A1 | B2 | C3 | D4 | E5 | F6 | G7 | H8);
    }

    #[test]
    fn test_ray_on_downwards_diagonal() {
        init();
        assert_eq!(ray(D5, E4), A8 | B7 | C6 | D5 | E4 | F3 | G2 | H1);
    }

    #[test]
    fn test_ray_on_downwards_diagonal_at_side_of_board() {
        init();
        assert_eq!(ray(G8, H7), G8 | H7);
    }

    #[test]
    fn test_ray_on_full_diagonal() {
        init();
        assert_eq!(ray(A1, H8), A1 | B2 | C3 | D4 | E5 | F6 | G7 | H8);
    }
}
