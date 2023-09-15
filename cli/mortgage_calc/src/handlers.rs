use crate::MortgageCalculator;
use cli_table::{format::Justify, print_stdout, Cell, CellStruct, Style, Table};

#[derive(Table)]
pub struct AmortizationDetail {
    pub month: u32,
    pub interest_payment: f64,
    pub principal_payment: f64,
    pub remaining_balance: f64,
    pub mid: f64,
    pub net_payment: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CalculationPeriod {
    Monthly,
    Yearly,
}

use std::str::FromStr;

impl FromStr for CalculationPeriod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Monthly" => Ok(CalculationPeriod::Monthly),
            "Yearly" => Ok(CalculationPeriod::Yearly),
            _ => Err(format!("'{}' is not a valid calculation period", s)),
        }
    }
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

        match args.period {
            CalculationPeriod::Monthly => {
                schedule.push(AmortizationDetail {
                    month,
                    interest_payment: monthly_interest,
                    principal_payment,
                    remaining_balance: current_principal,
                    mid: mortgage_interest_deduction,
                    net_payment,
                });
            }
            CalculationPeriod::Yearly => {
                if month % 12 == 0 {
                    schedule.push(AmortizationDetail {
                        month: month / 12,
                        interest_payment: monthly_interest * 12.0,
                        principal_payment: principal_payment * 12.0,
                        remaining_balance: current_principal,
                        mid: mortgage_interest_deduction * 12.0,
                        net_payment: net_payment * 12.0,
                    });
                }
            }
        }
    }

    schedule
}

pub fn display_schedule(schedule: &Vec<AmortizationDetail>, period: CalculationPeriod) {
    let period_label = match period {
        CalculationPeriod::Monthly => "Month",
        CalculationPeriod::Yearly => "Year",
    };

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
            period_label.cell(),
            "Interest".cell(),
            "Principal".cell(),
            "Remaining Balance".cell(),
            "MID".cell(),
            "Net Payment".cell(),
        ])
        .bold(true);

    assert!(print_stdout(table).is_ok());
}
