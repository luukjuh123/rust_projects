mod handlers;

fn main() {
    loop {
        // Prompt user for operation
        println!("Choose an operation: (+, -, *, /) or 'q' to quit:");
        let operation = get_user_input();

        if operation == "q" {
            break;
        }

        println!("Enter the first number:");
        let num1: f64 = get_user_input()
            .parse()
            .expect("Please enter a valid number.");

        println!("Enter the second number:");
        let num2: f64 = get_user_input()
            .parse()
            .expect("Please enter a valid number.");

        let result = match operation.as_str() {
            "+" => handlers::add(num1, num2),
            "-" => handlers::subtract(num1, num2),
            "*" => handlers::multiply(num1, num2),
            "/" => handlers::divide(num1, num2),
            _ => {
                println!("Invalid operation!");
                continue;
            }
        };

        println!("Result: {}", result);
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
    input.trim().to_string()
}
