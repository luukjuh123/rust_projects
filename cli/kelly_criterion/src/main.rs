mod handlers;
use clap::Parser;

#[derive(Parser)]
pub struct KellyInvestmentArgs {
    /// Probability that the investment increases in value (0 <= p <= 1)
    #[arg(short = 'p', long, value_name = "PROBABILITY_INCREASE")]
    p: f64,

    /// Fraction that is lost in a negative outcome (e.g., 0.1 for 10% loss)
    #[arg(short = 'l', long, value_name = "FRACTION_LOSS")]
    l: f64,

    /// Fraction that is gained in a positive outcome (e.g., 0.1 for 10% gain)
    #[arg(short = 'g', long, value_name = "FRACTION_GAIN")]
    g: f64,

    /// Current assets
    #[arg(short = 'a', long, value_name = "ASSETS")]
    assets: f64,
}

fn main() {
    let args = KellyInvestmentArgs::parse();

    let detail = handlers::kelly_investment(args);

    println!("Based on the Kelly Criterion:");
    handlers::display_bet(&detail);
}
