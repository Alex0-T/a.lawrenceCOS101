use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("nigerian_breweries.csv").expect("Failed to create file");
    file.write_all(b"Lager, Stout, Non-Alcoholic\n").expect("Failed to write to file");
    file.write_all(b"33 Export, Legend, Maltina\n").expect("Failed to write to file");
    file.write_all("Desperados, Turbo King, Amstel Malta\n".as_bytes()).expect("Failed to write to file");
    file.write_all("Goldberg, Williams, Malta Gold\n".as_bytes()).expect("Failed to write to file");
    file.write_all("Gulder,,Fayrouz\n".as_bytes()).expect("Failed to write to file");
    file.write_all("Heineken,,\n".as_bytes()).expect("Failed to write to file");
    file.write_all("Star,,\n".as_bytes()).expect("Failed to write to file");
 
    println!("Nigerian Breweries data saved successfully!");
}