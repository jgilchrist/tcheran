use crate::bitboard::{bitboards, Bitboard};
use crate::square::{squares, Square};
use crate::{
    board::Board, direction::Direction, game::Game, move_tables, moves::Move,
    piece::PromotionPieceKind, player::Player,
};

struct Ctx {
    all_pieces: Bitboard,
    their_pieces: Bitboard,
}

#[must_use]
pub fn generate_all_attacks(board: &Board, player: Player) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

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

#[must_use]
pub fn generate_attackers_of(board: &Board, player: Player, square: Square) -> Bitboard {
    let mut attackers = Bitboard::EMPTY;
    let our_pieces = board.player_pieces(player);
    let their_pieces = board.player_pieces(player.other());
    let all_pieces = our_pieces.all() | their_pieces.all();

    // Pawns: A square is attacked by pawns in the same positions as a pawn could capture if it was on
    // that square
    attackers |= move_tables::pawn_attacks(square, player) & their_pieces.pawns;

    // Knights: A square is attacked by any squares a knight could reach if it were on that square
    attackers |= move_tables::knight_attacks(square) & their_pieces.knights;

    // Sliders: A square is attacked by any squares a
    attackers |= move_tables::bishop_attacks(square, all_pieces) & their_pieces.bishops;
    attackers |= move_tables::rook_attacks(square, all_pieces) & their_pieces.rooks;
    attackers |= move_tables::queen_attacks(square, all_pieces) & their_pieces.queens;
    attackers |= move_tables::king_attacks(square) & their_pieces.king;

    attackers
}

#[allow(clippy::struct_excessive_bools)]
pub struct MoveTypes {
    pub quiet: bool,
    pub captures: bool,
    pub promotions: bool,
    pub castles: bool,
}

impl MoveTypes {
    pub const ALL: Self = Self {
        quiet: true,
        captures: true,
        promotions: true,
        castles: true,
    };

    pub const QUIESCENCE: Self = Self {
        captures: true,
        promotions: true,

        quiet: false,
        castles: false,
    };
}

#[must_use]
pub fn generate_moves(game: &Game, move_types: &MoveTypes) -> Vec<Move> {
    let ctx = get_ctx(game);

    let mut moves: Vec<Move> = Vec::with_capacity(64);
    generate_pawn_moves(&mut moves, game, move_types, &ctx);
    generate_knight_moves(&mut moves, game, move_types, &ctx);
    generate_bishop_moves(&mut moves, game, move_types, &ctx);
    generate_rook_moves(&mut moves, game, move_types, &ctx);
    generate_queen_moves(&mut moves, game, move_types, &ctx);
    generate_king_moves(&mut moves, game, move_types, &ctx);
    moves
}

fn get_ctx(game: &Game) -> Ctx {
    let our_pieces = game.board.player_pieces(game.player).all();
    let their_pieces = game.board.player_pieces(game.player.other()).all();
    let all_pieces = our_pieces | their_pieces;

    Ctx {
        all_pieces,
        their_pieces,
    }
}

