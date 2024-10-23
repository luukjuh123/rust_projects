extern crate log;

use env_logger::Env;

mod packet_capture;
mod packet_handler;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    if let Err(e) = packet_capture::start_capture() {
        eprintln!("Error capturing packets: {}", e);
    }
}
   
