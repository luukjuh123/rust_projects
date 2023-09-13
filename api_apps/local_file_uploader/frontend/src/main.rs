use reqwest;
use std::env;
use std::fs::File;
use std::io::BufReader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return Ok(());
    }
    let file_path = &args[1];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let url = "http://localhost:8000/upload";

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .multipart(reqwest::multipart::Form::new().part("file", reqwest::multipart::Part::reader(reader)))
        .send()
        .await?;

    println!("Server responded with: {}", response.text().await?);

    Ok(())
}
