use crate::bitboard::{bitboards, Bitboard};
use crate::board::PlayerPieces;
use crate::movegen::{attackers, pins, tables};
use crate::piece::PieceKind;
use crate::square::{squares, Square};
use crate::{
    direction::Direction, game::Game, moves::Move, piece::PromotionPieceKind, player::Player,
};

struct Ctx<'gen> {
    all_pieces: Bitboard,
    our_pieces: &'gen PlayerPieces,
    their_pieces: Bitboard,

    king: Square,

    checkers: Bitboard,
    pinned: Bitboard,
    pinners: Bitboard,
}

pub fn generate_moves<const QUIET: bool>(game: &Game) -> Vec<Move> {
    let ctx = get_ctx(game);
    let mut moves: Vec<Move> = Vec::with_capacity(64);

    let number_of_checkers = ctx.checkers.count();

    // When calculating the attacked squares, we need to remove our King from the board.
    // If we don't, squares behind the king look safe (since they are blocked by the king)
    // meaning we'd generate moves away from a slider while in check.
    let mut board_without_king = game.board;
    board_without_king.remove_at(ctx.king);
    let attacked_squares =
        attackers::generate_all_attacks(&board_without_king, game.player.other());

    // If we're in check by more than one attacker, we can only get out of check via a king move
    if number_of_checkers > 1 {
        generate_king_moves::<QUIET>(&mut moves, attacked_squares, &ctx);
        return moves;
    }

    let checker: Option<(Square, PieceKind)> = Square::from_bitboard_maybe(ctx.checkers)
        .map(|sq| (sq, game.board.piece_at(sq).unwrap().kind));

    let move_mask = match checker {
        // If we're not in check, pieces can move as normal
        None => !ctx.all_pieces,
        // If we are in check, pieces can only move to block the check
        Some((checker_sq, piece)) => {
            if piece.is_slider() {
                // If the piece giving check casts a ray, we can block the check by moving any piece
                // into that ray
                tables::between(checker_sq, ctx.king)
            } else {
                // If the piece giving check jumps, we can't get in the way, only capture that piece
                Bitboard::EMPTY
            }
        }
    };

    let capture_mask = match checker {
        // If we're not in check, we can capture any enemy piece
        None => ctx.their_pieces,
        // If we are in check, we can get out by capturing our checker
        Some(_) => ctx.checkers,
    };

    generate_pawn_moves::<QUIET>(&mut moves, game, move_mask, capture_mask, &ctx);
    generate_knight_moves::<QUIET>(&mut moves, move_mask, capture_mask, &ctx);
    generate_diagonal_slider_moves::<QUIET>(&mut moves, move_mask, capture_mask, &ctx);
    generate_orthogonal_slider_moves::<QUIET>(&mut moves, move_mask, capture_mask, &ctx);
    generate_king_moves::<QUIET>(&mut moves, attacked_squares, &ctx);
    generate_castles::<QUIET>(&mut moves, game, attacked_squares, &ctx);
    moves
}

fn get_ctx(game: &Game) -> Ctx {
    let our_pieces = game.board.player_pieces(game.player);
    let their_pieces = game.board.player_pieces(game.player.other()).all();
    let all_pieces = our_pieces.all() | their_pieces;

    let king = our_pieces.king.single();
    let (checkers, pinned, pinners) = pins::get_pins_and_checkers(&game.board, game.player, king);

    Ctx {
        all_pieces,
        our_pieces,
        their_pieces,

        king,

        checkers,
        pinned,
        pinners,
    }
}