fn generate_pawn_moves(moves: &mut Vec<Move>, game: &Game, move_types: &MoveTypes, ctx: &Ctx) {
    let pawns = game.board.player_pieces(game.player).pawns;

    let pawn_move_direction = Direction::pawn_move_direction(game.player);
    let back_rank = bitboards::pawn_back_rank(game.player);
    let will_promote_rank = bitboards::pawn_back_rank(game.player.other());

    let non_promoting_pawns = pawns & !will_promote_rank;
    let promoting_pawns = pawns & will_promote_rank;

    let pawn_move_blockers = ctx.all_pieces.in_direction(!pawn_move_direction);
    let double_push_blockers = pawn_move_blockers.in_direction(!pawn_move_direction);

    let capturable_pieces_left = ctx
        .their_pieces
        .in_direction(!pawn_move_direction)
        .in_direction(Direction::West);

    let capturable_pieces_right = ctx
        .their_pieces
        .in_direction(!pawn_move_direction)
        .in_direction(Direction::East);

    if move_types.captures {
        // Promotion capture: Pawns on the enemy's start rank will promote when capturing
        for pawn in promoting_pawns & capturable_pieces_left {
            let capture_left_square = pawn
                .in_direction(pawn_move_direction)
                .in_direction(Direction::East);

            for promotion in PromotionPieceKind::ALL {
                moves.push(Move::new_with_promotion(
                    pawn,
                    capture_left_square,
                    *promotion,
                ));
            }
        }

        for pawn in promoting_pawns & capturable_pieces_right {
            let capture_right_square = pawn
                .in_direction(pawn_move_direction)
                .in_direction(Direction::West);

            for promotion in PromotionPieceKind::ALL {
                moves.push(Move::new_with_promotion(
                    pawn,
                    capture_right_square,
                    *promotion,
                ));
            }
        }
    }

    if move_types.promotions {
        // Promotion push: Pawns on the enemy's start rank will promote when pushing
        for pawn in promoting_pawns & !pawn_move_blockers {
            let forward_one = pawn.in_direction(pawn_move_direction);
            for promotion in PromotionPieceKind::ALL {
                moves.push(Move::new_with_promotion(pawn, forward_one, *promotion));
            }
        }
    }

    if move_types.captures {
        // Non-promoting captures: All pawns can capture diagonally
        for pawn in non_promoting_pawns & capturable_pieces_left {
            let capture_square = pawn
                .in_direction(pawn_move_direction)
                .in_direction(Direction::East);

            moves.push(Move::new(pawn, capture_square));
        }

        for pawn in non_promoting_pawns & capturable_pieces_right {
            let capture_square = pawn
                .in_direction(pawn_move_direction)
                .in_direction(Direction::West);

            moves.push(Move::new(pawn, capture_square));
        }

        // En-passant capture: Pawns either side of the en-passant pawn can capture
        if let Some(en_passant_target) = game.en_passant_target {
            let capture_squares = move_tables::pawn_attacks(en_passant_target, game.player.other());

            for potential_en_passant_capture_start in capture_squares & non_promoting_pawns {
                moves.push(Move::new(
                    potential_en_passant_capture_start,
                    en_passant_target,
                ));
            }
        }
    }

    if move_types.quiet {
        // Push: All pawns with an empty square in front of them can move forward
        for pawn in non_promoting_pawns & !pawn_move_blockers {
            let forward_one = pawn.in_direction(pawn_move_direction);
            moves.push(Move::new(pawn, forward_one));
        }

        // Double push: All pawns on the start rank with empty squares in front of them can move forward two squares
        for pawn in non_promoting_pawns & back_rank & !pawn_move_blockers & !double_push_blockers {
            let forward_two = pawn
                .in_direction(pawn_move_direction)
                .in_direction(pawn_move_direction);

            moves.push(Move::new(pawn, forward_two));
        }
    }
}

fn generate_knight_moves(moves: &mut Vec<Move>, game: &Game, move_types: &MoveTypes, ctx: &Ctx) {
    let knights = game.board.player_pieces(game.player).knights;

    for knight in knights {
        let destinations = move_tables::knight_attacks(knight);
        let move_destinations = destinations & !ctx.all_pieces;
        let capture_destinations = destinations & ctx.their_pieces;

        if move_types.captures {
            for dst in capture_destinations {
                moves.push(Move::new(knight, dst));
            }
        }

        if move_types.quiet {
            for dst in move_destinations {
                moves.push(Move::new(knight, dst));
            }
        }
    }
}

fn generate_bishop_moves(moves: &mut Vec<Move>, game: &Game, move_types: &MoveTypes, ctx: &Ctx) {
    let bishops = game.board.player_pieces(game.player).bishops;

    for bishop in bishops {
        let destinations = move_tables::bishop_attacks(bishop, ctx.all_pieces);
        let move_destinations = destinations & !ctx.all_pieces;
        let capture_destinations = destinations & ctx.their_pieces;

        if move_types.captures {
            for dst in capture_destinations {
                moves.push(Move::new(bishop, dst));
            }
        }

        if move_types.quiet {
            for dst in move_destinations {
                moves.push(Move::new(bishop, dst));
            }
        }
    }
}

fn generate_rook_moves(moves: &mut Vec<Move>, game: &Game, move_types: &MoveTypes, ctx: &Ctx) {
    let rooks = game.board.player_pieces(game.player).rooks;

    for rook in rooks {
        let destinations = move_tables::rook_attacks(rook, ctx.all_pieces);
        let move_destinations = destinations & !ctx.all_pieces;
        let capture_destinations = destinations & ctx.their_pieces;

        if move_types.captures {
            for dst in capture_destinations {
                moves.push(Move::new(rook, dst));
            }
        }

        if move_types.quiet {
            for dst in move_destinations {
                moves.push(Move::new(rook, dst));
            }
        }
    }
}

