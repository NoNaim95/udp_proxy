#[allow(dead_code)]
mod inspector {
    use std::net::SocketAddrV4;
    use udp_proxy::udp_proxy::Packet;
    pub struct Inspector {
        client: Option<SocketAddrV4>,
        dst: Option<SocketAddrV4>,
        sent_packets: Vec<Packet>,
        number_of_intercepted_packets: u32,
    }
    impl Inspector {
        pub fn new(client: Option<SocketAddrV4>, dst: Option<SocketAddrV4>) -> Inspector {
            let i = Inspector {
                client,
                dst,
                sent_packets: vec![],
                number_of_intercepted_packets: 0,
            };
            i
        }

        pub fn on_packet(&mut self, packet: &mut Packet) {
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

use crate::inspector::Inspector;
use udp_proxy::udp_proxy::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut proxy;

    let (src_ip, src_port, dst_ip, dst_port): (&str, &str, &str, &str);
    if args.len() == 5 {
        (src_ip, src_port, dst_ip, dst_port) = (&args[1], &args[2], &args[3], &args[4]);
        println!("Binding Proxy to {src_ip}:{src_port}");
        println!("Going to Forward packets to {dst_ip}:{dst_port}");
    } else {
        (src_ip, src_port, dst_ip, dst_port) = ("127.0.0.1", "4444", "127.0.0.1", "28763");
        println!("No Command Line Args supplied, going to use the defaults");
        println!("Binding Proxy to 127.0.0.1:4444");
        println!("Going to Forward packets to 127.0.0.1:28763");
    }

    let mut inspector = Inspector::new(None, None);
    let mut interceptor = |p: &mut Packet| {
        inspector.on_packet(p);
    };

    proxy = ProxyServer::new(src_ip, src_port, dst_ip, dst_port, &mut interceptor);
    println!("Start forwarding");
    proxy.start_forwarding();
}
