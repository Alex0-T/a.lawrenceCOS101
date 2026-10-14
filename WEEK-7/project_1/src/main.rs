use std::io;

fn main() {
    loop {
        println!("\n=== The Shape Calculator ===");
        println!("1. Trapezium Area");
        println!("2. Rhombus Area");
        println!("3. Parallelogram Area");
        println!("4. Cube Surface Area");
        println!("5. Cylinder Volume");
        println!("6. Exit");
        println!("Enter your choice (1-6):");

        let choice = read_number();

        match choice as u32 {
            1 => {
                let area = calculate_trapezium();
                println!("Area of Trapezium: {:.2}", area);
            }
            2 => {
                let area = calculate_rhombus();
                println!("Area of Rhombus: {:.2}", area);
            }
            3 => {
                let area = calculate_parallelogram();
                println!("Area of Parallelogram: {:.2}", area);
            }
            4 => {
                let area = calculate_cube();
                println!("Surface Area of Cube: {:.2}", area);
            }
            5 => {
                let volume = calculate_cylinder();
                println!("Volume of Cylinder: {:.2}", volume);
            }
            6 => {
                println!("Exiting program. Goodbye!");
                break;
            }
            _ => println!("Invalid selection. Please enter a number between 1 and 6."),
        }
    }
}

// Helper function to read standard numbers from console
fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().parse::<f64>().unwrap_or(0.0)
}

fn calculate_trapezium() -> f64 {
    println!("Enter height:");
    let height = read_number();

    println!("Enter base 1:");
    let base1 = read_number();

    println!("Enter base 2:");
    let base2 = read_number();

    (height / 2.0) * (base1 + base2)
}

fn calculate_rhombus() -> f64 {
    println!("Enter diagonal 1:");
    let diagonal1 = read_number();

    println!("Enter diagonal 2:");
    let diagonal2 = read_number();

    0.5 * diagonal1 * diagonal2
}

fn calculate_parallelogram() -> f64 {
    println!("Enter base:");
    let base = read_number();

    println!("Enter altitude:");
    let altitude = read_number();

    base * altitude
}

fn calculate_cube() -> f64 {
    println!("Enter side length:");
    let side = read_number();

    6.0 * side * side
}

fn calculate_cylinder() -> f64 {
    println!("Enter radius:");
    let radius = read_number();

    println!("Enter height:");
    let height = read_number();

    std::f64::consts::PI * radius * radius * height
}