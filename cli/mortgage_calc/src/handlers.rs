use crate::MortgageCalculator;
use cli_table::{format::Justify, print_stdout, Cell, CellStruct, Style, Table};

#[derive(Table)]
pub struct AmortizationDetail {
    month: u32,
    interest_payment: f64,
    principal_payment: f64,
    remaining_balance: f64,
    mid: f64,
    net_payment: f64,
}

pub fn amortization_schedule(args: MortgageCalculator) -> Vec<AmortizationDetail> {
    let monthly_rate = args.rate / 12.0 / 100.0;
    let num_payments = args.term * 12;
    let monthly_payment = args.principal
        * (monthly_rate * (1.0 + monthly_rate).powi(num_payments as i32))
        / ((1.0 + monthly_rate).powi(num_payments as i32) - 1.0);

    let notional_rental_value_annual = args.woz * 0.0035; // Annual notional rental value

    let tax_bracket_for_rental = if args.income > 73032.0 { 0.495 } else { 0.3693 };
    let tax_cost_annual = notional_rental_value_annual * tax_bracket_for_rental; // Tax on the notional rental value
    let tax_cost_per_month = tax_cost_annual / 12.0; // Monthly tax on the notional rental value

    let mortgage_interest_deduction_bracket = 0.3693;
    let mut current_principal = args.principal;
    let mut schedule = Vec::new();

    for month in 1..=num_payments {
        let monthly_interest = current_principal * monthly_rate;
        let principal_payment = monthly_payment - monthly_interest;
        let mortgage_interest_deduction =
            monthly_interest * mortgage_interest_deduction_bracket - tax_cost_per_month;

        let net_payment = monthly_interest + principal_payment - mortgage_interest_deduction;

        current_principal -= principal_payment;

        schedule.push(AmortizationDetail {
            month,
            interest_payment: monthly_interest,
            principal_payment,
            remaining_balance: current_principal,
            mid: mortgage_interest_deduction,
            net_payment,
        });
    }

    schedule
}

pub fn display_schedule(schedule: Vec<AmortizationDetail>) {
    let table: Vec<Vec<CellStruct>> = schedule
        .into_iter()
        .map(|detail| {
            vec![
                detail.month.to_string().cell().justify(Justify::Right),
                format!("{:.2}", detail.interest_payment)
                    .cell()
                    .justify(Justify::Right),
                format!("{:.2}", detail.principal_payment)
                    .cell()
                    .justify(Justify::Right),
                format!("{:.2}", detail.remaining_balance)
                    .cell()
                    .justify(Justify::Right),
                format!("{:.2}", detail.mid).cell().justify(Justify::Right),
                format!("{:.2}", detail.net_payment)
                    .cell()
                    .justify(Justify::Right),
            ]
        })
        .collect();

    let table = table
        .table()
        .title(vec![
            "Month".cell().bold(true),
            "Interest".cell().bold(true),
            "Principal".cell().bold(true),
            "Remaining Balance".cell().bold(true),
            "MID".cell().bold(true),
            "Net Payment".cell().bold(true),
        ])
        .bold(true);

    assert!(print_stdout(table).is_ok());
}
