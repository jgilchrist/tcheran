use crate::chess::bitboard::{bitboards, Bitboard};
use crate::chess::movegen::{attackers, pins, tables};
use crate::chess::square::{squares, Square};
use crate::chess::{game::Game, moves::Move, piece::PromotionPieceKind, player::Player};

pub fn generate_moves<const QUIET: bool>(game: &Game) -> Vec<Move> {
    let our_pieces = game.board.player_pieces(game.player);
    let their_pieces = game.board.player_pieces(game.player.other()).all();
    let all_pieces = our_pieces.all() | their_pieces;

    let king = our_pieces.king().single();

    let mut moves: Vec<Move> = Vec::with_capacity(64);

    let checkers = attackers::generate_attackers_of(&game.board, game.player, king);
    let number_of_checkers = checkers.count();

    // If we're in check by more than one attacker, we can only get out of check via a king move
    if number_of_checkers > 1 {
        generate_king_moves::<QUIET>(&mut moves, game, king, their_pieces, all_pieces);
        return moves;
    }

    let check_mask = if number_of_checkers == 1 {
        let checker_sq = checkers.single();
        tables::between(checker_sq, king) | checkers
    } else {
        Bitboard::FULL
    };

    let (orthogonal_pins, diagonal_pins) = pins::get_pins(&game.board, game.player, king);

    generate_pawn_moves::<QUIET>(
        &mut moves,
        game,
        our_pieces.pawns(),
        king,
        their_pieces,
        all_pieces,
        check_mask,
        orthogonal_pins,
        diagonal_pins,
    );
    generate_knight_moves::<QUIET>(
        &mut moves,
        our_pieces.knights(),
        their_pieces,
        all_pieces,
        check_mask,
        orthogonal_pins,
        diagonal_pins,
    );
    generate_diagonal_slider_moves::<QUIET>(
        &mut moves,
        our_pieces.bishops() | our_pieces.queens(),
        their_pieces,
        all_pieces,
        check_mask,
        orthogonal_pins,
        diagonal_pins,
    );
    generate_orthogonal_slider_moves::<QUIET>(
        &mut moves,
        our_pieces.rooks() | our_pieces.queens(),
        their_pieces,
        all_pieces,
        check_mask,
        orthogonal_pins,
        diagonal_pins,
    );
    generate_king_moves::<QUIET>(&mut moves, game, king, their_pieces, all_pieces);

    if QUIET && !checkers.any() {
        generate_castles::<QUIET>(&mut moves, game, all_pieces);
    }

    moves
}

