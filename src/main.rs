use std::io;
use std::io::Write;

fn calculate(x: f64, y: f64, operator: &str) -> f64 {
    match operator {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => panic!("Invalid operator"),
    }
}
fn main() {
    println!("Welcome to the calculator!");

    let mut input = String::new();
    let x: f64;
    let y: f64;
    let mut operator = String::new();

    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    x = input.trim().parse::<f64>().unwrap();

    input.clear();
    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    y = input.trim().parse::<f64>().unwrap();

    input.clear();
    print!("Enter the operator (+, -, *, /): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut operator).unwrap();

    let result = calculate(x, y, operator.trim());
    println!("The result is: {}", result);
}
