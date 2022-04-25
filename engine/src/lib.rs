use chess::{debug, game::Game, moves::Move};

mod eval;
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
    let mut legal_moves = game.legal_moves();
    legal_moves.sort_unstable_by_key(|m| eval::eval(&game.make_move(m).unwrap()));

    let mv = *legal_moves.first().expect("Could not find a legal move");
    let next_game = game.make_move(&mv).unwrap();
    debug::log("eval", format!("{:?}", eval::eval(&next_game)));
    mv
}
