use std::io;

fn checker(){
    let mut input = String::new();
    println!("Enter a character: ");
    io::stdin().read_line(&mut input).expect("failed to read input");
    let ch: char = input.trim().parse().expect("Invalid input");

    if ch >= '0' && ch <= '9' {
        println!("Character {ch} is a digit");
    }
    else{
        println!("Character {ch} is not a digit");
    }
}

fn main() {
    //calling function
    println!("Welcome! This program whether a character variable contains a digit or not");
    checker();
}