fn generate_pawn_moves<const QUIET: bool>(
    moves: &mut Vec<Move>,
    game: &Game,
    move_mask: Bitboard,
    capture_mask: Bitboard,
    ctx: &Ctx,
) {
    let pawns = ctx.our_pieces.pawns;

    let pawn_move_direction = Direction::pawn_move_direction(game.player);
    let pawn_capture_left_direction = Direction::pawn_capture_left_direction(game.player);
    let pawn_capture_right_direction = Direction::pawn_capture_right_direction(game.player);

    let back_rank = bitboards::pawn_back_rank(game.player);
    let will_promote_rank = bitboards::pawn_back_rank(game.player.other());

    let non_pinned_non_promoting_pawns = pawns & !will_promote_rank & !ctx.pinned;
    let non_pinned_promoting_pawns = pawns & will_promote_rank & !ctx.pinned;

    let pinned_pawns = pawns & ctx.pinned;

    // TODO: Is this redundant with move_mask?
    let pawn_move_blockers = ctx.all_pieces.in_direction(!pawn_move_direction);
    let double_push_blockers = pawn_move_blockers.in_direction(!pawn_move_direction);

    let capturable_pieces_left =
        (ctx.their_pieces & capture_mask).in_direction(!pawn_capture_left_direction);

    let capturable_pinner_pieces_left =
        (ctx.their_pieces & capture_mask & ctx.pinners).in_direction(!pawn_capture_left_direction);

    let capturable_pieces_right =
        (ctx.their_pieces & capture_mask).in_direction(!pawn_capture_right_direction);

    let capturable_pinner_pieces_right =
        (ctx.their_pieces & capture_mask & ctx.pinners).in_direction(!pawn_capture_right_direction);

    // Promotion capture: Pawns on the enemy's start rank will promote when capturing
    for pawn in non_pinned_promoting_pawns & capturable_pieces_left {
        let capture_left_square = pawn.in_direction(pawn_capture_left_direction);

        for promotion in PromotionPieceKind::ALL {
            moves.push(Move::new_with_promotion(
                pawn,
                capture_left_square,
                *promotion,
            ));
        }
    }

    for pawn in non_pinned_promoting_pawns & capturable_pieces_right {
        let capture_right_square = pawn.in_direction(pawn_capture_right_direction);

        for promotion in PromotionPieceKind::ALL {
            moves.push(Move::new_with_promotion(
                pawn,
                capture_right_square,
                *promotion,
            ));
        }
    }

    // Promotion push: Pawns on the enemy's start rank will promote when pushing
    for pawn in non_pinned_promoting_pawns & !pawn_move_blockers {
        let forward_one = pawn.in_direction(pawn_move_direction);

        if move_mask.contains(forward_one) {
            for promotion in PromotionPieceKind::ALL {
                moves.push(Move::new_with_promotion(pawn, forward_one, *promotion));
            }
        }
    }

    // Non-promoting captures: All pawns can capture diagonally
    for pawn in non_pinned_non_promoting_pawns & capturable_pieces_left {
        let capture_square = pawn.in_direction(pawn_capture_left_direction);

        moves.push(Move::new(pawn, capture_square));
    }

    for pawn in non_pinned_non_promoting_pawns & capturable_pieces_right {
        let capture_square = pawn.in_direction(pawn_capture_right_direction);

        moves.push(Move::new(pawn, capture_square));
    }

    // Pinned pawns can only capture pinners along their pin ray
    for pinned_pawn in pinned_pawns & capturable_pinner_pieces_left {
        let ray = tables::ray(pinned_pawn, ctx.king);
        let capture_square = pinned_pawn.in_direction(pawn_capture_left_direction);

        if ray.contains(capture_square) {
            moves.push(Move::new(pinned_pawn, capture_square));
        }
    }

    for pinned_pawn in pinned_pawns & capturable_pinner_pieces_right {
        let ray = tables::ray(pinned_pawn, ctx.king);
        let capture_square = pinned_pawn.in_direction(pawn_capture_right_direction);

        if ray.contains(capture_square) {
            moves.push(Move::new(pinned_pawn, capture_square));
        }
    }

    // En-passant capture: Pawns either side of the en-passant pawn can capture
    if let Some(en_passant_target) = game.en_passant_target {
        // We may use en passant to move the pawn to a square which blocks check, so we & with move_mask
        let pawns_in_capturing_positions =
            tables::pawn_attacks(en_passant_target, game.player.other());

        for potential_en_passant_capture_start in
            pawns_in_capturing_positions & non_pinned_non_promoting_pawns
        {
            let captured_pawn = en_passant_target.in_direction(!pawn_move_direction);

            // We need to check that we do not reveal a check by making this en-passant capture
            let mut board_without_en_passant_participants = game.board;
            board_without_en_passant_participants.remove_at(potential_en_passant_capture_start);
            board_without_en_passant_participants.remove_at(captured_pawn);

            let king_in_check = attackers::generate_attackers_of(
                &board_without_en_passant_participants,
                game.player,
                ctx.king,
            )
            .any();

            if king_in_check {
                continue;
            }

            moves.push(Move::new(
                potential_en_passant_capture_start,
                en_passant_target,
            ));
        }
    }

    if QUIET {
        // Push: All pawns with an empty square in front of them can move forward
        for pawn in non_pinned_non_promoting_pawns & !pawn_move_blockers {
            let forward_one = pawn.in_direction(pawn_move_direction);

            if move_mask.contains(forward_one) {
                moves.push(Move::new(pawn, forward_one));
            }
        }

        // Push: Pinned pawns can move along the pin ray
        for pinned_pawn in pinned_pawns & !pawn_move_blockers {
            let ray = tables::ray(pinned_pawn, ctx.king);
            let forward_one = pinned_pawn.in_direction(pawn_move_direction);

            if move_mask.contains(forward_one) && ray.contains(forward_one) {
                moves.push(Move::new(pinned_pawn, forward_one));
            }
        }

        // Double push: All pawns on the start rank with empty squares in front of them can move forward two squares
        for pawn in
            non_pinned_non_promoting_pawns & back_rank & !pawn_move_blockers & !double_push_blockers
        {
            let forward_two = pawn
                .in_direction(pawn_move_direction)
                .in_direction(pawn_move_direction);

            if move_mask.contains(forward_two) {
                moves.push(Move::new(pawn, forward_two));
            }
        }

        // Double push: Pinned pawns can still double-push along the pin ray
        for pinned_pawn in pinned_pawns & back_rank & !pawn_move_blockers & !double_push_blockers {
            let ray = tables::ray(pinned_pawn, ctx.king);
            let forward_two = pinned_pawn
                .in_direction(pawn_move_direction)
                .in_direction(pawn_move_direction);

            if move_mask.contains(forward_two) && ray.contains(forward_two) {
                moves.push(Move::new(pinned_pawn, forward_two));
            }
        }
    }
}

