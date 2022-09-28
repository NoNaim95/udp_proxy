use std::net::{SocketAddr, SocketAddrV4, UdpSocket};
use std::str;

#[allow(dead_code)]

pub struct Packet<'a> {
    pub data: &'a [u8],
    sender: SocketAddrV4,
    receiver: SocketAddrV4,
}

impl<'a> Packet<'a> {
    fn new(data: &'a [u8], sender: SocketAddrV4, receiver: SocketAddrV4) -> Packet {
        let p = Packet {
            data,
            sender,
            receiver,
        };
        p
    }
}

pub struct ProxyServer<'a> {
    socket: UdpSocket,
    dest_addr: SocketAddrV4,
    client: Option<SocketAddrV4>,
    interceptor: &'a dyn Fn(&mut Packet),
}

impl<'a> ProxyServer<'a> {
    pub fn new(
        local_ip: &str,
        local_port: &str,
        dst_ip: &str,
        dst_port: &str,
        f: &'a dyn Fn(&mut Packet),
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
            if sender != self.dest_addr {
                self.client = Some(sender);
            }
            let receiver = if sender == self.dest_addr {
                self.client.unwrap()
            } else {
                self.dest_addr
            };
            let mut packet = Packet::new(&filled_buf, sender, receiver);
            (self.interceptor)(&mut packet);
            self.socket
                .send_to(filled_buf, receiver)
                .expect("Could not send bytes to Destination");
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn some_test() {}
}
