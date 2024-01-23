use crate::chess::bitboard::{bitboards, Bitboard};
use crate::chess::movegen::{attackers, tables};
use crate::chess::movelist::MoveList;
use crate::chess::square::{squares, Square};
use crate::chess::{game::Game, moves::Move, piece::PromotionPieceKind};

// Convenience method to prevent tests from having to construct their own
// movelist and allow them to iterate easily over the resulting list of moves
pub fn get_pseudo_legal_moves(game: &Game) -> MoveList {
    let mut movelist = MoveList::new();
    generate_pseudo_legal_moves::<true>(game, &mut movelist);
    movelist
}

#[allow(unused)]
pub fn generate_pseudo_legal_moves<const QUIET: bool>(game: &Game, moves: &mut MoveList) {
    let our_pieces = game.board.pieces(game.player);
    let their_pieces = game.board.pieces(game.player.other()).all();
    let all_pieces = our_pieces.all() | their_pieces;

    let king = our_pieces.king().single();

    generate_pawn_moves::<QUIET>(moves, game, our_pieces.pawns(), their_pieces, all_pieces);

    generate_knight_moves::<QUIET>(moves, our_pieces.knights(), their_pieces, all_pieces);

    generate_diagonal_slider_moves::<QUIET>(
        moves,
        our_pieces.bishops() | our_pieces.queens(),
        their_pieces,
        all_pieces,
    );

    generate_orthogonal_slider_moves::<QUIET>(
        moves,
        our_pieces.rooks() | our_pieces.queens(),
        their_pieces,
        all_pieces,
    );

    generate_king_moves::<QUIET>(moves, king, their_pieces, all_pieces);

    if QUIET {
        generate_castles::<QUIET>(moves, game, all_pieces);
    }
}

fn generate_pawn_moves<const QUIET: bool>(
    moves: &mut MoveList,
    game: &Game,
    pawns: Bitboard,
    their_pieces: Bitboard,
    all_pieces: Bitboard,
) {
    // Pawns can move onto empty squares, as long as they block check if in check
    let available_move_squares = !all_pieces;
    let single_push_available_move_pawns = available_move_squares.backward(game.player);

    // Pawns can push once if they can move by pin rules, are not obstructed, and block check if in check
    let can_push_once_pawns = pawns & single_push_available_move_pawns;

    let will_promote_rank = bitboards::pawn_back_rank(game.player.other());

    // Promotion capture: Pawns on the enemy's start rank will promote when capturing
    for pawn in pawns & will_promote_rank {
        let attacks = tables::pawn_attacks(pawn, game.player);

        for target in attacks & their_pieces {
            moves.push(Move::promotion(pawn, target, PromotionPieceKind::Queen));
            moves.push(Move::promotion(pawn, target, PromotionPieceKind::Rook));
            moves.push(Move::promotion(pawn, target, PromotionPieceKind::Knight));
            moves.push(Move::promotion(pawn, target, PromotionPieceKind::Bishop));
        }
    }

    // Promotion push: Pawns on the enemy's start rank will promote when pushing
    for pawn in can_push_once_pawns & will_promote_rank {
        let target = pawn.forward(game.player);

        moves.push(Move::promotion(pawn, target, PromotionPieceKind::Queen));

        // Consider underpromoting pushes to be 'quiet' moves
        if QUIET {
            moves.push(Move::promotion(pawn, target, PromotionPieceKind::Rook));
            moves.push(Move::promotion(pawn, target, PromotionPieceKind::Knight));
            moves.push(Move::promotion(pawn, target, PromotionPieceKind::Bishop));
        }
    }

    // Non-promoting captures: All pawns can capture diagonally
    for pawn in pawns & !will_promote_rank {
        let attacks = tables::pawn_attacks(pawn, game.player);

        for target in attacks & their_pieces {
            moves.push(Move::new(pawn, target));
        }
    }

    // En-passant capture: Pawns either side of the en-passant pawn can capture
    if let Some(en_passant_target) = game.en_passant_target {
        let potential_capturers =
            pawns & tables::pawn_attacks(en_passant_target, game.player.other());

        for potential_en_passant_capture_start in potential_capturers {
            moves.push(Move::new(
                potential_en_passant_capture_start,
                en_passant_target,
            ));
        }
    }

    if QUIET {
        let back_rank = bitboards::pawn_back_rank(game.player);

        // Push: All pawns with an empty square in front of them can move forward
        for pawn in can_push_once_pawns & !will_promote_rank {
            let forward_one = pawn.forward(game.player);
            moves.push(Move::new(pawn, forward_one));
        }

        let double_push_blockers = all_pieces.backward(game.player);

        let can_push_twice_pawns = pawns
            & back_rank
            & !double_push_blockers
            & single_push_available_move_pawns.backward(game.player);

        // Double push: All pawns on the start rank with empty squares in front of them can move forward two squares
        for pawn in can_push_twice_pawns {
            let forward_two = pawn.forward(game.player).forward(game.player);
            moves.push(Move::new(pawn, forward_two));
        }
    }
}

