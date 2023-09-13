fn main() {
    loop {
        println!(
            "Choose a calc operation c (for monthly costs), m (for mortgage calc) or 'q' to quit:"
        );
        let operation = get_user_input();

        if operation == "q" {
            break;
        }

        if operation == "c" {
            println!("Enter total amount to be paid:");
            let payment_amount = get_user_input();
        }

        if operation == "m" {
            println!("Enter monthly income:");
            let monthly_income = get_user_input();
        }

        println!("Enter Montly Payment Amount:");
        let payment_amount = get_user_input();

        println!("Enter the first number:");
        let num1: f64 = get_user_input()
            .parse()
            .expect("Please enter a valid number.");

        println!("Enter the second number:");
        let num2: f64 = get_user_input()
            .parse()
            .expect("Please enter a valid number.");
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
    input.trim().to_string()
}
