mod handlers;
use clap::Parser;

#[derive(Parser)]
pub struct DebtCalculator {
    /// Sets the principal debt amount
    #[arg(short, long, value_name = "PRINCIPAL")]
    principal: f64,

    /// Sets the annual interest rate (in percentage, e.g., 5 for 5%)
    #[arg(short, long, value_name = "RATE")]
    rate: f64,

    /// Sets the loan term in years
    #[arg(short, long, value_name = "TERM")]
    term: u32,

    /// Sets the calculation period
    #[arg(long, value_name = "PERIOD")]
    period: handlers::CalculationPeriod,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let args = DebtCalculator::parse();

    let principal: f64 = args.principal;
    let rate: f64 = args.rate;
    let term: u32 = args.term;
    let period: handlers::CalculationPeriod = args.period;

    println!("Principal: ${}", principal);
    println!("Annual Interest Rate: {}%", rate);
    println!("Term: {} years", term);
    println!("Period: {:?}", period);

    // Debug mode
    match args.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    let schedule = handlers::amortization_schedule(args);
    handlers::display_schedule(&schedule, period);
}
