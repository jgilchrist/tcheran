use chess::{game::Game, movegen::generate_moves, r#move::Move};
use rand::prelude::SliceRandom;

pub mod uci;

pub fn engine_version() -> &'static str {
    // If we can't determine the version from git tags, fall back to the version
    // specified in the Cargo manifest
    let version = git_version::git_version!(cargo_prefix = "cargo:v");

    // If we used Cargo's manifest, adjust the format so it matches the Git version tag format
    if version.starts_with("cargo:") {
        return version
            .strip_prefix("cargo:")
            .unwrap()
            .strip_suffix(".0")
            .unwrap();
    }

    // Otherwise, if we got a version from Git, we can use it directly
    version
}

fn run(game: &Game) -> Move {
    let moves = generate_moves(game);

    let legal_moves: Vec<Move> = moves
        .into_iter()
        .filter(|m| {
            let next_state = game.make_move(m).unwrap();
            !next_state.king_in_check(&game.player)
        })
        .collect();

    *legal_moves.choose(&mut rand::thread_rng()).unwrap()
}
