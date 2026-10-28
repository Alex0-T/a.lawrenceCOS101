struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn purchase_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let laptops = [
        Laptop {
            brand: String::from("HP"),
            price: 650000,
        },
        Laptop {
            brand: String::from("IBM"),
            price: 755000,
        },
        Laptop {
            brand: String::from("Toshiba"),
            price: 550000,
        },
        Laptop {
            brand: String::from("Dell"),
            price: 650000,
        },
    ];

    let mut total_cost = 0;

    for laptop in &laptops {
        let cost = laptop.purchase_cost(3);

        println!("{}: N{}", laptop.brand, cost);

        total_cost += cost;
    }

    println!("Total cost: N{}", total_cost);
}