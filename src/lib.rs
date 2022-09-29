#[allow(dead_code)]
mod utils;

pub mod udp_proxy{
    use std::net::{SocketAddr, SocketAddrV4, UdpSocket};
    use std::str;

    #[derive(Clone)]
    pub struct Packet{
	pub data: Vec<u8>,
	sender: SocketAddrV4,
	receiver: SocketAddrV4,
    }

    pub struct ProxyServer<'a> {
	socket: UdpSocket,
	dest_addr: SocketAddrV4,
	client: Option<SocketAddrV4>,
	interceptor: &'a mut dyn FnMut(&mut Packet),
    }

    impl Packet{
	fn new(buf: Vec<u8>, sender: SocketAddrV4, receiver: SocketAddrV4) -> Packet {
	    let p = Packet {
		data: buf,
		sender,
		receiver,
	    };
	    p
	}
    }


    impl<'a> ProxyServer<'a> {
	pub fn new(
	    local_ip: &str,
	    local_port: &str,
	    dst_ip: &str,
	    dst_port: &str,
	    f: &'a mut dyn FnMut(&mut Packet),
	) -> ProxyServer<'a> {
	    let proxy = ProxyServer {
		socket: UdpSocket::bind(std::format!("{local_ip}:{local_port}"))
		    .expect("couldn't bind to address"),
		dest_addr: format!("{dst_ip}:{dst_port}")
		    .parse()
		    .expect("Given Destination Ip Address could not be parsed"),
		client: None,
		interceptor: f,
	    };
	    proxy
	}

	pub fn start_forwarding(&mut self) {
	    loop {
		let mut buf = vec![0; 4096];
		let (number_of_bytes_read, src_addr) = self
		    .socket
		    .recv_from(&mut buf)
		    .expect("Did not receive any Data");
		buf.resize(number_of_bytes_read, 0);
		let sender = match src_addr {
		    SocketAddr::V4(addr) => addr,
		    SocketAddr::V6(_) => panic!("IpV6 is not supported yet"),
		};
		if sender != self.dest_addr {
		    self.client = Some(sender);
		}
		let receiver = if sender == self.dest_addr {
		    self.client.unwrap()
		} else {
		    self.dest_addr
		};
		let mut packet = Packet::new(buf, sender, receiver);
		(self.interceptor)(&mut packet);
		self.socket
		    .send_to(&packet.data, packet.receiver)
		    .expect("Could not send bytes to Destination");
	    }
	}

	pub fn get_dest(&self) -> SocketAddrV4{
	    self.dest_addr
	}
	pub fn get_client(&self) -> Option<SocketAddrV4>{
	    self.client
	}
    }


}



#[cfg(test)]
mod tests {
    #[test]
    fn some_test() {}
}
