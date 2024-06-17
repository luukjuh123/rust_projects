use clap::Parser;
use tokio::net::TcpStream;
use tokio::time::{self, Duration};
use tokio::sync::Semaphore;
use std::sync::Arc;
use log::{info, debug, error};
use env_logger;

#[derive(Parser)]
pub struct PortScanner {
    /// Sets the ip to scan (default: "127.0.0.1")
    #[arg(short, long, value_name = "IP",  default_value_t = String::from("127.0.0.1"))]
    ip_address: String,

    /// Sets the start port (default: 1)
    #[arg(long, value_name = "START",  default_value_t = 1)]
    start_port: u16,

    /// Sets the start port (default: 1024)
    #[arg(long, value_name = "END",  default_value_t = 1024)]
    end_port: u16,

    /// Sets the start port (default: 2)
    #[arg(long, value_name = "TIMEOUT",  default_value_t = 2)]
    timeout: u64,

    /// Sets the concurrency limit (default: 100)
    #[arg(long, value_name = "CONCURRENCY", default_value_t = 100)]
    concurrency: usize,


    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[tokio::main]
async fn main() {
    let args = PortScanner::parse();

    // Initialize logger
    let log_level = match args.debug {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    env_logger::Builder::new()
        .filter(None, log_level)
        .init();

    let ip_address = args.ip_address;
    let start_port = args.start_port;
    let end_port = args.end_port;
    let timeout = args.timeout;
    let concurrency = args.concurrency;

    let semaphore = Arc::new(Semaphore::new(concurrency));
    let mut handles = vec![];
    let open_ports = Arc::new(tokio::sync::Mutex::new(Vec::new()));

    for port in start_port..=end_port {
        let ip_address = ip_address.clone();
        let semaphore = semaphore.clone();
        let open_ports = open_ports.clone();

        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            let address = format!("{}:{}", ip_address, port);
            let try_connect = time::timeout(
                Duration::from_secs(timeout),
                TcpStream::connect(&address),
            )
            .await;

            match try_connect {
                Ok(Ok(_)) => {
                    open_ports.lock().await.push(port);
                    info!("Port {} is open", port);
                },
                _ => {
                    debug!("Port {} is closed", port);
                },
            }
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        if let Err(e) = handle.await {
            error!("Task failed: {:?}", e);
        }
    }

    // Print the summary of open ports
    let open_ports = open_ports.lock().await;
    println!("\nOpen Ports:");
    for port in open_ports.iter() {
        println!("Port {} is open", port);
    }
}