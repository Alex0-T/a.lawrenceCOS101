use std::io;

fn main() {
    let mut candidates: Vec<(String, i32)> = Vec::new();

    println!("========================================");
    println!("            EY INTERVIEW");
    println!("========================================");

    println!("How many candidates are being interviewed?");

    let mut number_input = String::new();

    io::stdin()
        .read_line(&mut number_input)
        .expect("Failed to read input");

    let number_of_candidates: i32 = number_input
        .trim()
        .parse()
        .expect("Please enter a valid number.");

    if number_of_candidates <= 0 {
        println!("The number of candidates must be greater than zero.");
        return;
    }

    for i in 0..number_of_candidates {
        println!("\nCandidate {}", i + 1);

        println!("Enter candidate's name:");

        let mut name = String::new();

        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");

        let name = name.trim().to_string();

        println!("Enter years of programming experience:");

        let mut experience_input = String::new();

        io::stdin()
            .read_line(&mut experience_input)
            .expect("Failed to read input");

        let years: i32 = experience_input
            .trim()
            .parse()
            .expect("Please enter a valid number of years.");

        if years < 0 {
            println!("Years of experience cannot be negative.");
            return;
        }

        candidates.push((name, years));
    }

    let mut highest_name = "";
    let mut highest_years = -1;

    for (name, years) in &candidates {
        if *years > highest_years {
            highest_years = *years;
            highest_name = name;
        }
    }

    println!("\n========================================");
    println!("          EY INTERVIEW RESULTS");
    println!("========================================");
    println!("Most experienced candidate: {}", highest_name);
    println!("Years of programming experience: {}", highest_years);
    println!("========================================");
}