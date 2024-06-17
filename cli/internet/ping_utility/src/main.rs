use clap::Parser;
use pnet::packet::icmp::{echo_request::MutableEchoRequestPacket, IcmpTypes};
use pnet::transport::{icmp_packet_iter, transport_channel, TransportChannelType::Layer3};
use pnet::packet::ip::IpNextHeaderProtocols;
use std::net::Ipv4Addr;
use std::net::IpAddr;
use std::time::{Duration, Instant};
use pnet::packet::Packet;

#[derive(Parser)]
pub struct PingUtility {
    /// Sets the ip to scan (default: "127.0.0.1")
    #[arg(short, long, value_name = "IP",  default_value_t = String::from("8.8.8.8"))]
    ip_address: String,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}


fn main() {
    let args = PingUtility::parse();

    let target_ip = match args.ip_address.parse::<Ipv4Addr>() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("Invalid IP address.");
            return;
        }
    };

    let target_ip = IpAddr::V4(target_ip);

    let (mut tx, mut rx) = transport_channel(1024, Layer3(IpNextHeaderProtocols::Icmp))
    .expect("Error creating the transport channel");

    let mut icmp_packet_buf = [0u8; 64]; // Buffer for the ICMP packet
    let mut icmp_packet = MutableEchoRequestPacket::new(&mut icmp_packet_buf[..]).unwrap();

    // Set up the ICMP packet
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_sequence_number(1);
    icmp_packet.set_identifier(1);
    let checksum = pnet::util::checksum(icmp_packet.packet(), 1);
    icmp_packet.set_checksum(checksum);

    match tx.send_to(icmp_packet.to_immutable(), target_ip) {
        Ok(_) => println!("Sent ICMP echo request to {}", target_ip),
        Err(e) => eprintln!("Failed to send ICMP echo request: {}", e),
    }

    let start_time = Instant::now();
    let mut iter = icmp_packet_iter(&mut rx);
    loop {
        match iter.next_with_timeout(Duration::from_secs(1)) {
            Ok(Some((packet, _))) => {
                if packet.get_icmp_type() == IcmpTypes::EchoReply {
                    println!("Received ICMP echo reply in {:?}", start_time.elapsed());
                    break;
                }
            }
            Ok(None) => {
                println!("Timeout waiting for ICMP echo reply");
                break;
            }
            Err(e) => {
                println!("An error occurred while waiting for ICMP echo reply: {}", e);
                break;
            }
        }
    }
}
