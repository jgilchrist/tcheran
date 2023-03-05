use crate::{
    direction::Direction,
    game::Game,
    moves::Move,
    piece::PromotionPieceKind,
    player::Player,
    squares::{self, all::*, Squares},
};

struct Ctx {
    our_pieces: Squares,
    their_pieces: Squares,
    all_pieces: Squares,
}

pub fn generate_moves(game: &Game) -> Vec<Move> {
    let ctx = get_ctx(game);

    let mut moves: Vec<Move> = vec![];
    generate_pawn_moves(&mut moves, game, &ctx);
    generate_knight_moves(&mut moves, game, &ctx);
    generate_bishop_moves(&mut moves, game, &ctx);
    generate_rook_moves(&mut moves, game, &ctx);
    generate_queen_moves(&mut moves, game, &ctx);
    generate_king_moves(&mut moves, game, &ctx);
    moves
}

fn get_ctx(game: &Game) -> Ctx {
    let our_pieces = game.board.player_pieces(game.player).all();
    let their_pieces = game.board.player_pieces(game.player.other()).all();
    let all_pieces = our_pieces | their_pieces;

    Ctx {
        our_pieces,
        their_pieces,
        all_pieces,
    }
}

fn generate_pawn_moves(moves: &mut Vec<Move>, game: &Game, ctx: &Ctx) {
    let pawns = game.board.player_pieces(game.player).pawns;

    let pawn_move_direction = match game.player {
        Player::White => Direction::North,
        Player::Black => Direction::South,
    };

    let back_rank = match game.player {
        Player::White => squares::RANK_2,
        Player::Black => squares::RANK_7,
    };

    let will_promote_rank = match game.player {
        Player::White => squares::RANK_7,
        Player::Black => squares::RANK_2,
    };

    for start in pawns {
        let will_promote = !((will_promote_rank & start).is_empty());

        // Move forward by 1
        let forward_one = start.in_direction(&pawn_move_direction);

        if let Some(dst) = forward_one {
            if !ctx.all_pieces.contains(dst) {
                if will_promote {
                    for promotion in PromotionPieceKind::ALL {
                        moves.push(Move::new_with_promotion(start, dst, *promotion));
                    }
                } else {
                    moves.push(Move::new(start, dst));
                }
            }
        }

        // Capture
        let capture_left = forward_one.and_then(|s| s.west());

        if let Some(dst) = capture_left {
            if ctx.their_pieces.contains(dst) || game.en_passant_target == Some(dst) {
                if will_promote {
                    for promotion in PromotionPieceKind::ALL {
                        moves.push(Move::new_with_promotion(start, dst, *promotion));
                    }
                } else {
                    moves.push(Move::new(start, dst));
                }
            }
        }

        let capture_right = forward_one.and_then(|s| s.east());

        if let Some(dst) = capture_right {
            if ctx.their_pieces.contains(dst) || game.en_passant_target == Some(dst) {
                if will_promote {
                    for promotion in PromotionPieceKind::ALL {
                        moves.push(Move::new_with_promotion(start, dst, *promotion));
                    }
                } else {
                    moves.push(Move::new(start, dst));
                }
            }
        }
    }

    for start in pawns & back_rank {
        // Move forward by 2
        let forward_one = start.in_direction(&pawn_move_direction);

        if let Some(forward_one) = forward_one {
            if ctx.all_pieces.contains(forward_one) {
                // Cannot jump over pieces
                continue;
            }

            let forward_two = forward_one.in_direction(&pawn_move_direction);

            if let Some(forward_two) = forward_two {
                if !ctx.all_pieces.contains(forward_two) {
                    moves.push(Move::new(start, forward_two));
                }
            }
        }
    }
}

fn generate_knight_moves(moves: &mut Vec<Move>, game: &Game, ctx: &Ctx) {
    let knights = game.board.player_pieces(game.player).knights;

    for start in knights {
        // Going clockwise, starting at 12
        if let Some(nne) = start.north().and_then(|s| s.north_east()) {
            if !ctx.our_pieces.contains(nne) {
                moves.push(Move::new(start, nne));
            }
        }

        if let Some(een) = start.east().and_then(|s| s.north_east()) {
            if !ctx.our_pieces.contains(een) {
                moves.push(Move::new(start, een));
            }
        }

        if let Some(ees) = start.east().and_then(|s| s.south_east()) {
            if !ctx.our_pieces.contains(ees) {
                moves.push(Move::new(start, ees));
            }
        }

        if let Some(sse) = start.south().and_then(|s| s.south_east()) {
            if !ctx.our_pieces.contains(sse) {
                moves.push(Move::new(start, sse));
            }
        }

        if let Some(ssw) = start.south().and_then(|s| s.south_west()) {
            if !ctx.our_pieces.contains(ssw) {
                moves.push(Move::new(start, ssw));
            }
        }

        if let Some(wws) = start.west().and_then(|s| s.south_west()) {
            if !ctx.our_pieces.contains(wws) {
                moves.push(Move::new(start, wws));
            }
        }

        if let Some(wwn) = start.west().and_then(|s| s.north_west()) {
            if !ctx.our_pieces.contains(wwn) {
                moves.push(Move::new(start, wwn));
            }
        }

        if let Some(nnw) = start.north().and_then(|s| s.north_west()) {
            if !ctx.our_pieces.contains(nnw) {
                moves.push(Move::new(start, nnw));
            }
        }
    }
}

