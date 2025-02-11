//! A simple command-line calculator that supports addition, subtraction, multiplication, and division.
//!
//! # Examples
//!
//! ```sh
//! cargo run 5 + 3
//! cargo run 10 / 2
//! ```

use std::env;

/// Adds two numbers and returns the result.
fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtracts the second number from the first and returns the result.
fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiplies two numbers and returns the result.
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Divides the first number by the second and returns the result.
/// Returns an error if the second number is zero.
fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(a / b)
    }
}

fn main() {
    // Collect command-line arguments.
    let args: Vec<String> = env::args().collect();

    // Check if the user provided enough arguments.
    if args.len() != 4 {
        eprintln!("Usage: {} <num1> <operator> <num2>", args[0]);
        return;
    }

    // Parse the first number.
    let num1: f64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: '{}' is not a valid number", args[1]);
            return;
        }
    };

    // Parse the second number.
    let num2: f64 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: '{}' is not a valid number", args[3]);
            return;
        }
    };

    // Get the operator.
    let operator = &args[2];

    // Perform the calculation based on the operator.
    let result = match operator.as_str() {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => match divide(num1, num2) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        },
        _ => {
            eprintln!("Error: '{}' is not a valid operator", operator);
            return;
        }
    };

    // Print the result.
    println!("Result: {}", result);
}
