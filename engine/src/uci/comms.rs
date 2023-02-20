use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
};

use anyhow::Result;

pub trait UciComms {
    fn lines(&self) -> impl Iterator<Item = String>;

    fn send(&mut self, s: &str) -> Result<()>;
}

pub struct LocalComms {}

impl UciComms for LocalComms {
    fn lines(&self) -> impl Iterator<Item = String> {
        std::io::stdin().lock().lines().map(Result::unwrap)
    }

    fn send(&mut self, s: &str) -> Result<()> {
        println!("{s}");
        Ok(())
    }
}

pub struct RemoteComms {
    pub stream: TcpStream,
    pub writer: BufWriter<TcpStream>,
}

impl UciComms for RemoteComms {
    fn lines(&self) -> impl Iterator<Item = String> {
        BufReader::new(self.stream.try_clone().unwrap())
            .lines()
            .map(Result::unwrap)
    }

    fn send(&mut self, s: &str) -> Result<()> {
        writeln!(self.writer, "{s}").map_err(anyhow::Error::from)?;
        self.writer.flush().map_err(anyhow::Error::from)?;
        Ok(())
    }
}
