use std::net::SocketAddrV4;

#[derive(Clone)]
pub struct Packet {
    pub data: Vec<u8>,
    pub sender: SocketAddrV4,
    pub receiver: SocketAddrV4,
}

impl Packet {
    pub fn new(buf: Vec<u8>, sender: SocketAddrV4, receiver: SocketAddrV4) -> Packet {
        let p = Packet {
            data: buf,
            sender,
            receiver,
        };
        p
    }
}

