use chess::{game::Game, moves::Move};

pub fn order_moves(game: &Game, moves: &mut [Move]) {
    moves.sort_unstable_by_key(|mv| !is_capture(game, *mv));
}

fn is_capture(game: &Game, mv: Move) -> bool {
    game.board.piece_at(mv.dst).is_some()
}