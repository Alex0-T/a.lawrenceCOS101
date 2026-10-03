use std::io;

fn main() {
    println!("Hello, Our esteemed valuable customer 😁😁😁!!!!\n");
    println!("Welcome to The Mouthful Cravings.... where Quality supersedes Quantity\n");
    println!("Kindly select from our beautiful options below😊😊😊");
    println!("========================================");
    println!("          RESTAURANT MENU");
    println!("========================================");
    println!("P - Poundo Yam / Edikangiko Soup   N3200");
    println!("F - Fried Rice & Chicken            N3000");
    println!("A - Amala & Ewedu Soup              N2500");
    println!("E - Eba & Egusi Soup                N2000");
    println!("W - White Rice & Stew               N2500");
    println!("========================================");

    println!("Enter food type:");

    let mut food = String::new();
    io::stdin()
        .read_line(&mut food)
        .expect("Failed to read input");

    let food = food.trim().to_uppercase();

    let mut food_name = "";
    let mut price = 0;

    match food.as_str() {
        "P" => {
            food_name = "Poundo Yam / Edikangiko Soup";
            price = 3200;
        }

        "F" => {
            food_name = "Fried Rice & Chicken";
            price = 3000;
        }

        "A" => {
            food_name = "Amala & Ewedu Soup";
            price = 2500;
        }

        "E" => {
            food_name = "Eba & Egusi Soup";
            price = 2000;
        }

        "W" => {
            food_name = "White Rice & Stew";
            price = 2500;
        }

        _ => {
            println!("Invalid food type.");
        }
    }

    if price != 0 {
        println!("Enter quantity:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let quantity: i32 = input.trim().parse().expect("Invalid quantity");

        let subtotal = price * quantity;

        let discount;

        if subtotal > 10_000 {
            discount = subtotal * 5 / 100;
        } else {
            discount = 0;
        }

        let final_total = subtotal - discount;

        println!();
        println!("========================================");
        println!("              RECEIPT");
        println!("========================================");
        println!("Food:       {}", food_name);
        println!("Unit price: N{}", price);
        println!("Quantity:   {}", quantity);
        println!("----------------------------------------");
        println!("Subtotal:   N{}", subtotal);
        println!("Discount:   N{}", discount);
        println!("Total:      N{}", final_total);
        println!("=============Have a blessed day!!!===========================\n");
        println!(
            "We look forward to seeing you next time 👌👌
            Byeeeeee🤗🤗🤗"
        );
    }
}
