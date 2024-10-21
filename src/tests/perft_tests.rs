use crate::chess::fen::START_POS;
use crate::chess::game::Game;
use crate::chess::movegen;
use crate::chess::movegen::MovegenCache;
use crate::chess::moves::MoveList;
use crate::chess::perft::perft;
use crate::engine::search::move_picker::MovePicker;
use crate::engine::search::{PersistentState, SearchState};
use crate::engine::transposition_table::{TTOverwriteable, TranspositionTable};
use paste::paste;

fn test_perft(fen: &str, depth: u8, expected_positions: usize) {
    crate::init();
    let mut game = Game::from_fen(fen).unwrap();
    let actual_positions = perft(depth, &mut game);

    assert_eq!(expected_positions, actual_positions);
}

#[derive(Debug, Clone)]
struct PerftTranspositionTableData {
    nodes: usize,
    depth: u8,
}

impl TTOverwriteable for PerftTranspositionTableData {
    fn should_overwrite_with(&self, _: &Self) -> bool {
        false
    }
}

type PerftTranspositionTable = TranspositionTable<PerftTranspositionTableData>;

fn tt_perft(depth: u8, game: &mut Game, tt: &mut PerftTranspositionTable) -> usize {
    if depth == 1 {
        return game.moves().len();
    }

    if let Some(tt_data) = tt.get(&game.zobrist) {
        if tt_data.depth == depth {
            return tt_data.nodes;
        }
    }

    let result = game
        .moves()
        .to_vec()
        .into_iter()
        .map(|m| {
            game.make_move(m);
            let result = tt_perft(depth - 1, game, tt);
            game.undo_move();
            result
        })
        .sum();

    tt.insert(
        &game.zobrist,
        PerftTranspositionTableData {
            nodes: result,
            depth,
        },
    );

    result
}

fn test_perft_with_tt(fen: &str, depth: u8, expected_positions: usize) {
    crate::init();

    let mut tt = PerftTranspositionTable::new(256);

    let mut game = Game::from_fen(fen).unwrap();
    let actual_positions = tt_perft(depth, &mut game, &mut tt);

    assert_eq!(expected_positions, actual_positions);
}

fn movepicker_perft(
    depth: u8,
    game: &mut Game,
    persistent_state: &PersistentState,
    state: &mut SearchState,
) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut moves = 0;

    let test_individual_node_move_counts = true;
    let mut movegen_cache = MovegenCache::new();
    let mut captures_movelist = MoveList::new();
    let mut quiets_movelist = MoveList::new();

    let mut best_move = None;
    if test_individual_node_move_counts {
        movegen::generate_captures(game, &mut captures_movelist, &mut movegen_cache);
        movegen::generate_quiets(game, &mut quiets_movelist, &movegen_cache);

        // We want to make sure that we don't generate the best_move more than once, or any of the
        // killer moves more than once.
        //
        // There's no easy way to do that without specific scenarios to reproduce (which if found are
        // unit tests). But for refactoring there may be new scenarios which aren't captured in unit tests.
        //
        // To get good coverage, we use the length of the move list to determine whether to try captures/quiets
        // in best move/killers.
        if captures_movelist.len() >= 3 {
            best_move = Some(*captures_movelist.first().unwrap());
        } else if quiets_movelist.len() >= 3 {
            best_move = Some(*quiets_movelist.first().unwrap());
        }

        if quiets_movelist.len() >= 3 {
            state
                .killer_moves
                .try_push(depth, *quiets_movelist.get(2).unwrap());
        }

        if quiets_movelist.len() >= 4 {
            state
                .killer_moves
                .try_push(depth, *quiets_movelist.get(3).unwrap());
        }
    }

    let mut moves_at_this_node = Vec::new();
    let mut movepicker = MovePicker::new(best_move);
    while let Some(mv) = movepicker.next(game, persistent_state, state, depth) {
        game.make_move(mv);
        moves += movepicker_perft(depth - 1, game, persistent_state, state);
        game.undo_move();

        moves_at_this_node.push(mv);
    }

    if test_individual_node_move_counts {
        let legal_moves = game.moves().to_vec();

        if moves_at_this_node.len() < legal_moves.len() {
            let missing_moves = legal_moves
                .iter()
                .filter(|m| !moves_at_this_node.contains(m))
                .collect::<Vec<_>>();

            panic!("At fen {}\n{} legal moves, but only picked {}\nLegal moves: {:?}\nPicked moves: {:?}\nTT move: {:?}\nKiller moves: {:?} {:?}\nMissing moves: {:?}", game.to_fen(), legal_moves.len(), moves_at_this_node.len(), legal_moves, moves_at_this_node, best_move, state.killer_moves.get_0(depth), state.killer_moves.get_1(depth), missing_moves);
        }

        assert_eq!(moves_at_this_node.len(), legal_moves.len(),);
    }

    moves
}

fn test_perft_with_movepicker(fen: &str, depth: u8, expected_positions: usize) {
    crate::init();

    let persistent_state = PersistentState::new(16);
    let mut state = SearchState::new();

    let mut game = Game::from_fen(fen).unwrap();
    let actual_positions = movepicker_perft(depth, &mut game, &persistent_state, &mut state);

    assert_eq!(expected_positions, actual_positions);
}

macro_rules! perft_position {
    ($name:ident, $pos:expr, $depth:expr, $nodes:expr) => {
        paste! {
            #[test]
            fn [<perft_ $name>]() {
                test_perft($pos, $depth, $nodes);
            }

            #[test]
            fn [<perft_tt_ $name>]() {
                test_perft_with_tt($pos, $depth, $nodes);
            }

            #[test]
            #[ignore]
            fn [<perft_movepicker_ $name>]() {
                test_perft_with_movepicker($pos, $depth, $nodes);
            }
        }
    };
}

