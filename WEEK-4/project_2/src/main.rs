// Rust program for an incentive calculator

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your age: ");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");

    let age: i8 = input1
        .trim()
        .parse()
        .expect("Not a valid number");

    if age < 0 {
        println!("Age cannot be negative.");
        return;
    }

    println!("\nPlease select your skill level:");
    println!("1. Experienced");
    println!("2. Not experienced");

    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");

    let experience: i8 = input2
        .trim()
        .parse()
        .expect("Not a valid choice");

    if experience == 1 && age >= 40 {
        println!("Annual incentive: N1,560,000");
    } else if experience == 1 && age >= 30 {
        println!("Annual incentive: N1,480,000");
    } else if experience == 1 && age < 28 {
        println!("Annual incentive: N1,300,000");
    } else if experience == 2 {
        println!("Annual incentive: N100,000");
    } else if experience == 1 {
        println!("No incentive specified for experienced employees aged 28 or 29.");
    } else {
        println!("Invalid skill level.");
    }
}