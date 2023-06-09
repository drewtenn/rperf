use std::{net::TcpStream, net::UdpSocket, io::{Write, Read, self}};

pub struct Protocol {
	pub transfer : Box<dyn Socket>,
}

impl Protocol {
	pub fn new_tcp(host: String) -> Option<Self> {
		match TcpStream::connect(host) {
			Ok(tcp) => { 
				tcp.set_nonblocking(true);
				
				Some(Self { transfer : Box::new(Tcp::new(tcp)) })
			},
			Err(e) => { eprintln!("{}:?", e); None }
		}
	}

	pub fn new_udp(host: String) -> Option<Self> {
		match UdpSocket::bind(host) {
			Ok(udp) => { Some(Self { transfer : Box::new(Udp::new(udp)) })},
			Err(e) => { eprintln!("{}:?", e); None }
		}
	}
}

pub struct Tcp {
	stream: TcpStream
}

pub struct Udp {
	socket: UdpSocket
}

impl Tcp {
	pub fn new(stream: TcpStream) -> Self {
		Self { stream }
	 }
}

impl Udp {
	pub fn new(socket: UdpSocket) -> Self {
		Self { socket }
	 }
}

pub trait Socket {
	fn recv(&mut self, buf: &mut[u8]) -> io::Result<usize>;
	fn send(&mut self, buf: &[u8]) -> io::Result<usize>;
	fn set_nonblocking(&mut self, nonblocking: bool);
}

impl Socket for Tcp {
	fn recv(&mut self, buf: &mut[u8]) -> io::Result<usize> { self.stream.read(buf) }
	fn send(&mut self, buf: &[u8]) -> io::Result<usize> { self.stream.write(buf) }
	fn set_nonblocking(&mut self, nonblocking: bool) { self.stream.set_nonblocking(nonblocking); }
}

impl Socket for Udp {
	fn recv(&mut self, buf: &mut[u8]) -> io::Result<usize> { Ok(self.socket.recv_from(buf).expect("Failed to receive data.").0) }
	fn send(&mut self, buf: &[u8]) -> io::Result<usize> { self.socket.send(buf) }
	fn set_nonblocking(&mut self, nonblocking: bool) { self.socket.set_nonblocking(nonblocking); }
}
