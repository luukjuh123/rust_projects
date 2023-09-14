mod handlers;
use clap::Parser;

#[derive(Parser)]
pub struct MortgageCalculator {
    /// Sets the principal loan amount
    #[arg(short, long, value_name = "PRINCIPAL")]
    principal: f64,

    /// Sets the annual interest rate (in percentage, e.g., 5 for 5%)
    #[arg(short, long, value_name = "RATE")]
    rate: f64,

    /// Sets the loan term in years
    #[arg(short, long, value_name = "TERM")]
    term: u32,

    /// Sets the total worth of the house
    #[arg(short, long, value_name = "WOZ")]
    woz: f64,

    /// Sets the highest earning income
    #[arg(short, long, value_name = "INCOME")]
    income: f64,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let args = MortgageCalculator::parse();

    let principal: f64 = args.principal;
    let rate: f64 = args.rate;
    let term: u32 = args.term;
    let woz: f64 = args.woz;
    let income: f64 = args.income;

    println!("Principal: ${}", principal);
    println!("Annual Interest Rate: {}%", rate);
    println!("Loan Term: {} years", term);
    println!("Woz: {}", woz);
    println!("Income: {}", income);

    match args.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }
    // handlers::fixed_mortgage_calc(args);
    let schedule = handlers::amortization_schedule(args);
    handlers::display_schedule(schedule);
}
