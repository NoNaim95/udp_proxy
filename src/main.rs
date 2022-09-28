fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut proxy;

    let interceptor = |p: &mut udp_proxy::Packet| {
        println!("{:02X?}", p.data);
    };

    let src_ip: &str;
    let src_port: &str;
    let dst_ip: &str;
    let dst_port: &str;
    if args.len() == 5 {
        src_ip = &args[1];
        src_port = &args[2];
        dst_ip = &args[3];
        dst_port = &args[4];
        println!("Binding Proxy to {src_ip}:{src_port}");
        println!("Going to Forward packets to {dst_ip}:{dst_port}");
    } else {
        src_ip = "127.0.0.1";
        src_port = "4444";
        dst_ip = "127.0.0.1";
        dst_port = "28763";
        println!("No Command Line Args supplied, going to use the defaults");
        println!("Binding Proxy to 127.0.0.1:4444");
        println!("Going to Forward packets to 127.0.0.1:28763");
    }

    proxy = udp_proxy::ProxyServer::new(src_ip, src_port, dst_ip, dst_port, &interceptor);
    println!("Starting to forward");
    proxy.start_forwarding();
}
