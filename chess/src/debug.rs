use std::io::Write;

const LOG_PATH: &str = "/tmp/chess_engine";

// FIXME: It's not ideal to open a handle to the file every time we want to write a line
pub fn log(s: &str) {
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(LOG_PATH)
        .unwrap();

    writeln!(f, "{}", s).unwrap();
    f.flush().unwrap()
}
