use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

pub struct Tcp {
    stream: Option<TcpStream>,
}

pub trait Protocol {
    fn new(host: String) -> Self;
    fn recv(&mut self, buf: &mut [u8]) -> io::Result<usize>;
    fn send(&mut self, buf: &[u8]) -> usize;
    fn set_nonblocking(&mut self, nonblocking: bool);
}

impl Protocol for Tcp {
    fn new(host: String) -> Self {
        match TcpStream::connect(host) {
            Ok(tcp) => {
                let mut ret = Self { stream: Some(tcp) };
                ret.set_nonblocking(true);

                ret
            }
            Err(e) => {
                eprintln!("{}:?", e);

                Self { stream: None }
            }
        }
    }

    fn recv(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.stream {
            Some(ref mut stream) => stream.read(buf),
            None => Ok(0),
        }
    }

    fn send(&mut self, buf: &[u8]) -> usize {
        if let Some(ref mut stream) = self.stream {
            if let Ok(num_bytes) = stream.write(buf) {
                return num_bytes
            }
        }

        return 0;
    }

    fn set_nonblocking(&mut self, nonblocking: bool) {
        if let Some(ref mut stream) = self.stream {
            if let Err(_) = stream.set_nonblocking(nonblocking) {
                eprintln!("Unable to set stream to non-blocking.")
            }
        }
    }
}
