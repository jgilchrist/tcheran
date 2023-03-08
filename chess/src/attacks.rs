use crate::{
    board::Board,
    direction::Direction,
    piece::PieceKind,
    player::Player,
    square::{File, Rank, Square},
    squares::Squares,
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
        PieceKind::Bishop => generate_bishop_attacks(square, pieces),
        PieceKind::Rook => generate_rook_attacks(square, pieces),
        PieceKind::Queen => {
            generate_rook_attacks(square, pieces) | generate_bishop_attacks(square, pieces)
        }
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

pub fn generate_bishop_attacks(square: Square, pieces: Squares) -> Squares {
    generate_sliding_attacks(square, Direction::DIAGONAL, pieces)
}

pub fn generate_bishop_occupancies(square: Square) -> Squares {
    generate_sliding_occupancies(square, Direction::DIAGONAL)
}

pub fn generate_rook_attacks(square: Square, pieces: Squares) -> Squares {
    generate_sliding_attacks(square, Direction::CARDINAL, pieces)
}

pub fn generate_rook_occupancies(square: Square) -> Squares {
    generate_sliding_occupancies(square, Direction::CARDINAL)
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

fn generate_sliding_occupancies(square: Square, directions: &[Direction]) -> Squares {
    let mut squares = Squares::none();

    for direction in directions {
        let mut current_square = square;

        while let Some(dst) = current_square.in_direction(direction) {
            // Until we hit one of the edges
            // TODO: Optimise edge checking using an edges bitboard
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

fn generate_king_attacks(square: Square) -> Squares {
    let mut attacks = Squares::none();

    for direction in Direction::ALL {
        if let Some(dst) = square.in_direction(direction) {
            attacks |= dst;
        }
    }

    attacks
}
