use crate::game::Game;
use crate::moves::Move;

#[must_use]
pub fn perft(depth: u8, game: &mut Game) -> usize {
    if depth == 0 {
        // PERF: Replace with legal_moves once we're generating legal moves
        return 1;
    }

    game.pseudo_legal_moves()
        .iter()
        .filter_map(|m| {
            let player = game.player;
            game.make_move(m);

            if game.board.king_in_check(player) {
                game.undo_move();
                return None;
            }

            let result = perft(depth - 1, game);
            game.undo_move();

            Some(result)
        })
        .sum()
}

#[must_use]
pub fn perft_div(depth: u8, game: &mut Game) -> Vec<(Move, usize)> {
    let root_moves = game.pseudo_legal_moves();
    let mut perft_for_moves: Vec<(Move, usize)> = vec![];

    for mv in &root_moves {
        let player = game.player;
        game.make_move(mv);

        if game.board.king_in_check(player) {
            game.undo_move();
            continue;
        }

        let number_for_mv = if depth == 1 {
            1
        } else {
            perft(depth - 1, game)
        };

        game.undo_move();

        perft_for_moves.push((*mv, number_for_mv));
    }

    perft_for_moves
}
