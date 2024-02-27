use std::{fs, io::Write};

static mut ENABLE_LOGGING: bool = false;

pub fn set_logging_enabled(is_enabled: bool) {
    unsafe {
        ENABLE_LOGGING = is_enabled;
    }
}

pub fn crashlog<S: AsRef<str>>(s: S) {
    log_to_file(s, "crash.log");
}

pub fn log<S: AsRef<str>>(s: S) {
    if unsafe { !ENABLE_LOGGING } {
        return;
    }

    log_to_file(s, "log");
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
