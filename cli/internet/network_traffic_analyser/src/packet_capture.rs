use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::EthernetPacket;
use std::collections::HashMap;
use std::io::{self, Write};

use crate::packet_handler::handle_packet;

pub fn list_interfaces() -> Vec<NetworkInterface> {
    datalink::interfaces()
}

pub fn start_capture() -> Result<(), Box<dyn std::error::Error>> {
    let interfaces = list_interfaces();
    
    println!("Available interfaces:");
    for (index, iface) in interfaces.iter().enumerate() {
        println!("{}: {}", index, iface.name);
    }

    let interface = loop {
        print!("Select an interface by index: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<usize>() {
            Ok(index) if index < interfaces.len() => break &interfaces[index],
            Ok(_) => println!("Invalid index. Please enter a number between 0 and {}.", interfaces.len() - 1),
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    };

    println!("Selected interface: {}", interface.name);

    match datalink::channel(interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(_tx, mut rx)) => {
            let mut packet_count = 0;
            let mut traffic_volume: HashMap<String, u64> = HashMap::new();

            loop {
                match rx.next() {
                    Ok(packet) => {
                        if let Some(packet) = EthernetPacket::new(packet) {
                            packet_count += 1;
                            handle_packet(&packet, &mut traffic_volume);
                            log::info!("Total packets: {}", packet_count);
                            log::info!("Traffic volume: {:?}", traffic_volume);
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to capture packet: {}", e);
                    }
                }
            }
        }
        Ok(_) => {
            panic!("Unhandled channel type");
        }
        Err(e) => {
            eprintln!("Failed to create datalink channel on interface {}: {}", interface.name, e);
            println!("Try selecting another interface.");
            return start_capture(); // Restart the process
        }
    }
}

