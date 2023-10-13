use crate::{
    direction::Direction,
    square::{File, Rank, Square},
    squares::Squares,
};

pub fn generate_bishop_occupancies(square: Square) -> Squares {
    generate_sliding_occupancies(square, Direction::DIAGONAL)
}

pub fn generate_rook_occupancies(square: Square) -> Squares {
    generate_sliding_occupancies(square, Direction::CARDINAL)
}

pub fn generate_sliding_occupancies(square: Square, directions: &[Direction]) -> Squares {
    let mut squares = Squares::none();

    for direction in directions {
        let mut current_square = square;

        while let Some(dst) = current_square.in_direction(direction) {
            // Until we hit one of the edges
            let (src_rank, src_file) = (square.rank(), square.file());
            let (dst_rank, dst_file) = (dst.rank(), dst.file());

            if dst_rank == Rank::R1 && src_rank != Rank::R1
                || dst_rank == Rank::R8 && src_rank != Rank::R8
                || dst_file == File::A && src_file != File::A
                || dst_file == File::H && src_file != File::H
            {
                break;
            }

            current_square = dst;
            squares |= dst;
        }
    }

    squares
}
