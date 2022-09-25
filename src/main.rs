fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut proxy;
    if args.len() == 5 {
	let src_ip = &args[1];
	let src_port = &args[2];
	let dst_ip = &args[3];
	let dst_port = &args[4];
	println!("Binding Proxy to {src_ip}:{src_port}");
	println!("Going to Forward packets to {dst_ip}:{dst_port}");
        proxy = udp_proxy::ProxyServer::new(src_ip, src_port, dst_ip, dst_port);
    }
    else{
	println!("No Command Line Args supplied, going to use the defaults");
	println!("Binding Proxy to 127.0.0.1:4444");
	println!("Going to Forward packets to 127.0.0.1:28763");
	proxy = udp_proxy::ProxyServer::new("127.0.0.1", "4444", "127.0.0.1", "28763");
    }

    println!("Starting to forward");
    proxy.start_forwarding();
}