fn generate_pawn_moves<const QUIET: bool>(
    moves: &mut Vec<Move>,
    game: &Game,
    pawns: Bitboard,
    king: Square,
    their_pieces: Bitboard,
    all_pieces: Bitboard,
    check_mask: Bitboard,
    orthogonal_pins: Bitboard,
    diagonal_pins: Bitboard,
) {
    // Pawns that are pinned orthogonally would reveal the king by capturing diagonally
    let can_capture_pawns = pawns & !orthogonal_pins;

    // Pawns that are pinned diagonally would reveal the king by moving forward
    let can_move_pawns = pawns & !diagonal_pins;

    // Pawns can move onto empty squares, as long as they block check if in check
    let available_move_squares = !all_pieces & check_mask;
    let single_push_available_move_pawns = available_move_squares.backward(game.player);

    // Pawns can push once if they can move by pin rules, are not obstructed, and block check if in check
    let can_push_once_pawns = can_move_pawns & single_push_available_move_pawns;

    let capture_targets = their_pieces & check_mask;

    let will_promote_rank = bitboards::pawn_back_rank(game.player.other());

    // Promotion capture: Pawns on the enemy's start rank will promote when capturing
    for pawn in can_capture_pawns & will_promote_rank {
        let mut attacks = tables::pawn_attacks(pawn, game.player);

        if diagonal_pins.contains(pawn) {
            attacks &= diagonal_pins;
        }

        for target in attacks & capture_targets {
            moves.push(Move::promotion(pawn, target, PromotionPieceKind::Queen));
            moves.push(Move::promotion(pawn, target, PromotionPieceKind::Rook));
            moves.push(Move::promotion(pawn, target, PromotionPieceKind::Knight));
            moves.push(Move::promotion(pawn, target, PromotionPieceKind::Bishop));
        }
    }

    // Promotion push: Pawns on the enemy's start rank will promote when pushing
    for pawn in can_push_once_pawns & will_promote_rank {
        let target = pawn.forward(game.player);

        // Pawns cannot push forward if they are pinned orthogonally
        // There's no 'moving along the pin ray' for these pieces, since the target square is empty
        if !orthogonal_pins.contains(pawn) {
            moves.push(Move::promotion(pawn, target, PromotionPieceKind::Queen));

            // Consider underpromoting pushes to be 'quiet' moves
            if QUIET {
                moves.push(Move::promotion(pawn, target, PromotionPieceKind::Rook));
                moves.push(Move::promotion(pawn, target, PromotionPieceKind::Knight));
                moves.push(Move::promotion(pawn, target, PromotionPieceKind::Bishop));
            }
        }
    }

    // Non-promoting captures: All pawns can capture diagonally
    for pawn in can_capture_pawns & !will_promote_rank {
        let mut attacks = tables::pawn_attacks(pawn, game.player);

        if diagonal_pins.contains(pawn) {
            attacks &= diagonal_pins;
        }

        for target in attacks & capture_targets {
            moves.push(Move::new(pawn, target));
        }
    }

    // En-passant capture: Pawns either side of the en-passant pawn can capture
    if let Some(en_passant_target) = game.en_passant_target {
        let captured_pawn = en_passant_target.backward(game.player);

        if (check_mask & (en_passant_target.bb() | captured_pawn.bb())).any() {
            let potential_capturers =
                can_capture_pawns & tables::pawn_attacks(en_passant_target, game.player.other());

            for potential_en_passant_capture_start in potential_capturers {
                // Only consider this pawn if it is not pinned, or if it is pinned but captures along the pin ray
                if !diagonal_pins.contains(potential_en_passant_capture_start)
                    || diagonal_pins.contains(en_passant_target)
                {
                    // We need to check that we do not reveal a check by making this en-passant capture
                    let mut board_without_en_passant_participants = game.board.clone();
                    board_without_en_passant_participants
                        .remove_at(potential_en_passant_capture_start);
                    board_without_en_passant_participants.remove_at(captured_pawn);

                    let king_in_check = attackers::generate_attackers_of(
                        &board_without_en_passant_participants,
                        game.player,
                        king,
                    )
                    .any();

                    if !king_in_check {
                        moves.push(Move::new(
                            potential_en_passant_capture_start,
                            en_passant_target,
                        ));
                    }
                }
            }
        }
    }

    if QUIET {
        let back_rank = bitboards::pawn_back_rank(game.player);

        // Push: All pawns with an empty square in front of them can move forward
        for pawn in can_push_once_pawns & !will_promote_rank {
            let forward_one = pawn.forward(game.player);

            // Pawns cannot push forward if they are pinned orthogonally, unless they're moving along the pin ray
            if !orthogonal_pins.contains(pawn) || orthogonal_pins.contains(forward_one) {
                moves.push(Move::new(pawn, forward_one));
            }
        }

        let double_push_blockers = all_pieces.backward(game.player);

        let can_push_twice_pawns = can_move_pawns
            & back_rank
            & !double_push_blockers
            & single_push_available_move_pawns.backward(game.player);

        // Double push: All pawns on the start rank with empty squares in front of them can move forward two squares
        for pawn in can_push_twice_pawns {
            let forward_two = pawn.forward(game.player).forward(game.player);

            // Pawns cannot push forward if they are pinned orthogonally, unless they are moving along the pin ray
            if !orthogonal_pins.contains(pawn) || orthogonal_pins.contains(forward_two) {
                moves.push(Move::new(pawn, forward_two));
            }
        }
    }
}

fn generate_knight_moves<const QUIET: bool>(
    moves: &mut Vec<Move>,
    knights: Bitboard,
    their_pieces: Bitboard,
    all_pieces: Bitboard,
    check_mask: Bitboard,
    orthogonal_pins: Bitboard,
    diagonal_pins: Bitboard,
) {
    // Pinned knights can't move
    for knight in knights & !(orthogonal_pins | diagonal_pins) {
        let destinations = tables::knight_attacks(knight) & check_mask;

        let capture_destinations = destinations & their_pieces;
        for dst in capture_destinations {
            moves.push(Move::new(knight, dst));
        }

        if QUIET {
            let move_destinations = destinations & !all_pieces;
            for dst in move_destinations {
                moves.push(Move::new(knight, dst));
            }
        }
    }
}

