# rust-simple-calculator
In this repo, I did a simply calculator using rust. 


# rust-simple-calculator
In this repo, I did a simply calculator using rust. 


Here's the process:
1. Create a new Rust project using the cargo command, which is the package manager for Rust.

```
cargo new calculator
```

2. In the main.rs file of the new project, import the necessary libraries for input/output and mathematical operations. For example:

```
use std::io;
```

3. Define a function that will handle the calculator's logic. This function should take in two numbers and an operator as input, and return the result of the calculation. 

```
fn calculate(num1: i64, num2: i64, operator: &str) -> i64 {
    match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => panic!("Invalid operator"),
    }
}
```

4. In the main function, use io functions to get input from the user for the two numbers and the operator. 

```
let mut num1 = String::new();
io::stdin().read_line(&mut num1).unwrap();
let num1: i32 = num1.trim().parse().unwrap();

let mut num2 = String::new();
io::stdin().read_line(&mut num2).unwrap();
let num2: i32 = num2.trim().parse().unwrap();

let mut operator = String::new();
io::stdin().read_line(&mut operator).unwrap();
let operator: &str = operator.trim();
```

5. Call the calculate function with the user input and print the result.

```
let result = calculate(num1, num2, operator);
println!("{}", result);
```

6. Finally, build and run the project using the following command:
```
cargo run
```
