use crate::chess::game::Game;
use crate::chess::movegen;
use crate::chess::moves::Move;

pub fn legal_perft(depth: u8, game: &mut Game) -> usize {
    if depth == 1 {
        return movegen::get_legal_moves(game).to_vec().len();
    }

    movegen::get_legal_moves(game)
        .to_vec()
        .into_iter()
        .map(|m| {
            game.make_move(m);
            let result = legal_perft(depth - 1, game);
            game.undo_move();
            result
        })
        .sum()
}

#[allow(unused)]
pub fn pseudo_legal_perft(depth: u8, game: &mut Game) -> usize {
    if depth == 0 {
        return 1;
    }

    movegen::get_pseudo_legal_moves(game)
        .to_vec()
        .into_iter()
        .filter_map(|m| {
            let player = game.player;
            game.make_move(m);

            if game.board.king_in_check(player) {
                game.undo_move();
                return None;
            }

            let result = pseudo_legal_perft(depth - 1, game);
            game.undo_move();
            Some(result)
        })
        .sum()
}

pub fn legal_perft_div(depth: u8, game: &mut Game) -> Vec<(Move, usize)> {
    let root_moves = movegen::get_legal_moves(game).to_vec();

    let mut perft_for_moves: Vec<(Move, usize)> = vec![];

    if depth == 1 {
        for mv in root_moves {
            perft_for_moves.push((mv, 1));
        }

        return perft_for_moves;
    }

    for mv in root_moves {
        game.make_move(mv);
        let number_for_mv = legal_perft(depth - 1, game);
        game.undo_move();
        perft_for_moves.push((mv, number_for_mv));
    }

    perft_for_moves
}
