use std::io;
use std::io::Write;

use console::Term;

fn main() {
    let mut first_number: String = String::new();
    let mut second_number: String = String::new();
    let mut operation: Term = Term::stdout();
    let result: i128;

    print!("Enter the first number: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read line");

    print!("Enter the second number: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read line");

    let first_number: i128 = first_number.trim().parse().expect("Please type a number");
    let second_number: i128 = second_number.trim().parse().expect("Please type a number");

    print!("Select the operation (+, -, *, /): ");
    io::stdout().flush().unwrap();

    let operation: char = Term::read_char(&mut operation)
        .expect("Please type a valid character");

    println!("{operation}");

    match operation {
        '+' => result = first_number + second_number,
        '-' => result = first_number - second_number,
        '*' => result = first_number * second_number,
        '/' => result = first_number / second_number,
        _ => panic!("Please type a valid operation")
    }

    println!("Result: {}", result);
}
