use crate::handlers::AmortizationDetail;
use crate::handlers::CalculationPeriod;
use std::fs::File;
use std::io::Write;

pub fn write_schedule_to_txt(schedule: &Vec<AmortizationDetail>, period: CalculationPeriod) {
    let period_label = match period {
        CalculationPeriod::Monthly => "Month",
        CalculationPeriod::Yearly => "Year",
    };

    let mut file = File::create("amortization_schedule.txt").expect("Unable to create file");

    writeln!(
        file,
        "{},{},{},{},{},{}",
        period_label, "Interest", "Principal", "Remaining Balance", "MID", "Net Payment"
    )
    .expect("Unable to write data");

    for detail in schedule {
        writeln!(
            file,
            "{},{:.2},{:.2},{:.2},{:.2},{:.2}",
            detail.month,
            detail.interest_payment,
            detail.principal_payment,
            detail.remaining_balance,
            detail.mid,
            detail.net_payment
        )
        .expect("Unable to write data");
    }

    println!("Data written to amortization_schedule.txt");
}
