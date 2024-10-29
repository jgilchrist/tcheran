use crate::chess::game::Game;
use crate::chess::moves::Move;

pub fn perft(depth: u8, game: &Game) -> usize {
    if depth == 1 {
        return game.moves().len();
    }

    game.moves()
        .iter()
        .map(|m| {
            let g = game.make_move(*m);
            let result = perft(depth - 1, &g);
            result
        })
        .sum()
}

pub fn perft_div(depth: u8, game: &Game) -> Vec<(Move, usize)> {
    let root_moves = game.moves().to_vec();

    let mut perft_for_moves: Vec<(Move, usize)> = vec![];

    if depth == 1 {
        for mv in root_moves {
            perft_for_moves.push((mv, 1));
        }

        return perft_for_moves;
    }

    for mv in root_moves {
        let g = game.make_move(mv);
        let number_for_mv = perft(depth - 1, &g);
        perft_for_moves.push((mv, number_for_mv));
    }

    perft_for_moves
}
