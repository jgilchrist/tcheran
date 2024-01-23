use crate::chess::fen::START_POS;
use crate::chess::game::Game;
use crate::chess::perft::{legal_perft, pseudo_legal_perft};

fn test_perft(fen: &str, depth: u8, expected_positions: usize) {
    crate::init();
    let mut game = Game::from_fen(fen).unwrap();
    let actual_positions_legal = legal_perft(depth, &mut game);
    let actual_positions_pseudo_legal = pseudo_legal_perft(depth, &mut game);

    assert_eq!(expected_positions, actual_positions_legal);
    assert_eq!(expected_positions, actual_positions_pseudo_legal);
}

#[test]
#[ignore]
fn perft_startpos_1() {
    test_perft(START_POS, 1, 20);
}

#[test]
#[ignore]
fn perft_startpos_2() {
    test_perft(START_POS, 2, 400);
}

#[test]
#[ignore]
fn perft_startpos_3() {
    test_perft(START_POS, 3, 8902);
}

#[test]
#[ignore]
fn perft_startpos_4() {
    test_perft(START_POS, 4, 197_281);
}

#[test]
#[ignore]
fn perft_startpos_5() {
    test_perft(START_POS, 5, 4_865_609);
}

const KIWIPETE_POS: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

#[test]
#[ignore]
fn perft_kiwipete_1() {
    test_perft(KIWIPETE_POS, 1, 48);
}

#[test]
#[ignore]
fn perft_kiwipete_2() {
    test_perft(KIWIPETE_POS, 2, 2039);
}

#[test]
#[ignore]
fn perft_kiwipete_3() {
    test_perft(KIWIPETE_POS, 3, 97862);
}

#[test]
#[ignore]
fn perft_kiwipete_4() {
    test_perft(KIWIPETE_POS, 4, 4_085_603);
}

#[test]
#[ignore]
fn perft_kiwipete_5() {
    test_perft(KIWIPETE_POS, 5, 193_690_690);
}

const CHESSPROGRAMMING_WIKI_POS3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";

#[test]
#[ignore]
fn perft_chessprogramming_pos3_1() {
    test_perft(CHESSPROGRAMMING_WIKI_POS3, 1, 14);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos3_2() {
    test_perft(CHESSPROGRAMMING_WIKI_POS3, 2, 191);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos3_3() {
    test_perft(CHESSPROGRAMMING_WIKI_POS3, 3, 2812);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos3_4() {
    test_perft(CHESSPROGRAMMING_WIKI_POS3, 4, 43238);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos3_5() {
    test_perft(CHESSPROGRAMMING_WIKI_POS3, 5, 674_624);
}

const CHESSPROGRAMMING_WIKI_POS4: &str =
    "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";

#[test]
#[ignore]
fn perft_chessprogramming_pos4_1() {
    test_perft(CHESSPROGRAMMING_WIKI_POS4, 1, 6);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos4_2() {
    test_perft(CHESSPROGRAMMING_WIKI_POS4, 2, 264);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos4_3() {
    test_perft(CHESSPROGRAMMING_WIKI_POS4, 3, 9467);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos4_4() {
    test_perft(CHESSPROGRAMMING_WIKI_POS4, 4, 422_333);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos4_5() {
    test_perft(CHESSPROGRAMMING_WIKI_POS4, 5, 15_833_292);
}

const CHESSPROGRAMMING_WIKI_POS5: &str =
    "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";

#[test]
#[ignore]
fn perft_chessprogramming_pos5_1() {
    test_perft(CHESSPROGRAMMING_WIKI_POS5, 1, 44);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos5_2() {
    test_perft(CHESSPROGRAMMING_WIKI_POS5, 2, 1486);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos5_3() {
    test_perft(CHESSPROGRAMMING_WIKI_POS5, 3, 62379);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos5_4() {
    test_perft(CHESSPROGRAMMING_WIKI_POS5, 4, 2_103_487);
}

#[test]
#[ignore]
fn perft_chessprogramming_pos5_5() {
    test_perft(CHESSPROGRAMMING_WIKI_POS5, 5, 89_941_194);
}

// Extra perft tests positions taken from https://gist.github.com/peterellisjones/8c46c28141c162d1d8a0f0badbc9cff9

#[test]
#[ignore]
fn perft_gist_1() {
    test_perft("r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQ - 3 2", 1, 8);
}

#[test]
#[ignore]
fn perft_gist_2() {
    test_perft("8/8/8/2k5/2pP4/8/B7/4K3 b - d3 0 3", 1, 8);
}

#[test]
#[ignore]
fn perft_gist_3() {
    test_perft(
        "r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 2 2",
        1,
        19,
    );
}

#[test]
#[ignore]
fn perft_gist_4() {
    test_perft(
        "r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQkq - 3 2",
        1,
        5,
    );
}

#[test]
#[ignore]
fn perft_gist_5() {
    test_perft(
        "2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQ - 3 2",
        1,
        44,
    );
}

#[test]
#[ignore]
fn perft_gist_6() {
    test_perft(
        "rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R w KQ - 3 9",
        1,
        39,
    );
}

#[test]
#[ignore]
fn perft_gist_7() {
    test_perft("2r5/3pk3/8/2P5/8/2K5/8/8 w - - 5 4", 1, 9);
}

#[test]
#[ignore]
fn perft_gist_8() {
    test_perft(
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        3,
        62379,
    );
}

#[test]
#[ignore]
fn perft_gist_9() {
    test_perft(
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        3,
        89890,
    );
}

#[test]
#[ignore]
fn perft_gist_10() {
    test_perft("3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1", 6, 1_134_888);
}

#[test]
#[ignore]
fn perft_gist_11() {
    test_perft("8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1", 6, 1_015_133);
}

#[test]
#[ignore]
fn perft_gist_12() {
    test_perft("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1", 6, 1_440_467);
}

#[test]
#[ignore]
fn perft_gist_13() {
    test_perft("5k2/8/8/8/8/8/8/4K2R w K - 0 1", 6, 661_072);
}

#[test]
#[ignore]
fn perft_gist_14() {
    test_perft("3k4/8/8/8/8/8/8/R3K3 w Q - 0 1", 6, 803_711);
}

#[test]
#[ignore]
fn perft_gist_15() {
    test_perft("r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1", 4, 1_274_206);
}

#[test]
#[ignore]
fn perft_gist_16() {
    test_perft("r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1", 4, 1_720_476);
}

#[test]
#[ignore]
fn perft_gist_17() {
    test_perft("2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1", 6, 3_821_001);
}

#[test]
#[ignore]
fn perft_gist_18() {
    test_perft("8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1", 5, 1_004_658);
}

#[test]
#[ignore]
fn perft_gist_19() {
    test_perft("4k3/1P6/8/8/8/8/K7/8 w - - 0 1", 6, 217_342);
}

#[test]
#[ignore]
fn perft_gist_20() {
    test_perft("8/P1k5/K7/8/8/8/8/8 w - - 0 1", 6, 92683);
}

#[test]
#[ignore]
fn perft_gist_21() {
    test_perft("K1k5/8/P7/8/8/8/8/8 w - - 0 1", 6, 2217);
}

#[test]
#[ignore]
fn perft_gist_22() {
    test_perft("8/k1P5/8/1K6/8/8/8/8 w - - 0 1", 7, 567_584);
}

#[test]
#[ignore]
fn perft_gist_23() {
    test_perft("8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1", 4, 23527);
}
