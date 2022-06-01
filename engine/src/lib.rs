use chess::{debug, game::Game, moves::Move};

mod eval;
pub mod uci;
mod search;

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
    let best_move = search::search(game);

    let next_game = game.make_move(&best_move).unwrap();
    debug::log("eval", format!("{:?}", eval::eval(&next_game)));
    best_move
}
