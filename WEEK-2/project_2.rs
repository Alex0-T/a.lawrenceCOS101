fn main(){
	 let qty = [2, 1, 3, 3, 1];
     let amount = [450000.00, 1500000.00, 750000.00, 2850000.00, 250000.00];

     let items = ["Tosiba", "Mac", "HP", "Dell","Acer"];
     //sum of quantity
     let sum_qty = qty[0] + qty[1] + qty[2] + qty[3] + qty[4];

     //average quantity
     let average_qty = sum_qty as f64 / qty.len() as f64;

     //sum of amount
     let sum_amount = amount[0] + amount[1] + amount[2] + amount[3] + amount[4];     

     //average amount
     let average_amount = sum_amount / amount.len() as f64;

     println!("Items: {:?}", items);
     println!("Total Quantity: {}", sum_qty);
     println!("Average Quantity: {}", average_qty);
     println!("Total Amount: {}", sum_amount);
     println!("Average amount: {}", average_amount);

}