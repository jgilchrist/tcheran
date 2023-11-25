use std::{fs, io::Write};

pub fn log_file() -> String {
    "log".to_string()
}

// FIXME: It's not ideal to open a handle to the file every time we want to write a line
pub fn log<S: AsRef<str>>(s: S) {
    let current_exe =
        std::env::current_exe().expect("Unable to determine current executable directory");

    let path = current_exe.with_extension("log");

    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    writeln!(f, "[{}] {}", std::process::id(), s.as_ref()).unwrap();
    f.flush().unwrap();
}
