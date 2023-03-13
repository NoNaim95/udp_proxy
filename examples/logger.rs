#[allow(dead_code)]

use udp_proxy::*;
use udp_proxy::packet::*;

mod inspector {
    use udp_proxy::packet::Packet;

    pub struct Inspector {
        sent_packets: Vec<Packet>,
        number_of_intercepted_packets: u32,
    }
    impl Inspector {
        pub fn new() -> Inspector {
            let i = Inspector {
                sent_packets: vec![],
                number_of_intercepted_packets: 0,
            };
            i
        }

        pub fn on_packet(&mut self, packet: Packet) {
            self.number_of_intercepted_packets += 1;
            self.sent_packets.push(packet.clone());
            if self.number_of_intercepted_packets == 30 {
                for packet in &self.sent_packets {
                    println!("{:02X?}", packet.data);
                }
            }
        }
    }
}
use inspector::Inspector;

fn main() {
    let mut inspector = Inspector::new();
    let interceptor = move |p: Packet| {
        inspector.on_packet(p);
    };

    let (socket, addr) = ProxyServer::get_client(4444);

    let proxy = ProxyServer::new(socket, "103.172.92.234:28763".parse().unwrap(), addr);
    println!("Start forwarding");
    proxy.run(interceptor);
}
