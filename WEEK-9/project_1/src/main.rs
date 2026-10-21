use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("nigerian_breweries.csv").expect("Failed to create file");
    file.write_all(b"Category,Drink\n").expect("Failed to write to file");
    file.write_all(b"Lager, Goldberg\n").expect("Failed to write to file");
    file.write_all("Lager, Gulder\n".as_bytes()).expect("Failed to write to file");
    file.write_all("Lager, Heineken\n".as_bytes()).expect("Failed to write to file");
    file.write_all("Lager, Star\n".as_bytes()).expect("Failed to write to file");
    file.write_all("Stout, Legend\n".as_bytes()).expect("Failed to write to file");
    file.write_all("Stout, Turbo King\n".as_bytes()).expect("Failed to write to file");
    file.write_all("Stout, Williams\n".as_bytes()).expect("Failed to write to file");
    file.write_all("Non-Alcoholic, Maltina\n".as_bytes()).expect("Failed to write to file");
    file.write_all("Non-Alcoholic, Amstel Malta\n".as_bytes()).expect("Failed to write to file");
    file.write_all("Non-Alcoholic, Malta Gold\n".as_bytes()).expect("Failed to write to file");
    file.write_all("Non-Alcoholic, Fayrouz\n".as_bytes()).expect("Failed to write to file");

    println!("Nigerian Breweries data saved successfully!");
}