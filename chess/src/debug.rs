use std::{fs, io::Write, path::Path};

const LOG_DIR: &str = "/tmp/chess_engine";

// FIXME: It's not ideal to open a handle to the file every time we want to write a line
pub fn log<S: AsRef<str>>(file: &str, s: S) {
    fs::create_dir_all(LOG_DIR).expect("Unable to create log directory");
    let path = Path::new(LOG_DIR).join(file);

    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    writeln!(f, "{}", s.as_ref()).unwrap();
    f.flush().unwrap()
}
