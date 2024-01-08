use crate::chess::game::Game;
use crate::chess::moves::Move;
use crate::chess::player::Player;
use crate::chess::square::squares::all::*;
use crate::engine::eval::{Eval, WhiteEval};
use crate::engine::options::EngineOptions;
use crate::engine::search::transposition::SearchTranspositionTable;
use crate::engine::search::{search, NullControl, NullReporter, SearchRestrictions, TimeControl};

fn test_expected_move(fen: &str, depth: u8, mv: Move) -> (Move, WhiteEval) {
    crate::init();
    let mut game = Game::from_fen(fen).unwrap();

    let mut tt = SearchTranspositionTable::new();
    tt.resize(128);

    let (best_move, eval) = search(
        &mut game,
        &mut tt,
        &TimeControl::Infinite,
        &SearchRestrictions { depth: Some(depth) },
        &EngineOptions::default(),
        &NullControl,
        &NullReporter,
    );

    assert_eq!(best_move, mv);
    (best_move, eval)
}

#[test]
fn test_mate_on_100th_halfmove_detected() {
    let (_, eval) = test_expected_move(
        "4Q3/8/1p4pk/1PbB1p1p/7P/p3P1PK/P3qP2/8 w - - 99 88",
        5,
        Move::new(E8, H8),
    );

    assert_eq!(eval, Eval::mate_in(1).to_white_eval(Player::White));
}