fn generate_knight_moves<const QUIET: bool>(
    moves: &mut Vec<Move>,
    move_mask: Bitboard,
    capture_mask: Bitboard,
    ctx: &Ctx,
) {
    let knights = ctx.our_pieces.knights;

    // Pinned knights can't move
    for knight in knights & !ctx.pinned {
        let destinations = tables::knight_attacks(knight);
        let move_destinations = destinations & move_mask;
        let capture_destinations = destinations & capture_mask;

        for dst in capture_destinations {
            moves.push(Move::new(knight, dst));
        }

        if QUIET {
            for dst in move_destinations {
                moves.push(Move::new(knight, dst));
            }
        }
    }
}

fn generate_diagonal_slider_moves<const QUIET: bool>(
    moves: &mut Vec<Move>,
    move_mask: Bitboard,
    capture_mask: Bitboard,
    ctx: &Ctx,
) {
    let diagonal_sliders = ctx.our_pieces.bishops | ctx.our_pieces.queens;

    for diagonal_slider in diagonal_sliders & !ctx.pinned {
        let destinations = tables::bishop_attacks(diagonal_slider, ctx.all_pieces);
        let move_destinations = destinations & move_mask;
        let capture_destinations = destinations & capture_mask;

        for dst in capture_destinations {
            moves.push(Move::new(diagonal_slider, dst));
        }

        if QUIET {
            for dst in move_destinations {
                moves.push(Move::new(diagonal_slider, dst));
            }
        }
    }

    // Pinned pieces can move along the pin ray, or capture the pinning piece
    for pinned_diagonal_slider in diagonal_sliders & ctx.pinned {
        let destinations = tables::bishop_attacks(pinned_diagonal_slider, ctx.all_pieces);
        let ray = tables::ray(pinned_diagonal_slider, ctx.king);

        let move_destinations = destinations & ray & move_mask;
        let capture_destinations = destinations & ray & capture_mask;

        for dst in capture_destinations {
            moves.push(Move::new(pinned_diagonal_slider, dst));
        }

        if QUIET {
            for dst in move_destinations {
                moves.push(Move::new(pinned_diagonal_slider, dst));
            }
        }
    }
}

fn generate_orthogonal_slider_moves<const QUIET: bool>(
    moves: &mut Vec<Move>,
    move_mask: Bitboard,
    capture_mask: Bitboard,
    ctx: &Ctx,
) {
    let orthogonal_sliders = ctx.our_pieces.rooks | ctx.our_pieces.queens;

    for orthogonal_slider in orthogonal_sliders & !ctx.pinned {
        let destinations = tables::rook_attacks(orthogonal_slider, ctx.all_pieces);
        let move_destinations = destinations & move_mask;
        let capture_destinations = destinations & capture_mask;

        for dst in capture_destinations {
            moves.push(Move::new(orthogonal_slider, dst));
        }

        if QUIET {
            for dst in move_destinations {
                moves.push(Move::new(orthogonal_slider, dst));
            }
        }
    }

    // Pinned pieces can move along the pin ray, or capture the pinning piece
    for pinned_orthogonal_slider in orthogonal_sliders & ctx.pinned {
        let destinations = tables::rook_attacks(pinned_orthogonal_slider, ctx.all_pieces);
        let ray = tables::ray(pinned_orthogonal_slider, ctx.king);

        let move_destinations = destinations & ray & move_mask;
        let capture_destinations = destinations & ray & capture_mask;

        for dst in capture_destinations {
            moves.push(Move::new(pinned_orthogonal_slider, dst));
        }

        if QUIET {
            for dst in move_destinations {
                moves.push(Move::new(pinned_orthogonal_slider, dst));
            }
        }
    }
}

