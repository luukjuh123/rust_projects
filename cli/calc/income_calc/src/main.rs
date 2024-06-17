mod handlers;
use clap::Parser;

#[derive(Parser)]
pub struct LoanCalculator {
    /// Sets the principal loan amount
    #[arg(short, long, value_name = "GROSS")]
    gross_income: f64,

    /// Sets the calculation period
    #[arg(long, value_name = "PERIOD")]
    period: handlers::CalculationPeriod,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}


fn main() {
    let args = LoanCalculator::parse();

    let gross_income: f64 = args.gross_income;
    let period: handlers::CalculationPeriod = args.period;

    println!("Gross Income: ${}", gross_income);
    println!("Period: {:?}", period);

    match args.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    let netto_income = handlers::calculate_netto_income(gross_income, period);
}