fn generate_bishop_moves(moves: &mut Vec<Move>, game: &Game, ctx: &Ctx) {
    let bishops = game.board.player_pieces(game.player).bishops;

    for bishop in bishops {
        for direction in Direction::DIAGONAL {
            let mut current_square = bishop;

            // Until we're off the board
            while let Some(dst) = current_square.in_direction(direction) {
                current_square = dst;

                // Blocked by our piece
                if ctx.our_pieces.contains(dst) {
                    break;
                }

                // Capture a piece, but squares past here are off-limits
                if ctx.their_pieces.contains(dst) {
                    moves.push(Move::new(bishop, dst));
                    break;
                }

                moves.push(Move::new(bishop, dst));
            }
        }
    }
}

fn generate_rook_moves(moves: &mut Vec<Move>, game: &Game, ctx: &Ctx) {
    let rooks = game.board.player_pieces(game.player).rooks;

    for rook in rooks {
        for direction in Direction::CARDINAL {
            let mut current_square = rook;

            // Until we're off the board
            while let Some(dst) = current_square.in_direction(direction) {
                current_square = dst;

                // Blocked by our piece
                if ctx.our_pieces.contains(dst) {
                    break;
                }

                // Capture a piece, but squares past here are off-limits
                if ctx.their_pieces.contains(dst) {
                    moves.push(Move::new(rook, dst));
                    break;
                }

                moves.push(Move::new(rook, dst));
            }
        }
    }
}

fn generate_queen_moves(moves: &mut Vec<Move>, game: &Game, ctx: &Ctx) {
    let queens = game.board.player_pieces(game.player).queens;

    for queen in queens {
        for direction in Direction::ALL {
            let mut current_square = queen;

            // Until we're off the board
            while let Some(dst) = current_square.in_direction(direction) {
                current_square = dst;

                // Blocked by our piece
                if ctx.our_pieces.contains(dst) {
                    break;
                }

                // Capture a piece, but squares past here are off-limits
                if ctx.their_pieces.contains(dst) {
                    moves.push(Move::new(queen, dst));
                    break;
                }

                moves.push(Move::new(queen, dst));
            }
        }
    }
}

fn generate_king_moves(moves: &mut Vec<Move>, game: &Game, ctx: &Ctx) {
    let king = game.board.player_pieces(game.player).king.single();

    for direction in Direction::ALL {
        if let Some(dst) = king.in_direction(direction) {
            if ctx.our_pieces.contains(dst) {
                continue;
            }

            moves.push(Move::new(king, dst));
        }
    }

    let castle_rights_for_player = match game.player {
        Player::White => game.white_castle_rights,
        Player::Black => game.black_castle_rights,
    };

    let king_start_square = squares::king_start(game.player);

    if king == king_start_square && castle_rights_for_player.can_castle() {
        if castle_rights_for_player.king_side {
            let kingside_required_empty_squares = match game.player {
                Player::White => vec![F1, G1],
                Player::Black => vec![F8, G8],
            };

            let path_to_castle_is_empty = kingside_required_empty_squares
                .iter()
                .all(|&s| !ctx.all_pieces.contains(s));

            if path_to_castle_is_empty {
                moves.push(Move::new(king, squares::kingside_castle_dest(game.player)));
            }
        }

        if castle_rights_for_player.queen_side {
            let queenside_required_empty_squares = match game.player {
                Player::White => vec![B1, C1, D1],
                Player::Black => vec![B8, C8, D8],
            };

            let path_to_castle_is_empty = queenside_required_empty_squares
                .iter()
                .all(|&s| !ctx.all_pieces.contains(s));

            if path_to_castle_is_empty {
                moves.push(Move::new(king, squares::queenside_castle_dest(game.player)));
            }
        }
    }
}
