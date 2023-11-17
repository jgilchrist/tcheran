use chess::game::Game;
use chess::moves::Move;
use chess::player::Player;
use chess::square::squares::all::*;
use engine::eval::Eval;
use engine::options::EngineOptions;
use engine::search::{search, NegamaxEval};
use engine::strategy::{NullControl, NullReporter, SearchRestrictions, TimeControl};
use engine::transposition::transposition_table::SearchTranspositionTable;

fn test_expected_move(fen: &str, depth: u8, mv: Move) -> (Move, Eval) {
    engine::init();
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

    assert_eq!(eval, NegamaxEval::mate_in(1).to_eval(Player::White));
}
