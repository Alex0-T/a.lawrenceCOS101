fn main() {
    let qty1 = 2;
    let qty2 = 1;
    let qty3 = 3;
    let qty4 = 3;
    let qty5 = 1;

    let amount1 = 450000.00;
    let amount2 = 1500000.00;
    let amount3 = 750000.00;
    let amount4 = 2850000.00;
    let amount5 = 250000.00;

    let sum_qty = qty1 + qty2 + qty3 + qty4 + qty5;
    let average_qty = sum_qty / 5;

    let sum_amount = amount1 + amount2 + amount3 + amount4 + amount5;
    let average_amount = sum_amount / 5.0;

    println!("Tosiba: {} {}", qty1, amount1);
    println!("Mac: {} {}", qty2, amount2);
    println!("HP: {} {}", qty3, amount3);
    println!("Dell: {} {}", qty4, amount4);
    println!("Acer: {} {}", qty5, amount5);

    println!("Total Quantity: {}", sum_qty);
    println!("Average Quantity: {}", average_qty);
    println!("Total Amount: {}", sum_amount);
    println!("Average Amount: {}", average_amount);
}