fn generate_queen_moves(moves: &mut Vec<Move>, game: &Game, move_types: &MoveTypes, ctx: &Ctx) {
    let queens = game.board.player_pieces(game.player).queens;

    for queen in queens {
        let destinations = move_tables::queen_attacks(queen, ctx.all_pieces);
        let move_destinations = destinations & !ctx.all_pieces;
        let capture_destinations = destinations & ctx.their_pieces;

        if move_types.captures {
            for dst in capture_destinations {
                moves.push(Move::new(queen, dst));
            }
        }

        if move_types.quiet {
            for dst in move_destinations {
                moves.push(Move::new(queen, dst));
            }
        }
    }
}

fn generate_king_moves(moves: &mut Vec<Move>, game: &Game, move_types: &MoveTypes, ctx: &Ctx) {
    let king = game.board.player_pieces(game.player).king.single();

    let destinations = move_tables::king_attacks(king);
    let move_destinations = destinations & !ctx.all_pieces;
    let capture_destinations = destinations & ctx.their_pieces;

    if move_types.castles {
        let king_start_square = squares::king_start(game.player);

        if king == king_start_square {
            let castle_rights_for_player = match game.player {
                Player::White => game.white_castle_rights,
                Player::Black => game.black_castle_rights,
            };

            if castle_rights_for_player.can_castle() {
                let their_attacks = generate_all_attacks(&game.board, game.player.other());

                if !their_attacks.contains(king) {
                    if castle_rights_for_player.king_side {
                        let kingside_required_empty_and_not_attacked_squares =
                            bitboards::kingside_required_empty_and_not_attacked_squares(
                                game.player,
                            );

                        let pieces_in_the_way =
                            kingside_required_empty_and_not_attacked_squares & ctx.all_pieces;
                        let attacked_squares =
                            kingside_required_empty_and_not_attacked_squares & their_attacks;
                        let squares_preventing_castling = pieces_in_the_way | attacked_squares;

                        if squares_preventing_castling.is_empty() {
                            moves.push(Move::new(king, squares::kingside_castle_dest(game.player)));
                        }
                    }

                    if castle_rights_for_player.queen_side {
                        let queenside_required_empty_squares =
                            bitboards::queenside_required_empty_squares(game.player);
                        let queenside_required_not_attacked_squares =
                            bitboards::queenside_required_not_attacked_squares(game.player);

                        let pieces_in_the_way = queenside_required_empty_squares & ctx.all_pieces;
                        let attacked_squares =
                            queenside_required_not_attacked_squares & their_attacks;
                        let squares_preventing_castling = pieces_in_the_way | attacked_squares;

                        if squares_preventing_castling.is_empty() {
                            moves
                                .push(Move::new(king, squares::queenside_castle_dest(game.player)));
                        }
                    }
                }
            }
        }
    }

    if move_types.captures {
        for dst in capture_destinations {
            moves.push(Move::new(king, dst));
        }
    }

    if move_types.quiet {
        for dst in move_destinations {
            moves.push(Move::new(king, dst));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::squares::all::*;
    use crate::square::Square;

    #[inline(always)]
    fn should_allow_move(fen: &str, squares: (Square, Square)) {
        crate::init();
        let mut game = Game::from_fen(fen).unwrap();

        let moves = game
            .pseudo_legal_moves()
            .into_iter()
            .filter(|m| {
                let player = game.player;
                game.make_move(m);
                let is_in_check = game.board.king_in_check(player);
                game.undo_move();
                !is_in_check
            })
            .collect::<Vec<_>>();

        let (src, dst) = squares;
        let mv = Move::new(src, dst);

        assert!(moves.iter().any(|m| *m == mv));
    }

    #[test]
    fn test_simple_rook_move() {
        should_allow_move(
            "rnbqkbnr/1ppppppp/p7/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 0 2",
            (A1, A2),
        );
    }

    #[test]
    fn test_simple_bishop_move() {
        let fen = "rnbqkbnr/1ppppp1p/p5p1/8/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 3";
        should_allow_move(fen, (B2, C3));
        should_allow_move(fen, (B2, H8));
    }

    #[test]
    fn test_en_passant_bug_20230308() {
        should_allow_move(
            "rnbqkbnr/2pppppp/p7/Pp6/8/8/1PPPPPPP/RNBQKBNR w KQkq b6 0 3",
            (A5, B6),
        );
    }
}
