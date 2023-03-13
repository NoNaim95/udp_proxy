#[allow(dead_code)]
use std::net::{SocketAddr, SocketAddrV4, UdpSocket};

pub mod packet;
use packet::Packet;

pub struct ProxyServer {
    socket: UdpSocket,
    dest_addr: SocketAddrV4,
    client: SocketAddrV4,
}

impl ProxyServer {
    pub fn new(socket: UdpSocket, dest_addr: SocketAddrV4, client: SocketAddrV4) -> Self {
        ProxyServer {
            socket,
            dest_addr,
            client,
        }
    }

    pub fn get_client(port: u16) -> (UdpSocket, SocketAddrV4) {
        let mut buf = [];
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", port)).unwrap();
        let (_, src_addr) = socket
            .recv_from(&mut buf)
            .expect("Did not receive any Data");
        (
            socket,
            match src_addr {
                SocketAddr::V4(client_socket) => client_socket,
                SocketAddr::V6(_) => panic!("IPV6 not supported yet"),
            },
        )
    }

    pub fn run(self, mut f: impl FnMut(Packet)) -> ! {
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
            let receiver = if sender == self.dest_addr {
                self.client
            } else {
                self.dest_addr
            };
            let packet = Packet::new(buf, sender, receiver);
            f(packet.clone());
            self.socket
                .send_to(&packet.data, packet.receiver)
                .expect("Could not send bytes to Destination");
        }
    }
}
