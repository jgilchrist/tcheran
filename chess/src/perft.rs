use crate::game::Game;
use crate::moves::Move;

#[must_use]
pub fn perft(depth: u8, game: &mut Game) -> usize {
    if depth == 1 {
        return game.legal_moves().len();
    }

    game.legal_moves()
        .iter()
        .map(|m| {
            game.make_move(m);
            let result = perft(depth - 1, game);
            game.undo_move();

            result
        })
        .sum()
}

#[must_use]
pub fn perft_div(depth: u8, game: &mut Game) -> Vec<(Move, usize)> {
    let root_moves = game.legal_moves();
    let mut perft_for_moves: Vec<(Move, usize)> = vec![];

    for mv in root_moves {
        let number_for_mv = if depth == 1 {
            1
        } else {
            game.make_move(&mv);
            let result = perft(depth - 1, game);
            game.undo_move();
            result
        };

        perft_for_moves.push((mv, number_for_mv));
    }

    perft_for_moves
}
