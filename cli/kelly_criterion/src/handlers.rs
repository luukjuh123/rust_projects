use crate::KellyInvestmentArgs;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

#[derive(Table)]
pub struct BetDetail {
    pub optimal_fraction: f64,
    pub optimal_bet: f64,
}

pub fn kelly_investment(args: KellyInvestmentArgs) -> BetDetail {
    let q = 1.0 - args.p;
    let f_star = (args.p / args.l) - (q / args.g);
    let optimal_investment = f_star * args.assets;

    // if f_star > 1.0 {
    //     f_star = 1.0; // Cap the fraction at 100%
    // }

    BetDetail {
        optimal_fraction: f_star,
        optimal_bet: optimal_investment,
    }
}

pub fn display_bet(detail: &BetDetail) {
    let table = vec![
        vec![
            "Optimal Fraction".to_string().cell(),
            format!("{:.2}%", detail.optimal_fraction * 100.0)
                .cell()
                .justify(Justify::Right),
        ],
        vec![
            "Optimal Bet ($)".to_string().cell(),
            format!("{:.2}", detail.optimal_bet)
                .cell()
                .justify(Justify::Right),
        ],
    ]
    .table()
    .bold(true);

    assert!(print_stdout(table).is_ok());
}
