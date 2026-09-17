//Rust program to find the roots of a quadratic equation

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter a: ");
    io::stdin()
        .read_line(&mut input1)
        .expect("Not a valid string");
    let a: f32 = input1.trim().parse().expect("Not a valid number");

    if a == 0.0 {
        println!("This is not a quadratic equation.");
        return;
    }

    println!("Enter b: ");
    io::stdin()
        .read_line(&mut input2)
        .expect("Not a valid string");
    let b: f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter c: ");
    io::stdin()
        .read_line(&mut input3)
        .expect("Not a valid string");
    let c: f32 = input3.trim().parse().expect("Not a valid number");

    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
        let x1 = (-b + d.sqrt()) / (2.0 * a);
        let x2 = (-b - d.sqrt()) / (2.0 * a);

        println!("Two distinct roots:");
        println!("x1 = {}", x1);
        println!("x2 = {}", x2);
    } else if d == 0.0 {
        let x = -b / (2.0 * a);

        println!("Exactly one real root:");
        println!("x = {}", x);
    } else {
        println!("No real roots.");
    }
}
