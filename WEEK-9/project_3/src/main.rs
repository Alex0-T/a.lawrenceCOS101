use std::fs::File;
use std::fs::read_to_string;
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

    // Creating the commissioners CSV file
    let mut file1 = File::create("commissioners.csv")
        .expect("Failed to create commissioners file");

    file1.write_all(b"Commissioner\n")
        .expect("Failed to write to commissioners file");

    for i in 0..5 {
        let record = format!("{}\n", commissioners[i]);

        file1.write_all(record.as_bytes())
            .expect("Failed to write to commissioners file");
    }

    // Creating the ministries CSV file
    let mut file2 = File::create("ministries.csv")
        .expect("Failed to create ministries file");

    file2.write_all(b"Ministry\n")
        .expect("Failed to write to ministries file");

    for i in 0..5 {
        let record = format!("{}\n", ministries[i]);

        file2.write_all(record.as_bytes())
            .expect("Failed to write to ministries file");
    }

    // Creating the zones CSV file
    let mut file3 = File::create("zones.csv")
        .expect("Failed to create zones file");

    file3.write_all(b"Zone\n")
        .expect("Failed to write to zones file");

    for i in 0..5 {
        let record = format!("{}\n", zones[i]);

        file3.write_all(record.as_bytes())
            .expect("Failed to write to zones file");
    }

    // Reading the CSV files
    let commissioners_data = read_to_string("commissioners.csv")
        .expect("Failed to read commissioners file");

    let ministries_data = read_to_string("ministries.csv")
        .expect("Failed to read ministries file");

    let zones_data = read_to_string("zones.csv")
        .expect("Failed to read zones file");

    // Convert the file contents into lines
    let commissioners_lines: Vec<&str> =
        commissioners_data.lines().collect();

    let ministries_lines: Vec<&str> =
        ministries_data.lines().collect();

    let zones_lines: Vec<&str> =
        zones_data.lines().collect();

    // Creating the merged CSV file
    let mut file4 = File::create("EFCC_merge.csv")
        .expect("Failed to create merged file");

    file4.write_all(b"Commissioner,Ministry,Zone\n")
        .expect("Failed to write to merged file");

    for i in 1..commissioners_lines.len() {
        let record = format!(
            "{},{},{}\n",
            commissioners_lines[i],
            ministries_lines[i],
            zones_lines[i]
        );

        file4.write_all(record.as_bytes())
            .expect("Failed to write to merged file");
    }

    println!("Three CSV files created, read and merged successfully!");
}