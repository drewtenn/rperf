use std::{thread::{self}, time::Instant};

use super::{protocol::Protocol, connect, test::Test, Message};

#[derive(Default)]
pub struct Stream {
    pub protocol: Option<Protocol>,
}

impl Stream {
    pub fn new() -> Self {
        Self { protocol: None }
    }

    pub fn start(test: &Test, host: String) {
        let tx = test.tx_channel.clone();

        thread::spawn(move || {
            let mut stream = Stream::new();

            if let Some(protocol) = connect(host) {
                stream.protocol = Some(protocol);

                let now = Instant::now();

                let tx_buffer = vec![1; 100_000_000].into_boxed_slice();

                loop {
                    stream.send_data(&tx_buffer);

                    if now.elapsed().as_millis() >= 5000 {
                        tx.send(Message::TestEnd);
                        break;
                    }
                }
            }
        });
    }

    pub fn send_data(&mut self, buf: &[u8]) {
        if let Some(ref mut protocol) = self.protocol {

            if let Ok(bytes) = protocol.transfer.send(buf) {
                //println!("{} bytes sent", bytes);
            }
        }
    }
}
