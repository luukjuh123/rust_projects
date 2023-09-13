use reqwest;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return Ok(());
    }
    let file_path = args[1].clone();

    let file = File::open(&file_path)?;
    let mut reader = BufReader::new(file);

    // Read the entire file content into a byte vector
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    // Create the part from the buffer
    let part = reqwest::multipart::Part::bytes(buffer).file_name(file_path);

    let url = "http://localhost:3030/upload";

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .multipart(reqwest::multipart::Form::new().part("file", part))
        .send()
        .await?;

    println!("Server responded with: {}", response.text().await?);

    Ok(())
}
