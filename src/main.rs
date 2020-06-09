use std::env;
use std::net::{SocketAddr, UdpSocket};

include!("wol.rs");

fn send_wol_package() {
    let sync_stream = [0xFF; 6];
    let dst_mac = [0x00, 0x90, 0x27, 0x85, 0xcf, 0x01];

    // Create Magic packet
    let mut vector: Vec<u8> = Vec::new();
    vector.extend(sync_stream.iter().copied());
    for _i in 0..16 {
        vector.extend(dst_mac.iter().copied());
    }
    let buffer = vector.as_slice();

    // Create a bind socket
    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    socket
        .set_broadcast(true)
        .expect("Unable to set broadcast flag");

    // Create destination server address
    const UDP_PORT: u16 = 9;
    let dst_addr = "255.255.255.255";
    let server_details = format!("{}:{}", dst_addr, UDP_PORT);

    // Create destination socket address
    let server: SocketAddr = server_details
        .parse()
        .expect("Unable to parse socket address");

    // Send UDP packet
    socket.send_to(buffer, server).expect("couldn't send data");
}

fn usage() {
    println!("Usage: ./wol-rs MAC-addr");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(mac_addr) => {
            println!("Send WOL to MAC {}", mac_addr);
            send_wol_package();
        }
        None => {
            usage();
        }
    }
}
