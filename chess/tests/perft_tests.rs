use chess::game::Game;
use chess::perft::perft;

fn test_perft(game: &mut Game, depth: u8, expected_positions: usize) {
    chess::init();
    let actual_positions = perft(depth, game);

    assert_eq!(expected_positions, actual_positions);
}

#[test]
#[ignore]
fn perft_startpos_1() {
    test_perft(&mut Game::new(), 1, 20);
}

#[test]
#[ignore]
fn perft_startpos_2() {
    test_perft(&mut Game::new(), 2, 400);
}

#[test]
#[ignore]
fn perft_startpos_3() {
    test_perft(&mut Game::new(), 3, 8902);
}

#[test]
#[ignore]
fn perft_startpos_4() {
    test_perft(&mut Game::new(), 4, 197281);
}

#[test]
#[ignore]
fn perft_startpos_5() {
    test_perft(&mut Game::new(), 5, 4865609);
}

fn kiwipete_game() -> Game {
    Game::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap()
}

#[test]
#[ignore]
fn perft_kiwipete_1() {
    test_perft(&mut kiwipete_game(), 1, 48);
}

#[test]
#[ignore]
fn perft_kiwipete_2() {
    test_perft(&mut kiwipete_game(), 2, 2039);
}

#[test]
#[ignore]
fn perft_kiwipete_3() {
    test_perft(&mut kiwipete_game(), 3, 97862);
}

#[test]
#[ignore]
fn perft_kiwipete_4() {
    test_perft(&mut kiwipete_game(), 4, 4085603);
}

#[test]
#[ignore]
fn perft_kiwipete_5() {
    test_perft(&mut kiwipete_game(), 5, 193690690);
}

fn chessprogramming_wiki_pos3() -> Game {
    // From https://www.chessprogramming.org/Perft_Results
    Game::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap()
}

#[test]
#[ignore]
fn perft_chessprogramming_pos3_1() {
    test_perft(&mut chessprogramming_wiki_pos3(), 1, 14);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos3_2() {
    test_perft(&mut chessprogramming_wiki_pos3(), 2, 191);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos3_3() {
    test_perft(&mut chessprogramming_wiki_pos3(), 3, 2812);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos3_4() {
    test_perft(&mut chessprogramming_wiki_pos3(), 4, 43238);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos3_5() {
    test_perft(&mut chessprogramming_wiki_pos3(), 5, 674624);
}

fn chessprogramming_wiki_pos4() -> Game {
    // From https://www.chessprogramming.org/Perft_Results
    Game::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap()
}

#[test]
#[ignore]
fn perft_chessprogramming_pos4_1() {
    test_perft(&mut chessprogramming_wiki_pos4(), 1, 6);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos4_2() {
    test_perft(&mut chessprogramming_wiki_pos4(), 2, 264);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos4_3() {
    test_perft(&mut chessprogramming_wiki_pos4(), 3, 9467);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos4_4() {
    test_perft(&mut chessprogramming_wiki_pos4(), 4, 422333);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos4_5() {
    test_perft(&mut chessprogramming_wiki_pos4(), 5, 15833292);
}

fn chessprogramming_wiki_pos5() -> Game {
    // From https://www.chessprogramming.org/Perft_Results
    Game::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap()
}

#[test]
#[ignore]
fn perft_chessprogramming_pos5_1() {
    test_perft(&mut chessprogramming_wiki_pos5(), 1, 44);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos5_2() {
    test_perft(&mut chessprogramming_wiki_pos5(), 2, 1486);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos5_3() {
    test_perft(&mut chessprogramming_wiki_pos5(), 3, 62379);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos5_4() {
    test_perft(&mut chessprogramming_wiki_pos5(), 4, 2103487);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos5_5() {
    test_perft(&mut chessprogramming_wiki_pos5(), 5, 89941194);
}
