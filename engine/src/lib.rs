pub mod eval;
pub mod options;
pub mod strategy;
pub mod uci;
pub mod util;

pub mod game;
pub mod search;
pub mod transposition;

pub use eval::eval;

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

pub fn init() {
    chess::init();

    eval::init();
}
