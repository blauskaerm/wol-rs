use std::net::{SocketAddr, UdpSocket};
use std::process;

extern crate clap;
use clap::{App, Arg};

fn send_wol_package(dst_mac: [u8; 6]) {
    let wol_sync_stream = [0xFF; 6];

    // Create Magic packet
    let buffer_cap: usize = wol_sync_stream.len() + 16 * dst_mac.len();
    let mut buffer: Vec<u8> = Vec::with_capacity(buffer_cap);
    buffer.extend(wol_sync_stream.iter().copied());
    for _i in 0..16 {
        buffer.extend(dst_mac.iter().copied());
    }

    // Create a bind socket
    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    socket
        .set_broadcast(true)
        .expect("Unable to set broadcast flag");

    // Create destination server address
    const UDP_PORT: u16 = 9;
    let dst_addr = "255.255.255.255";
    let dst_details = format!("{}:{}", dst_addr, UDP_PORT);

    // Create destination socket address
    let socket_addr: SocketAddr = dst_details.parse().expect("Unable to parse socket address");

    // Send UDP packet
    socket
        .send_to(buffer.as_slice(), socket_addr)
        .expect("couldn't send data");
}

fn parse_mac_argument(mac_string: &String) -> Result<[u8; 6], ()> {
    let mut result = [0x00; 6];

    const MAC_ADDR_STR_LEN: usize = 17;
    if mac_string.len() == MAC_ADDR_STR_LEN {
        for i in 0..6 {
            let mac_slice = &mac_string[(3 * i)..(3 * i + 2)];

            match i64::from_str_radix(mac_slice, 16) {
                Ok(value) => {
                    result[i] = value as u8;
                }
                Err(_) => return Err(()),
            }
        }
    } else {
        return Err(());
    }

    Ok(result)
}

fn main() {
    let matches = App::new("wol-rs")
        .version("0.1")
        .author("Erik Viktorsson <blauskaerm@protonmail.ch>")
        .about("Simple Wake On Lan client")
        .arg(
            Arg::with_name("MAC")
                .help("MAC address of target machine (XX:XX:XX:XX:XX:XX)")
                .required(true)
                .index(1),
        )
        .get_matches();

    let mac_addr_string = String::from(matches.value_of("MAC").unwrap_or(""));

    match parse_mac_argument(&mac_addr_string) {
        Ok(mac_addr) => {
            println!("Send WOL to MAC {}", mac_addr_string);
            send_wol_package(mac_addr);
        }
        Err(_) => {
            eprintln!("Failed to parse MAC address");
            process::exit(-1)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_mac_ok() {
        let mac_addr_arg = String::from("00:90:27:85:CF:00");
        let expected: [u8; 6] = [0x00, 0x90, 0x27, 0x85, 0xCF, 0x00];
        let test_result = parse_mac_argument(&mac_addr_arg);
        assert_eq!(test_result, Ok(expected));
    }

    #[test]
    fn test_parse_mac_short() {
        let mac_addr_arg = String::from("00:90:27:85:CF:0");
        let test_result = parse_mac_argument(&mac_addr_arg);
        assert_eq!(test_result, Err(()));
    }

    #[test]
    fn test_parse_mac_long() {
        let mac_addr_arg = String::from("00:90:27:85:CF:00:00");
        let test_result = parse_mac_argument(&mac_addr_arg);
        assert_eq!(test_result, Err(()));
    }

    #[test]
    fn test_parse_mac_invalid() {
        let mac_addr_arg = String::from("00:90:27:85:QF:00");
        let test_result = parse_mac_argument(&mac_addr_arg);
        assert_eq!(test_result, Err(()));
    }
}
