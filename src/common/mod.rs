use num_enum::TryFromPrimitive;

use self::protocol::Protocol;

pub mod protocol;
pub mod test;
pub mod timer;
pub mod stream;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(i8)]
pub enum Message {
    TestStart = 1,
    TestRunning = 2,
    ResultRequest = 3,
    TestEnd = 4,
    StreamBegin = 5,
    StreamRunning = 6,
    StreamEnd = 7,
    AllStreamsEnd = 8,
    ParamExchange = 9,
    CreateStreams = 10,
    ServerTerminate = 11,
    ClientTerminate = 12,
    ExchangeResults = 13,
    DisplayResults = 14,
    IperfStart = 15,
    IperfDone = 16,
    AccessDenied = -1,
    ServerError = -2,
}

pub fn connect(host: String) -> Option<Protocol> {
    match Protocol::new_tcp(host) {
        Some(mut protocol) => {
            send_cookie(&mut protocol);

            Some(protocol)
        }
        None => None,
    }
}

fn send_cookie(protocol: &mut Protocol) {
    let cookie = "Aj6ard5dsxid53kuwtvayyfi5mfe2g6jpxmq\0";
    protocol.transfer.send(cookie.as_bytes());
}