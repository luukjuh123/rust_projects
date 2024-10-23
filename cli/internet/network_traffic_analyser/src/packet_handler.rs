use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use std::collections::HashMap;

pub fn handle_packet(packet: &EthernetPacket, traffic_volume: &mut HashMap<String, u64>) {
    match packet.get_ethertype() {
        pnet::packet::ethernet::EtherTypes::Ipv4 => {
            if let Some(ipv4_packet) = Ipv4Packet::new(packet.payload()) {
                let src = ipv4_packet.get_source().to_string();
                let dst = ipv4_packet.get_destination().to_string();
                let len = ipv4_packet.packet().len() as u64;

                *traffic_volume.entry(src.clone()).or_insert(0) += len;
                *traffic_volume.entry(dst.clone()).or_insert(0) += len;

                log::info!("Captured IPv4 packet from {} to {}: {:?}", src, dst, ipv4_packet);

                match ipv4_packet.get_next_level_protocol() {
                    pnet::packet::ip::IpNextHeaderProtocols::Tcp => {
                        if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
                            log::info!("Captured TCP packet: {:?}", tcp_packet);
                        }
                    }
                    pnet::packet::ip::IpNextHeaderProtocols::Udp => {
                        if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
                            log::info!("Captured UDP packet: {:?}", udp_packet);
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

