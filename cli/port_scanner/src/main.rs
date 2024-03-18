use clap::Parser;
use tokio::net::TcpStream;
use tokio::time::{self, Duration};

#[derive(Parser)]
pub struct PortScanner {
    /// Sets the ip to scan (default: "127.0.0.1")
    #[arg(short, long, value_name = "IP",  default_value_t = String::from("127.0.0.1"))]
    ip_address: String,

    /// Sets the start port (default: 1)
    #[arg(long, value_name = "START",  default_value_t = 1)]
    start_port: u16,

    /// Sets the start port (default: 1024)
    #[arg(long, value_name = "START",  default_value_t = 1024)]
    end_port: u16,

    /// Sets the start port (default: 2)
    #[arg(long, value_name = "START",  default_value_t = 2)]
    timeout: u64,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[tokio::main]
async fn main() {
    let args = PortScanner::parse();

    let ip_address = args.ip_address;
    let start_port = args.start_port;
    let end_port = args.end_port;
    let timeout = args.timeout;

    let mut handles = vec![];

    for port in start_port..=end_port {
        let ip_address = ip_address.clone();
        let handle = tokio::spawn(async move {
            let address = format!("{}:{}", ip_address, port);
            let try_connect = time::timeout(
                Duration::from_secs(timeout),
                TcpStream::connect(&address),
            )
            .await;

            match try_connect {
                Ok(Ok(_)) => println!("Port {} is open", port),
                _ => {} // Optionally print closed ports or handle errors
            }
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let _ = handle.await;
    }
}