perft_position!(startpos_1, START_POS, 1, 20);
perft_position!(startpos_2, START_POS, 2, 400);
perft_position!(startpos_3, START_POS, 3, 8902);
perft_position!(startpos_4, START_POS, 4, 197_281);
perft_position!(startpos_5, START_POS, 5, 4_865_609);

const KIWIPETE_POS: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
perft_position!(kiwipete_1, KIWIPETE_POS, 1, 48);
perft_position!(kiwipete_2, KIWIPETE_POS, 2, 2039);
perft_position!(kiwipete_3, KIWIPETE_POS, 3, 97862);
perft_position!(kiwipete_4, KIWIPETE_POS, 4, 4_085_603);
perft_position!(kiwipete_5, KIWIPETE_POS, 5, 193_690_690);

const CHESSPROGRAMMING_WIKI_POS3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
perft_position!(chessprogramming_pos3_1, CHESSPROGRAMMING_WIKI_POS3, 1, 14);
perft_position!(chessprogramming_pos3_2, CHESSPROGRAMMING_WIKI_POS3, 2, 191);
perft_position!(chessprogramming_pos3_3, CHESSPROGRAMMING_WIKI_POS3, 3, 2812);
perft_position!(
    chessprogramming_pos3_4,
    CHESSPROGRAMMING_WIKI_POS3,
    4,
    43238
);
perft_position!(
    chessprogramming_pos3_5,
    CHESSPROGRAMMING_WIKI_POS3,
    5,
    674_624
);

const CHESSPROGRAMMING_WIKI_POS4: &str =
    "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
perft_position!(chessprogramming_pos4_1, CHESSPROGRAMMING_WIKI_POS4, 1, 6);
perft_position!(chessprogramming_pos4_2, CHESSPROGRAMMING_WIKI_POS4, 2, 264);
perft_position!(chessprogramming_pos4_3, CHESSPROGRAMMING_WIKI_POS4, 3, 9467);
perft_position!(
    chessprogramming_pos4_4,
    CHESSPROGRAMMING_WIKI_POS4,
    4,
    422_333
);
perft_position!(
    chessprogramming_pos4_5,
    CHESSPROGRAMMING_WIKI_POS4,
    5,
    15_833_292
);

const CHESSPROGRAMMING_WIKI_POS5: &str =
    "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
perft_position!(chessprogramming_pos5_1, CHESSPROGRAMMING_WIKI_POS5, 1, 44);
perft_position!(chessprogramming_pos5_2, CHESSPROGRAMMING_WIKI_POS5, 2, 1486);
perft_position!(
    chessprogramming_pos5_3,
    CHESSPROGRAMMING_WIKI_POS5,
    3,
    62379
);
perft_position!(
    chessprogramming_pos5_4,
    CHESSPROGRAMMING_WIKI_POS5,
    4,
    2_103_487
);
perft_position!(
    chessprogramming_pos5_5,
    CHESSPROGRAMMING_WIKI_POS5,
    5,
    89_941_194
);

// Extra perft tests positions taken from https://gist.github.com/peterellisjones/8c46c28141c162d1d8a0f0badbc9cff9
perft_position!(gist_1, "r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQ - 3 2", 1, 8);
perft_position!(gist_2, "8/8/8/2k5/2pP4/8/B7/4K3 b - d3 0 3", 1, 8);
perft_position!(
    gist_3,
    "r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 2 2",
    1,
    19
);
perft_position!(
    gist_4,
    "r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQkq - 3 2",
    1,
    5
);
perft_position!(
    gist_5,
    "2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQ - 3 2",
    1,
    44
);
perft_position!(
    gist_6,
    "rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R w KQ - 3 9",
    1,
    39
);
perft_position!(gist_7, "2r5/3pk3/8/2P5/8/2K5/8/8 w - - 5 4", 1, 9);
perft_position!(
    gist_8,
    "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
    3,
    62379
);
perft_position!(
    gist_9,
    "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
    3,
    89890
);
perft_position!(gist_10, "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1", 6, 1_134_888);
perft_position!(gist_11, "8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1", 6, 1_015_133);
perft_position!(gist_12, "8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1", 6, 1_440_467);
perft_position!(gist_13, "5k2/8/8/8/8/8/8/4K2R w K - 0 1", 6, 661_072);
perft_position!(gist_14, "3k4/8/8/8/8/8/8/R3K3 w Q - 0 1", 6, 803_711);
perft_position!(
    gist_15,
    "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1",
    4,
    1_274_206
);
perft_position!(
    gist_16,
    "r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1",
    4,
    1_720_476
);
perft_position!(gist_17, "2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1", 6, 3_821_001);
perft_position!(gist_18, "8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1", 5, 1_004_658);
perft_position!(gist_19, "4k3/1P6/8/8/8/8/K7/8 w - - 0 1", 6, 217_342);
perft_position!(gist_20, "8/P1k5/K7/8/8/8/8/8 w - - 0 1", 6, 92683);
perft_position!(gist_21, "K1k5/8/P7/8/8/8/8/8 w - - 0 1", 6, 2217);
perft_position!(gist_22, "8/k1P5/8/1K6/8/8/8/8 w - - 0 1", 7, 567_584);
perft_position!(gist_23, "8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1", 4, 23527);
