use clap::Parser;
use rand::seq::SliceRandom;

#[derive(Parser)]
pub struct PasswordGenerator {
    /// Sets the length of the password
    #[arg(short, long, value_name = "LENGTH")]
    length: u16,

    /// Include special characters in the password
    #[arg(short, long, value_name = "SPECIALS")]
    specials: bool,

    /// Exclude numbers from the password
    #[arg(short, long, value_name = "NO_NUMBERS")]
    no_numbers: bool,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let args = PasswordGenerator::parse();

    let length: u16 = args.length;
    let specials: bool = args.specials;
    let no_numbers: bool = args.no_numbers;

    let mut charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    if !no_numbers {
        charset.extend("0123456789".chars());
    }

    if specials {
        charset.extend("!@#$%^&*()-_=+<>?".chars());
    }

    let password: String = (0..length)
        .map(|_| charset.choose(&mut rand::thread_rng()).unwrap())
        .collect();

    println!("Generated password: {}", password);
}
