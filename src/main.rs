use std::net::{UdpSocket, SocketAddr};

include!("wol.rs");

fn main() {

    let sync_stream = [0xFF;6];
    let dst_mac = [
        0x1c,
        0x39,
        0x47,
        0xd0,
        0x9d,
        0xec
    ];


    let buffer = [0x48,0x45,0x4C,0x4C,0x4F,0x0A];

    // Create a bind socket
    let socket = UdpSocket::bind("127.0.0.1:34254")
        .expect("couldn't bind to address");

    // Create destination server address
    const UDP_PORT: u16 = 8080;
    let dst_addr = "127.0.0.1";
    let server_details = format!("{}:{}", dst_addr, UDP_PORT);

    // Create destination socket address
    let server: SocketAddr = server_details
        .parse()
        .expect("Unable to parse socket address");

    // Send UDP packet
    socket.send_to(&buffer, server)
        .expect("couldn't send data");
}
