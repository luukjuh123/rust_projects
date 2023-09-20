use crate::DebtCalculator;
use cli_table::{format::Justify, print_stdout, Cell, CellStruct, Style, Table};

#[derive(Table)]
pub struct AmortizationDetail {
    pub month: u32,
    pub interest_payment: f64,
    pub principal_payment: f64,
    pub remaining_balance: f64,
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

pub fn amortization_schedule(args: DebtCalculator) -> Vec<AmortizationDetail> {
    let monthly_rate = args.rate / 12.0 / 100.0;
    let num_payments = args.term * 12;
    let total_interest = args.principal * monthly_rate * num_payments as f64;
    let monthly_interest = total_interest / num_payments as f64;
    let monthly_principal_payment = args.principal / num_payments as f64;
    let monthly_payment = monthly_principal_payment + monthly_interest;

    let mut current_principal = args.principal;
    let mut schedule = Vec::new();

    for month in 1..=num_payments {
        current_principal -= monthly_principal_payment;

        match args.period {
            CalculationPeriod::Monthly => {
                schedule.push(AmortizationDetail {
                    month,
                    interest_payment: monthly_interest,
                    principal_payment: monthly_principal_payment,
                    remaining_balance: current_principal,
                });
            }
            CalculationPeriod::Yearly => {
                if month % 12 == 0 {
                    schedule.push(AmortizationDetail {
                        month: month / 12,
                        interest_payment: monthly_interest * 12.0,
                        principal_payment: monthly_principal_payment * 12.0,
                        remaining_balance: current_principal,
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
        ])
        .bold(true);

    assert!(print_stdout(table).is_ok());
}
