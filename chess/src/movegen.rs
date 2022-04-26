use crate::{
    bitboard::{self, Bitboard},
    direction::Direction,
    game::Game,
    moves::Move,
    piece::PromotionPieceKind,
    player::Player,
    square::{self, Square},
};

struct Ctx {
    our_pieces: Bitboard,
    their_pieces: Bitboard,
    all_pieces: Bitboard,
}

pub fn generate_moves(game: &Game) -> Vec<Move> {
    let ctx = get_ctx(game);

    let mut moves: Vec<Move> = vec![];
    moves.extend(generate_pawn_moves(game, &ctx));
    moves.extend(generate_knight_moves(game, &ctx));
    moves.extend(generate_bishop_moves(game, &ctx));
    moves.extend(generate_rook_moves(game, &ctx));
    moves.extend(generate_queen_moves(game, &ctx));
    moves.extend(generate_king_moves(game, &ctx));
    moves
}

fn get_ctx(game: &Game) -> Ctx {
    let our_pieces = game.board.player_pieces(&game.player).all();
    let their_pieces = game.board.player_pieces(&game.player.other()).all();
    let all_pieces = our_pieces | their_pieces;

    Ctx {
        our_pieces,
        their_pieces,
        all_pieces,
    }
}

fn generate_pawn_moves(game: &Game, ctx: &Ctx) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    let pawns = game.board.player_pieces(&game.player).pawns;

    let pawn_move_direction = match game.player {
        Player::White => Direction::North,
        Player::Black => Direction::South,
    };

    let back_rank = match game.player {
        Player::White => bitboard::known::RANK_2,
        Player::Black => bitboard::known::RANK_7,
    };

    let will_promote_rank = match game.player {
        Player::White => bitboard::known::RANK_7,
        Player::Black => bitboard::known::RANK_2,
    };

    for start in pawns.squares() {
        let will_promote = !((Bitboard::from_square(&start) & will_promote_rank).is_empty());

        // Move forward by 1
        let forward_one = start.in_direction(&pawn_move_direction);

        if let Some(dst) = forward_one {
            if !ctx.all_pieces.has_square(&dst) {
                match will_promote {
                    false => moves.push(Move::new(start, dst)),
                    true => {
                        for promotion in PromotionPieceKind::ALL.iter() {
                            moves.push(Move::new_with_promotion(start, dst, *promotion))
                        }
                    }
                }
            }
        }

        // Capture
        let capture_left = forward_one.and_then(|s| s.west());

        if let Some(dst) = capture_left {
            if ctx.their_pieces.has_square(&dst) || game.en_passant_target == Some(dst) {
                match will_promote {
                    false => moves.push(Move::new(start, dst)),
                    true => {
                        for promotion in PromotionPieceKind::ALL.iter() {
                            moves.push(Move::new_with_promotion(start, dst, *promotion))
                        }
                    }
                }
            }
        }

        let capture_right = forward_one.and_then(|s| s.east());

        if let Some(dst) = capture_right {
            if ctx.their_pieces.has_square(&dst) || game.en_passant_target == Some(dst) {
                match will_promote {
                    false => moves.push(Move::new(start, dst)),
                    true => {
                        for promotion in PromotionPieceKind::ALL.iter() {
                            moves.push(Move::new_with_promotion(start, dst, *promotion))
                        }
                    }
                }
            }
        }
    }

    for start in (pawns & back_rank).squares() {
        // Move forward by 2
        let forward_one = start.in_direction(&pawn_move_direction);

        if let Some(forward_one) = forward_one {
            if ctx.all_pieces.has_square(&forward_one) {
                // Cannot jump over pieces
                continue;
            }

            let forward_two = forward_one.in_direction(&pawn_move_direction);

            if let Some(forward_two) = forward_two {
                if !ctx.all_pieces.has_square(&forward_two) {
                    moves.push(Move::new(start, forward_two));
                }
            }
        }
    }

    moves
}

fn generate_knight_moves(game: &Game, ctx: &Ctx) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    let knights = game.board.player_pieces(&game.player).knights;

    for start in knights.squares() {
        // Going clockwise, starting at 12
        let nne = Some(start)
            .and_then(|s| s.north())
            .and_then(|s| s.north())
            .and_then(|s| s.east());

        let een = Some(start)
            .and_then(|s| s.east())
            .and_then(|s| s.east())
            .and_then(|s| s.north());

        let ees = Some(start)
            .and_then(|s| s.east())
            .and_then(|s| s.east())
            .and_then(|s| s.south());

        let sse = Some(start)
            .and_then(|s| s.south())
            .and_then(|s| s.south())
            .and_then(|s| s.east());

        let ssw = Some(start)
            .and_then(|s| s.south())
            .and_then(|s| s.south())
            .and_then(|s| s.west());

        let wws = Some(start)
            .and_then(|s| s.west())
            .and_then(|s| s.west())
            .and_then(|s| s.south());

        let wwn = Some(start)
            .and_then(|s| s.west())
            .and_then(|s| s.west())
            .and_then(|s| s.north());

        let nnw = Some(start)
            .and_then(|s| s.north())
            .and_then(|s| s.north())
            .and_then(|s| s.west());

        let maybe_destination_squares = vec![nne, een, ees, sse, ssw, wws, wwn, nnw];

        let allowed_moves: Vec<Move> = maybe_destination_squares
            .into_iter()
            .flatten()
            .filter(|s| !ctx.our_pieces.has_square(s))
            .map(|s| Move::new(start, s))
            .collect();

        moves.extend(allowed_moves);
    }

    moves
}

