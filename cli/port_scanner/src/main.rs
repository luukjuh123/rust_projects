use tokio::net::TcpStream;
use tokio::time::{self, Duration};

const IP_ADDRESS: &str = "127.0.0.1";
const START_PORT: u16 = 1;
const END_PORT: u16 = 10240;
const TIMEOUT: u64 = 2;  // timeout in seconds

#[derive(Parser)]
pub struct DebtCalculator {
    /// Sets the principal debt amount
    #[arg(short, long, value_name = "NETWORK")]
    network: f64,
}


#[tokio::main]
async fn main() {
    for port in START_PORT..=END_PORT {
        let address = format!("{}:{}", IP_ADDRESS, port);

        // Use timeout function from tokio::time
        let try_connect = time::timeout(
            Duration::from_secs(TIMEOUT),
            TcpStream::connect(&address)
        ).await;

        match try_connect {
            Ok(Ok(_)) => {
                println!("Port {} is open", port);
            },
            _ => {
                // Comment this out if you don't want to see which ports are closed
                // println!("Port {} is closed", port);
            }
        }
    }
}
