use std::io::BufRead;

pub mod commands;
pub mod responses;

/// Implementation of the Universal Chess Interface (UCI) protocol

pub fn uci() {
    println!("Welcome!");
    println!("In UCI mode.");

    let stdin = std::io::stdin();

    // TODO: Parsing UCI commands
    // TODO: Responding to UCI commands
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}
