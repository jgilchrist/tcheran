use crate::{
    board::Board,
    direction::Direction,
    game::Game,
    move_tables,
    moves::Move,
    piece::PromotionPieceKind,
    player::Player,
    squares::{self, all::*, Squares},
};

struct Ctx {
    their_pieces: Squares,
    enemy_or_empty: Squares,
    all_pieces: Squares,
}

pub fn generate_all_attacks(board: &Board, player: Player) -> Squares {
    let mut attacks = Squares::none();

    let our_pieces = board.player_pieces(player);
    let their_pieces = board.player_pieces(player.other()).all();
    let all_pieces = our_pieces.all() | their_pieces;

    for pawn in our_pieces.pawns {
        attacks |= move_tables::pawn_attacks(pawn, player);
    }

    for knight in our_pieces.knights {
        attacks |= move_tables::knight_attacks(knight);
    }

    for bishop in our_pieces.bishops {
        attacks |= move_tables::bishop_attacks(bishop, all_pieces);
    }

    for rook in our_pieces.rooks {
        attacks |= move_tables::rook_attacks(rook, all_pieces);
    }

    for queen in our_pieces.queens {
        attacks |= move_tables::queen_attacks(queen, all_pieces);
    }

    for king in our_pieces.king {
        attacks |= move_tables::king_attacks(king);
    }

    attacks
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
    let enemy_or_empty = our_pieces.invert();
    let their_pieces = game.board.player_pieces(game.player.other()).all();
    let all_pieces = our_pieces | their_pieces;

    Ctx {
        their_pieces,
        enemy_or_empty,
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
        let attacks = move_tables::pawn_attacks(start, game.player) & ctx.their_pieces;

        for dst in attacks {
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

    for knight in knights {
        let destinations = move_tables::knight_attacks(knight) & ctx.enemy_or_empty;

        for dst in destinations {
            moves.push(Move::new(knight, dst));
        }
    }
}

fn generate_bishop_moves(moves: &mut Vec<Move>, game: &Game, ctx: &Ctx) {
    let bishops = game.board.player_pieces(game.player).bishops;

    for bishop in bishops {
        let destinations = move_tables::bishop_attacks(bishop, ctx.all_pieces) & ctx.enemy_or_empty;

        for dst in destinations {
            moves.push(Move::new(bishop, dst));
        }
    }
}

fn generate_rook_moves(moves: &mut Vec<Move>, game: &Game, ctx: &Ctx) {
    let rooks = game.board.player_pieces(game.player).rooks;

    for rook in rooks {
        let destinations = move_tables::rook_attacks(rook, ctx.all_pieces) & ctx.enemy_or_empty;

        for dst in destinations {
            moves.push(Move::new(rook, dst));
        }
    }
}

fn generate_queen_moves(moves: &mut Vec<Move>, game: &Game, ctx: &Ctx) {
    let queens = game.board.player_pieces(game.player).queens;

    for queen in queens {
        let destinations = move_tables::queen_attacks(queen, ctx.all_pieces) & ctx.enemy_or_empty;

        for dst in destinations {
            moves.push(Move::new(queen, dst));
        }
    }
}

fn generate_king_moves(moves: &mut Vec<Move>, game: &Game, ctx: &Ctx) {
    let king = game.board.player_pieces(game.player).king.single();

    let destinations = move_tables::king_attacks(king) & ctx.enemy_or_empty;

    for dst in destinations {
        moves.push(Move::new(king, dst));
    }

    let king_start_square = squares::king_start(game.player);

    if king == king_start_square {
        let castle_rights_for_player = match game.player {
            Player::White => game.white_castle_rights,
            Player::Black => game.black_castle_rights,
        };

        if castle_rights_for_player.can_castle() {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_rook_move() {
        crate::init();
        let game =
            Game::from_fen("rnbqkbnr/1ppppppp/p7/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 0 2").unwrap();
        let moves = game.legal_moves();

        let has_rook_a1_a2 = moves.iter().any(|m| m.src == A1 && m.dst == A2);

        assert!(has_rook_a1_a2);
    }

    #[test]
    fn test_simple_bishop_move() {
        crate::init();
        let game = Game::from_fen("rnbqkbnr/1ppppp1p/p5p1/8/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 3")
            .unwrap();
        let moves = game.legal_moves();

        let has_simple_bishop_move = moves.iter().any(|m| m.src == B2 && m.dst == C3);
        let has_rook_capture = moves.iter().any(|m| m.src == B2 && m.dst == H8);

        assert!(has_simple_bishop_move);
        assert!(has_rook_capture);
    }
}