fn generate_diagonal_slider_moves<const QUIET: bool>(
    moves: &mut Vec<Move>,
    diagonal_sliders: Bitboard,
    their_pieces: Bitboard,
    all_pieces: Bitboard,
    check_mask: Bitboard,
    orthogonal_pins: Bitboard,
    diagonal_pins: Bitboard,
) {
    // Diagonal sliders which are pinned orthogonally would expose the king by moving
    for diagonal_slider in diagonal_sliders & !orthogonal_pins {
        let mut destinations = tables::bishop_attacks(diagonal_slider, all_pieces) & check_mask;

        // If the slider is pinned, it can only move along the pin ray
        if diagonal_pins.contains(diagonal_slider) {
            destinations &= diagonal_pins;
        }

        let capture_destinations = destinations & their_pieces;
        for dst in capture_destinations {
            moves.push(Move::new(diagonal_slider, dst));
        }

        if QUIET {
            let move_destinations = destinations & !all_pieces;
            for dst in move_destinations {
                moves.push(Move::new(diagonal_slider, dst));
            }
        }
    }
}

fn generate_orthogonal_slider_moves<const QUIET: bool>(
    moves: &mut Vec<Move>,
    orthogonal_sliders: Bitboard,
    their_pieces: Bitboard,
    all_pieces: Bitboard,
    check_mask: Bitboard,
    orthogonal_pins: Bitboard,
    diagonal_pins: Bitboard,
) {
    // Orthogonal sliders which are pinned diagonally would expose the king by moving
    for orthogonal_slider in orthogonal_sliders & !diagonal_pins {
        let mut destinations = tables::rook_attacks(orthogonal_slider, all_pieces) & check_mask;

        if orthogonal_pins.contains(orthogonal_slider) {
            destinations &= orthogonal_pins;
        }

        let capture_destinations = destinations & their_pieces;
        for dst in capture_destinations {
            moves.push(Move::new(orthogonal_slider, dst));
        }

        if QUIET {
            let move_destinations = destinations & !all_pieces;
            for dst in move_destinations {
                moves.push(Move::new(orthogonal_slider, dst));
            }
        }
    }
}

fn generate_king_moves<const QUIET: bool>(
    moves: &mut Vec<Move>,
    game: &Game,
    king: Square,
    their_pieces: Bitboard,
    all_pieces: Bitboard,
) {
    let destinations = tables::king_attacks(king);

    // When calculating the attacked squares, we need to remove our King from the board.
    // If we don't, squares behind the king look safe (since they are blocked by the king)
    // meaning we'd generate moves away from a slider while in check.
    let mut board_without_king = game.board.clone();
    board_without_king.remove_at(king);

    for dst in destinations & their_pieces {
        if attackers::generate_attackers_of(&board_without_king, game.player, dst).is_empty() {
            moves.push(Move::new(king, dst));
        }
    }

    if QUIET {
        for dst in destinations & !all_pieces {
            if attackers::generate_attackers_of(&board_without_king, game.player, dst).is_empty() {
                moves.push(Move::new(king, dst));
            }
        }
    }
}

fn generate_castles<const QUIET: bool>(moves: &mut Vec<Move>, game: &Game, all_pieces: Bitboard) {
    let castle_rights_for_player = match game.player {
        Player::White => game.white_castle_rights,
        Player::Black => game.black_castle_rights,
    };

    if castle_rights_for_player.king_side {
        generate_castle_move_for_side::<true>(moves, game, all_pieces);
    }

    if castle_rights_for_player.queen_side {
        generate_castle_move_for_side::<false>(moves, game, all_pieces);
    }
}

fn generate_castle_move_for_side<const KINGSIDE: bool>(
    moves: &mut Vec<Move>,
    game: &Game,
    all_pieces: Bitboard,
) {
    let king_start_square = squares::king_start(game.player);

    let (required_empty_squares, target_square, middle_square) =
        bitboards::castle_squares::<KINGSIDE>(game.player);

    if (required_empty_squares & all_pieces).is_empty()
        && attackers::generate_attackers_of(&game.board, game.player, middle_square).is_empty()
        && attackers::generate_attackers_of(&game.board, game.player, target_square).is_empty()
    {
        moves.push(Move::new(king_start_square, target_square));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::square::squares::all::*;
    use crate::chess::square::Square;

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
