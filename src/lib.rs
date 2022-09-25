use std::net::{SocketAddr, SocketAddrV4, UdpSocket};
use std::str;


pub struct ProxyServer {
    socket: UdpSocket,
    dest_ip: SocketAddrV4,
    clients: Vec<SocketAddrV4>,
}

impl ProxyServer {
    pub fn new(local_ip: &str, local_port: &str, dst_ip: &str, dst_port: &str) -> ProxyServer {
        let proxy = ProxyServer {
            socket: UdpSocket::bind(std::format!("{local_ip}:{local_port}"))
                .expect("couldn't bind to address"),
            dest_ip: format!("{dst_ip}:{dst_port}")
                .parse()
                .expect("Given Destination Ip Address could not be parsed"),
            clients: Vec::new(),
        };
        proxy
    }

    pub fn start_forwarding(&mut self) {
        loop {
            let mut buf = [0; 4096];
            let (number_of_bytes_read, src_addr) = self
                .socket
                .recv_from(&mut buf)
                .expect("Did not receive any Data");
            let filled_buf = &mut buf[..number_of_bytes_read];
            let sender = match src_addr {
                SocketAddr::V4(addr) => addr,
                SocketAddr::V6(_) => panic!("IpV6 is not supported yet"),
            };
            if !self.clients.contains(&sender) && sender != self.dest_ip {
                self.clients.push(sender);
            }
	    let receiver = if sender == self.dest_ip{ self.clients[0]} else {self.dest_ip};
            self.socket
                .send_to(filled_buf, receiver)
                .expect("Could not send bytes to Destination");
            println!("{:02X?}", filled_buf);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn some_test() {}
}
