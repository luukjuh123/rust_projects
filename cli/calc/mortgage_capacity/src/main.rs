use clap::Parser;


#[derive(Parser)]
pub struct MortgageCapacityCalculator {
    /// Your salary
    #[arg(short, long, value_name = "INCOME")]
    income: f64,

    /// Partner's salary
    #[arg(short, long, value_name = "partnerincome")]
    partnerincome: f64,

    /// Total monthly financial obligations (loans, student debt, etc.)
    #[arg(short, long, value_name = "OBLIGATIONS")]
    obligations: f64,

    /// Interest rate (in percentage)
    #[arg(short, long, value_name = "RATE")]
    rate: f64,

    /// Loan term (in months)
    #[arg(short, long, value_name = "TERM")]
    term: u32,

    /// Market value of the house
    #[arg(short, long, value_name = "MARKET_VALUE")]
    market_value: f64,

    /// Debug mode
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let args = MortgageCapacityCalculator::parse();

    let combined_income = args.income + args.partnerincome;
    let max_monthly_payment = calculate_max_monthly_payment(combined_income, args.obligations);
    let mortgage_capacity = calculate_mortgage_capacity(max_monthly_payment, args.rate, args.term);

    println!("With a combined income of {:.2} and a monthly payment of {:.2}, the estimated mortgage capacity is: {:.2}",
             combined_income, max_monthly_payment, mortgage_capacity);
}

fn calculate_max_monthly_payment(income: f64, obligations: f64) -> f64 {
    // Allowing 30% of total income for mortgage payments
    let max_payment = income * 0.30;
    // Deduct financial obligations
    max_payment - obligations
}

fn calculate_mortgage_capacity(max_monthly_payment: f64, rate: f64, term: u32) -> f64 {
    let monthly_rate = rate / 12.0 / 100.0;
    let num_payments = term;
    max_monthly_payment * ((1.0 - (1.0 + monthly_rate).powi(-(num_payments as i32))) / monthly_rate)
}

