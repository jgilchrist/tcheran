use anyhow::Result;
use std::{
    collections::VecDeque,
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

mod log;
use log::log;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run an engine and proxy its stdin and stdout to a proxy
    Engine { cmd: String, port: u32 },

    /// Connect to an engine
    Proxy { port: u32 },
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}

pub fn run(cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Engine { cmd, port } => {
            eprintln!("hi");
            let address = format!("127.0.0.1:{}", port);
            let new_stream = TcpStream::connect(address)?;
            log(format!("Connection established: {new_stream:?}"));

            let mut reader_stream: BufReader<TcpStream> =
                BufReader::new(new_stream.try_clone().unwrap());
            let mut writer_stream: BufWriter<TcpStream> =
                BufWriter::new(new_stream.try_clone().unwrap());

            let mut command = Command::new(cmd)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;

            let handle_stdin = thread::spawn(move || {
                std::io::copy(&mut reader_stream, command.stdin.as_mut().unwrap())
            });

            let _handle_stdout = thread::spawn(move || {
                std::io::copy(command.stdout.as_mut().unwrap(), &mut writer_stream)
            });

            handle_stdin.join().unwrap()?;
            Ok(())
        }
        Commands::Proxy { port } => {
            eprintln!("hi2");
            let address = format!("127.0.0.1:{}", port);
            let listener = TcpListener::bind(address)?;
            eprintln!("Listening on port {}", port);

            let messages: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
            let connected: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

            // Copy messages from stdin to the messages list
            {
                let messages = Arc::clone(&messages);

                thread::spawn(move || {
                    let messages = messages;

                    let stdin_lock = std::io::stdin().lock();

                    for line in stdin_lock.lines() {
                        let line = line.unwrap();

                        let mut messages = messages.lock().unwrap();

                        messages.push_back(line);
                    }
                });
            }

            loop {
                let mut reader_stream: BufReader<TcpStream>;
                let mut writer_stream: BufWriter<TcpStream>;

                let new_stream = listener.incoming().next().unwrap().unwrap();

                log(format!("Connection established: {new_stream:?}"));

                reader_stream = BufReader::new(new_stream.try_clone().unwrap());
                writer_stream = BufWriter::new(new_stream.try_clone().unwrap());

                {
                    *connected.lock().unwrap() = true;
                }

                {
                    let messages = Arc::clone(&messages);
                    let connected = Arc::clone(&connected);

                    thread::spawn(move || {
                        let messages = Arc::clone(&messages);

                        loop {
                            {
                                if !*connected.lock().unwrap() {
                                    break;
                                }
                            }

                            {
                                let mut messages = messages.lock().unwrap();

                                if let Some(m) = messages.front() {
                                    log(format!("> {}", m.clone()));
                                    let result = writeln!(writer_stream, "{}", m);
                                    writer_stream.flush().unwrap();

                                    match result {
                                        Ok(_) => {
                                            messages.pop_front();
                                        }
                                        Err(e) => log(format!("{:?}", e)),
                                    }
                                }
                            }

                            thread::sleep(Duration::from_millis(50));
                        }

                        log("Stopped sending messages")
                    });
                }

                {
                    let connected = Arc::clone(&connected);

                    let stdout_handle = thread::spawn(move || {
                        let mut stdout_lock = std::io::stdout().lock();
                        std::io::copy(&mut reader_stream, &mut stdout_lock).unwrap();
                        *connected.lock().unwrap() = false;
                    });

                    stdout_handle.join().unwrap();
                    log("Connection broken. Waiting for a new connection.");
                }
            }
        }
    }
}

fn main() -> Result<()> {
    std::panic::set_hook(Box::new(|info| {
        println!("{info}");
        log(format!("{info:?}"));
    }));

    let args = parse_cli();
    run(args.command)
}
