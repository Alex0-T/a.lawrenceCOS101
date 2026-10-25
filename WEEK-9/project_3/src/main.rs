use std::fs::File;
use std::io::Write;

fn main() {
    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Create the commissioners CSV file
    let mut file_1 = File::create("commissioners.csv")
        .expect("Failed to create commissioners file");

    file_1.write_all("Commissioner\n".as_bytes())
        .expect("Failed to write to commissioners file");

    for i in 0..5 {
        let record = format!("{}\n", commissioners[i]);

        file_1.write_all(record.as_bytes())
            .expect("Failed to write to commissioners file");
    }

    // Create the ministries CSV file
    let mut file_2 = File::create("ministries.csv")
        .expect("Failed to create ministries file");

    file_2.write_all("Ministry\n".as_bytes())
        .expect("Failed to write to ministries file");

    for i in 0..5 {
        let record = format!("{}\n", ministries[i]);

        file_2.write_all(record.as_bytes())
            .expect("Failed to write to ministries file");
    }

    // Create the zones CSV file
    let mut file_3 = File::create("zones.csv")
        .expect("Failed to create zones file");

    file_3.write_all("Zone\n".as_bytes())
        .expect("Failed to write to zones file");

    for i in 0..5 {
        let record = format!("{}\n", zones[i]);

        file_3.write_all(record.as_bytes())
            .expect("Failed to write to zones file");
    }

    // Merge the three datasets into one CSV file
    let mut file_4 = File::create("EFCC_merge.csv")
        .expect("Failed to create merged file");

    file_4.write_all("Commissioner,Ministry,Zone\n".as_bytes())
        .expect("Failed to write to merged file");

    for i in 0..5 {
        let record = format!(
            "{},{},{}\n",
            commissioners[i],
            ministries[i],
            zones[i]
        );

        file_4.write_all(record.as_bytes())
            .expect("Failed to write to merged file");
    }

    println!("Three CSV files created and merged successfully!");
}
