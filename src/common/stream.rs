use std::{
    thread::{self},
    time::Instant, sync::{Arc, Mutex},
};

use timer::Timer;

use super::{connect, protocol::Protocol, test::Test, Message};

pub struct Stream;

impl Stream {
    pub fn start<T>(test: &Test, host: String)
    where
        T: Protocol
    {
        let tx = test.tx_channel.clone();

        thread::spawn(move || {
            let now = Instant::now();
            let tx_buffer = vec![1; 100000].into_boxed_slice();
            let mut stream: T = connect(host);

            let total_bytes = Arc::new(Mutex::new(0));
            let tb = total_bytes.clone();

            let timer = Timer::new();
            let _guard = timer.schedule_repeating(chrono::Duration::milliseconds(1000), move|| {
                println!("{}", *tb.lock().unwrap());
            });
    
            loop {
                *total_bytes.lock().unwrap() += stream.send(&tx_buffer);

                if now.elapsed().as_millis() >= 5000 {
                    if let Err(e) = tx.send(Message::TestEnd) {
                        eprintln!("{:?}", e)
                    }
                    break;
                }
            }
        });
    }
}
