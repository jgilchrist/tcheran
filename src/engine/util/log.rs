use std::{fs, io::Write};

pub fn crashlog<S: AsRef<str>>(s: S) {
    log_to_file(s, "crash.log");
}

// FIXME: It's not ideal to open a handle to the file every time we want to write a line
fn log_to_file<S: AsRef<str>>(s: S, extension: &str) {
    let current_exe =
        std::env::current_exe().expect("Unable to determine current executable directory");

    let path = current_exe.with_extension(extension);

    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    writeln!(f, "[{}] {}", std::process::id(), s.as_ref()).unwrap();
    f.flush().unwrap();
}
