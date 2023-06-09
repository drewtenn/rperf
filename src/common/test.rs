use std::sync::mpsc;

use super::Message;
use super::protocol::Protocol;
use super::stream::Stream;

pub struct Test {
	pub control_channel: Option<Protocol>,
	pub stream: Option<Stream>,
	pub is_started: bool,
	pub is_running: bool,
	pub state: Message,
	pub rx_channel: std::sync::mpsc::Receiver<Message>,
	pub tx_channel: std::sync::mpsc::Sender<Message>
}

impl Test {
	pub fn new() -> Self {
		let (send, recv) = mpsc::channel();
		
		Self { 
			control_channel: None,
			stream: None,
			is_started: false,
			is_running: false,
			rx_channel: recv,
			tx_channel: send,
			state: Message::TestStart,
		 }
	}
}