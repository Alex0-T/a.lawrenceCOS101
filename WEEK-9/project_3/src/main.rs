use std::fs::File;
use std::io::Write;

fn main() {
    let commissioners = ["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etiye"];
    let ministries = ["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let zones = ["South West", "North East", "South South", "Suth West", "South East"];

    let mut file = File::create("EFCC_merge.csv").expect("Failed to cretae file");
    file.write_all("Commissioner, Ministry, Zone\n".as_bytes()).expect("Failed to create file");

    for i in 0..5 {
        let record = format!("{}, {}, {}\n", commissioners[i], ministries[i],zones[i]);
        file.write_all(record.as_bytes()).expect("Failed to write to file");
    }
    println!("EFCC datasets merged successfully!");
}