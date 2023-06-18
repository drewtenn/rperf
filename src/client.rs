use crate::common::protocol::{Protocol, Tcp};
use crate::common::stream::Stream;
use crate::common::test::Test;
use crate::common::{Message, connect};

const HOST: &str = "10.1.10.3:5202";

pub fn run_client() {
    let mut test = Test::new();
    
    test.control_channel = Some(connect(String::from(HOST)));

    client_loop(&mut test);
}

fn client_recv(test: &mut Test) -> bool {
    let mut rx_buffer = [0u8; 1];

    if let Some(protocol) = &mut test.control_channel {
        if let Ok(num_bytes) = protocol.recv(&mut rx_buffer) {
            if num_bytes > 0 {
                if let Ok(message) = Message::try_from(rx_buffer[0] as i8) {
                    if handle_message_client(test, message) {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn client_loop(test: &mut Test) {   
    loop {
        if client_recv(test) {
            break;
        }

        if let Ok(message) = test.rx_channel.try_recv() {
            match message {
                Message::TestEnd => send_test_end(test),
                _ => eprintln!("Unknown message")
            }
        }
    }
}

fn handle_message_client(test: &mut Test, message: Message) -> bool {
    let mut is_done = false;

    match message {
        Message::ParamExchange => {
            println!("Received paramter exchange control message.");
            send_options(test);
        }

        Message::CreateStreams => {
            println!("Received create streams control message.");
            create_streams(test);
        }

        Message::TestStart => {
            test.is_started = true;
            println!("Received test start control message.")
        }

        Message::TestRunning => {
            test.is_running = true;
            println!("Received test running control message.")
        }

        Message::ExchangeResults => {
            println!("Received exchange results control message.");
            exchange_results(test);
        }

        Message::DisplayResults => {
            println!("Received display results control message.");
            send_iperf_done(test);
            is_done = true;
        }

        Message::IperfDone => {
            println!("Received done control message.");
            is_done = true;
        }

        _ => eprintln!("Received unknown control message."),
    }

    is_done
}

fn exchange_results(test: &mut Test) {
    let json = "{\"cpu_util_total\":0,\"cpu_util_user\":0,\"cpu_util_system\":0,\"sender_has_retransmits\":0,\"streams\":[{\"id\":1,\"bytes\":552730624,\"retransmits\":-1,\"jitter\":0,\"errors\":0,\"packets\":0}]}";

    let nsize = (json.len() as u32).to_be_bytes();

    if let Some(ref mut protocol) = test.control_channel {
        protocol.send(&nsize);
        protocol.send(json.as_bytes());

        println!("Results sent.")
    }

    let mut rx_buffer = [0u8; 4];

    if let Some(protocol) = &mut test.control_channel {
        protocol.set_nonblocking(false);

        if let Ok(num_bytes) = protocol.recv(&mut rx_buffer) {
            if num_bytes > 0 {
                let results_len = u32::from_be_bytes(rx_buffer);
                
                let mut results_data = vec![0; results_len as usize];
                
                if let Ok(_) = protocol.recv(&mut results_data) {
                    if let Ok(utf_str) = String::from_utf8(results_data) {
                        println!("{:?}", utf_str);
                    }
                }
            }
        }

        protocol.set_nonblocking(true);
    }
}

fn create_streams(test: &Test) {
    Stream::start::<Tcp>(test, HOST.to_string());
}

fn send_iperf_done(test: &mut Test) {
    if let Some(ref mut protocol) = test.control_channel {
        let byte:[u8; 1] = [Message::IperfDone as u8];

        protocol.send(&byte);
        
        println!("iPerf done sent.")
    }
}

fn send_test_end(test: &mut Test) {
    if let Some(ref mut protocol) = test.control_channel {
        let byte:[u8; 1] = [Message::TestEnd as u8];
        
        protocol.send(&byte);
        
        println!("Test end sent.")
    }
}

fn send_options(test: &mut Test) {
    let json = "{\"tcp\":true,\"omit\":0,\"time\":1,\"parallel\":1,\"len\":131072,\"client_version\":\"3.1.3\"}";

    let nsize = (json.len() as u32).to_be_bytes();

    if let Some(ref mut protocol) = test.control_channel {
        protocol.send(&nsize);
        protocol.send(json.as_bytes());

        println!("Options sent.")
    }
}