fn generate_bishop_moves(game: &Game, ctx: &Ctx) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    let bishops = game.board.player_pieces(&game.player).bishops;

    for bishop in bishops.squares() {
        for direction in Direction::DIAGONAL {
            let mut current_square = bishop;

            // Until we're off the board
            while let Some(dst) = current_square.in_direction(direction) {
                current_square = dst;

                // Blocked by our piece
                if ctx.our_pieces.has_square(&dst) {
                    break;
                }

                // Capture a piece, but squares past here are off-limits
                if ctx.their_pieces.has_square(&dst) {
                    moves.push(Move::new(bishop, dst));
                    break;
                }

                moves.push(Move::new(bishop, dst));
            }
        }
    }

    moves
}

fn generate_rook_moves(game: &Game, ctx: &Ctx) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    let rooks = game.board.player_pieces(&game.player).rooks;

    for rook in rooks.squares() {
        for direction in Direction::NON_DIAGONAL {
            let mut current_square = rook;

            // Until we're off the board
            while let Some(dst) = current_square.in_direction(direction) {
                current_square = dst;

                // Blocked by our piece
                if ctx.our_pieces.has_square(&dst) {
                    break;
                }

                // Capture a piece, but squares past here are off-limits
                if ctx.their_pieces.has_square(&dst) {
                    moves.push(Move::new(rook, dst));
                    break;
                }

                moves.push(Move::new(rook, dst));
            }
        }
    }

    moves
}

fn generate_queen_moves(game: &Game, ctx: &Ctx) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    let queens = game.board.player_pieces(&game.player).queen;

    for queen in queens.squares() {
        for direction in Direction::ALL {
            let mut current_square = queen;

            // Until we're off the board
            while let Some(dst) = current_square.in_direction(direction) {
                current_square = dst;

                // Blocked by our piece
                if ctx.our_pieces.has_square(&dst) {
                    break;
                }

                // Capture a piece, but squares past here are off-limits
                if ctx.their_pieces.has_square(&dst) {
                    moves.push(Move::new(queen, dst));
                    break;
                }

                moves.push(Move::new(queen, dst));
            }
        }
    }

    moves
}

fn generate_king_moves(game: &Game, ctx: &Ctx) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    let king = game
        .board
        .player_pieces(&game.player)
        .king
        .to_square_definite();

    for direction in Direction::ALL {
        if let Some(dst) = king.in_direction(direction) {
            if ctx.our_pieces.has_square(&dst) {
                continue;
            }

            moves.push(Move::new(king, dst));
        }
    }

    let castle_rights_for_player = match game.player {
        Player::White => game.white_castle_rights,
        Player::Black => game.black_castle_rights,
    };

    let king_start_square = match game.player {
        Player::White => square::known::WHITE_KING_START,
        Player::Black => square::known::BLACK_KING_START,
    };

    if king == king_start_square && castle_rights_for_player.can_castle() {
        if castle_rights_for_player.king_side {
            let kingside_required_empty_squares = match game.player {
                Player::White => vec![Square::F1, Square::G1],
                Player::Black => vec![Square::F8, Square::G8],
            };

            let path_to_castle_is_empty = kingside_required_empty_squares
                .iter()
                .all(|s| !ctx.all_pieces.has_square(s));

            if path_to_castle_is_empty {
                let kingside_dst_square = match game.player {
                    Player::White => square::known::WHITE_KINGSIDE_CASTLE,
                    Player::Black => square::known::BLACK_KINGSIDE_CASTLE,
                };

                moves.push(Move::new(king, kingside_dst_square));
            }
        }

        if castle_rights_for_player.queen_side {
            let queenside_required_empty_squares = match game.player {
                Player::White => vec![Square::B1, Square::C1, Square::D1],
                Player::Black => vec![Square::B8, Square::C8, Square::D8],
            };

            let path_to_castle_is_empty = queenside_required_empty_squares
                .iter()
                .all(|s| !ctx.all_pieces.has_square(s));

            if path_to_castle_is_empty {
                let queenside_dst_square = match game.player {
                    Player::White => square::known::WHITE_QUEENSIDE_CASTLE,
                    Player::Black => square::known::BLACK_QUEENSIDE_CASTLE,
                };

                moves.push(Move::new(king, queenside_dst_square));
            }
        }
    }

    moves
}
