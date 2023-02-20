use anyhow::Result;
use std::{
    collections::VecDeque,
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

mod log;
use log::log;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3001")?;
    eprintln!("Listening on port 3001");

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