fn generate_king_moves<const QUIET: bool>(
    moves: &mut Vec<Move>,
    attacked_squares: Bitboard,
    ctx: &Ctx,
) {
    let king = ctx.our_pieces.king.single();

    // Kings can't move to attacked squares
    let king_move_mask = !ctx.all_pieces & !attacked_squares;

    // Kings can't capture defended pieces
    let king_capture_mask = ctx.their_pieces & !attacked_squares;

    let destinations = tables::king_attacks(king);

    for dst in destinations & king_capture_mask {
        moves.push(Move::new(king, dst));
    }

    if QUIET {
        for dst in destinations & king_move_mask {
            moves.push(Move::new(king, dst));
        }
    }
}

fn generate_castles<const QUIET: bool>(
    moves: &mut Vec<Move>,
    game: &Game,
    attacked_squares: Bitboard,
    ctx: &Ctx,
) {
    if !QUIET {
        return;
    }

    let king = ctx.our_pieces.king.single();

    // We can't castle if we're in check
    if attacked_squares.contains(king) {
        return;
    }

    let king_start_square = squares::king_start(game.player);

    // We can only castle if the King is still on its start square
    if king != king_start_square {
        return;
    }

    let castle_rights_for_player = match game.player {
        Player::White => game.white_castle_rights,
        Player::Black => game.black_castle_rights,
    };

    // If we already forfeited castle rights, we can't castle
    if !castle_rights_for_player.can_castle() {
        return;
    }

    if castle_rights_for_player.king_side {
        let kingside_required_empty_and_not_attacked_squares =
            bitboards::kingside_required_empty_and_not_attacked_squares(game.player);

        let pieces_in_the_way = kingside_required_empty_and_not_attacked_squares & ctx.all_pieces;
        let attacked_squares_in_the_way =
            kingside_required_empty_and_not_attacked_squares & attacked_squares;
        let squares_preventing_castling = pieces_in_the_way | attacked_squares_in_the_way;

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
        let attacked_squares_in_the_way =
            queenside_required_not_attacked_squares & attacked_squares;
        let squares_preventing_castling = pieces_in_the_way | attacked_squares_in_the_way;

        if squares_preventing_castling.is_empty() {
            moves.push(Move::new(king, squares::queenside_castle_dest(game.player)));
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
        let game = Game::from_fen(fen).unwrap();
        let moves = game.moves();
        let (src, dst) = squares;
        let mv = Move::new(src, dst);

        assert!(moves.iter().any(|m| *m == mv));
    }

    #[inline(always)]
    fn should_not_allow_move(fen: &str, squares: (Square, Square)) {
        crate::init();
        let game = Game::from_fen(fen).unwrap();
        let moves = generate_moves::<true>(&game);
        let (src, dst) = squares;
        let mv = Move::new(src, dst);

        assert!(moves.iter().all(|m| *m != mv));
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
    fn test_cant_capture_own_king() {
        should_not_allow_move(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            (F1, G1),
        );
    }

    #[test]
    fn test_kiwipete_en_passant_bug() {
        should_allow_move(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/Pp2P3/2N2Q1p/1PPBBPPP/R3K2R b KQkq a3 0 1",
            (B4, A3),
        );
    }

    #[test]
    fn test_pawn_push_along_pin_bug() {
        should_allow_move(
            "rnb1kbnr/pppp1ppp/4pq2/8/8/5P2/PPPPPKPP/RNBQ1BNR w kq - 2 3",
            (F3, F4),
        );
    }

    #[test]
    fn test_forbid_en_passant_revealed_check() {
        should_not_allow_move("8/8/8/8/k2Pp2Q/8/8/3K4 b - d3 0 1", (E4, D3));
    }

    #[test]
    fn test_forbid_pushing_pawn_into_pinning_piece() {
        should_not_allow_move(
            "rnbq2kr/pp1Pbppp/2p3Q1/8/2B5/8/PPP1NnPP/RNB1K2R b KQ - 4 9",
            (F7, G6),
        );
    }

    #[test]
    fn test_en_passant_bug_20230308() {
        should_allow_move(
            "rnbqkbnr/2pppppp/p7/Pp6/8/8/1PPPPPPP/RNBQKBNR w KQkq b6 0 3",
            (A5, B6),
        );
    }
}
