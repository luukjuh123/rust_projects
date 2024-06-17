use clap::Parser;
use tokio::net::lookup_host;

#[derive(Parser)]
pub struct DnsLookup {
    /// Sets the hostname to resolve (default: "localhost")
    #[arg(short, long, value_name = "HOST", default_value_t = String::from("localhost"))]
    hostname: String,
}

#[tokio::main]
async fn main() {
    let args = DnsLookup::parse();

    match lookup_host(args.hostname).await {
        Ok(addresses) => {
            for address in addresses {
                println!("Resolved address: {}", address);
            }
        }
        Err(e) => println!("DNS lookup failed: {}", e),
    }
}