fn generate_knight_moves<const QUIET: bool>(
    moves: &mut MoveList,
    knights: Bitboard,
    their_pieces: Bitboard,
    all_pieces: Bitboard,
) {
    for knight in knights {
        let destinations = tables::knight_attacks(knight);

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
    moves: &mut MoveList,
    diagonal_sliders: Bitboard,
    their_pieces: Bitboard,
    all_pieces: Bitboard,
) {
    for diagonal_slider in diagonal_sliders {
        let destinations = tables::bishop_attacks(diagonal_slider, all_pieces);

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
    moves: &mut MoveList,
    orthogonal_sliders: Bitboard,
    their_pieces: Bitboard,
    all_pieces: Bitboard,
) {
    for orthogonal_slider in orthogonal_sliders {
        let destinations = tables::rook_attacks(orthogonal_slider, all_pieces);

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
    moves: &mut MoveList,
    king: Square,
    their_pieces: Bitboard,
    all_pieces: Bitboard,
) {
    let destinations = tables::king_attacks(king);

    for dst in destinations & their_pieces {
        moves.push(Move::new(king, dst));
    }

    if QUIET {
        for dst in destinations & !all_pieces {
            moves.push(Move::new(king, dst));
        }
    }
}

fn generate_castles<const QUIET: bool>(moves: &mut MoveList, game: &Game, all_pieces: Bitboard) {
    let castle_rights_for_player = game.castle_rights[game.player.array_idx()];

    if castle_rights_for_player.king_side {
        generate_castle_move_for_side::<true>(moves, game, all_pieces);
    }

    if castle_rights_for_player.queen_side {
        generate_castle_move_for_side::<false>(moves, game, all_pieces);
    }
}

fn generate_castle_move_for_side<const KINGSIDE: bool>(
    moves: &mut MoveList,
    game: &Game,
    all_pieces: Bitboard,
) {
    let king_start_square = squares::king_start(game.player);

    let (required_empty_squares, target_square, middle_square) =
        bitboards::castle_squares::<KINGSIDE>(game.player);

    if (required_empty_squares & all_pieces).is_empty()
        && attackers::generate_attackers_of(&game.board, game.player, king_start_square).is_empty()
        && attackers::generate_attackers_of(&game.board, game.player, middle_square).is_empty()
        && attackers::generate_attackers_of(&game.board, game.player, target_square).is_empty()
    {
        moves.push(Move::new(king_start_square, target_square));
    }
}

#[cfg(test)]
mod tests {
    use crate::chess::square::squares::all::*;
    use crate::chess::square::Square;

    use super::*;

    #[inline(always)]
    fn should_allow_move(fen: &str, squares: (Square, Square)) {
        crate::init();
        let game = Game::from_fen(fen).unwrap();
        let mut movelist = MoveList::new();
        generate_pseudo_legal_moves::<true>(&game, &mut movelist);
        let (src, dst) = squares;
        let mv = Move::new(src, dst);

        assert!(movelist.to_vec().iter().any(|m| *m == mv));
    }

    #[inline(always)]
    fn should_not_allow_move(fen: &str, squares: (Square, Square)) {
        crate::init();
        let game = Game::from_fen(fen).unwrap();
        let mut movelist = MoveList::new();
        generate_pseudo_legal_moves::<true>(&game, &mut movelist);
        let (src, dst) = squares;
        let mv = Move::new(src, dst);

        assert!(movelist.to_vec().iter().all(|m| *m != mv));
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
    fn test_en_passant_bug_20230308() {
        should_allow_move(
            "rnbqkbnr/2pppppp/p7/Pp6/8/8/1PPPPPPP/RNBQKBNR w KQkq b6 0 3",
            (A5, B6),
        );
    }
}
