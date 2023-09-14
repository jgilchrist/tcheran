use std::{fs, io::Write, path::Path};

const LOG_DIRECTORY: &str = "/tmp/chess_engine";

#[must_use]
pub fn log_file() -> String {
    "log".to_string()
}

// FIXME: It's not ideal to open a handle to the file every time we want to write a line
pub fn log<S: AsRef<str>>(s: S) {
    fs::create_dir_all(LOG_DIRECTORY).expect("Unable to create log directory");
    let path = Path::new(LOG_DIRECTORY).join(log_file());

    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    writeln!(f, "[{}] {}", std::process::id(), s.as_ref()).unwrap();
    f.flush().unwrap();
}
