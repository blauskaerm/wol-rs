use std::net::UdpSocket;

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

    let socket = UdpSocket::bind("127.0.0.1:34254")
        .expect("couldn't bind to address");

    socket.send_to(&buffer, "127.0.0.1:8080")
        .expect("couldn't send data");


    println!("{}", sync_stream[0]);
}
