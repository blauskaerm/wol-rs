use std::env;
use std::net::{SocketAddr, UdpSocket};
use std::process;

include!("wol.rs");

fn send_wol_package(dst_mac: [u8; 6]) {
    let sync_stream = [0xFF; 6];

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

fn parse_mac_argument(mac_string: &String) -> [u8; 6] {
    let mut result = [0x00; 6];

    for i in 0..6 {
        let mac_slice = &mac_string[(3 * i)..(3 * i + 2)];

        match i64::from_str_radix(mac_slice, 16) {
            Ok(value) => {
                result[i] = value as u8;
            }
            Err(_) => {
                println!("Error parsing");
                process::exit(1);
            }
        }
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(mac_addr_string) => {
            let mac_addr = parse_mac_argument(&mac_addr_string);

            println!("Send WOL to MAC {}", mac_addr_string);
            send_wol_package(mac_addr);
        }
        None => {
            usage();
        }
    }
}
