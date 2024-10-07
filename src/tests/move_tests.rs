use crate::chess::game::Game;
use crate::chess::moves::Move;
use crate::chess::square::squares::all::*;
use crate::engine::options::EngineOptions;
use crate::engine::search::time_control::TimeStrategy;
use crate::engine::search::{
    search, CapturingReporter, PersistentState, SearchRestrictions, SearchScore, TimeControl,
};

fn test_expected_move(fen: &str, depth: u8, mv: Move) -> (Move, SearchScore) {
    crate::init();
    let game = Game::from_fen(fen).unwrap();
    let mut persistent_state = PersistentState::new(16);

    let mut capturing_reporter = CapturingReporter::new();
    let (mut time_strategy, _) =
        TimeStrategy::new(&game, &TimeControl::Infinite, &EngineOptions::default());

    let best_move = search(
        &game,
        &mut persistent_state,
        &mut time_strategy,
        &SearchRestrictions { depth: Some(depth) },
        &EngineOptions::default(),
        &mut capturing_reporter,
    );

    assert_eq!(best_move, mv);
    (best_move, capturing_reporter.score.unwrap())
}

#[test]
fn test_mate_on_100th_halfmove_detected() {
    let (_, eval) = test_expected_move(
        "4Q3/8/1p4pk/1PbB1p1p/7P/p3P1PK/P3qP2/8 w - - 99 88",
        5,
        Move::new(E8, H8),
    );

    assert_eq!(eval, SearchScore::Mate(1));
}
