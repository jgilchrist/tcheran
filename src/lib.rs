pub mod chess;
pub mod engine;

#[cfg(not(feature = "release"))]
pub mod utils;

#[cfg(test)]
pub mod tests;

pub const ENGINE_NAME: &str = "Tcheran";

#[cfg(all(feature = "default", feature = "release"))]
compile_error!("features \"default\" and \"release\" cannot be enabled simultaneously");

pub fn engine_version() -> String {
    let cargo_version = env!("CARGO_PKG_VERSION");
    let version = cargo_version.strip_suffix(".0").unwrap();
    let dev_suffix = if cfg!(feature = "release") {
        ""
    } else {
        "-dev"
    };

    format!("v{version}{dev_suffix}")
}

pub fn init() {
    chess::init();
    engine::init();
}
