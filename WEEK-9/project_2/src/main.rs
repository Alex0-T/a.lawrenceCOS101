use std::fs::File;
use std::io::Write;

fn main() {
    let students = vec![
    ["Oluchi Mordi", "ACC10211111", "Accounting", "300"],
    ["Adams Aliyu", "EC10110101", "Economics", "100"],
    ["Shania Bolade", "CSC10328828", "Computer", "200"],
    ["Adekunle Gold", "EEE11020202", "Electrical", "200"],
    ["Bianca Edemoh", "MEE10202001", "Mechanical", "100"],
    ];

    println!("Student Name, Matric. Number, Department, Level");

    for student in &students {
        println!("{}, {}, {}, {}", student[0], student[1], student[2], student[3]);
    }

    let mut file = File::create("PAU_SMIS.csv").expect("Failed to create file");

    file.write_all("Student Name, Matric. Number, Department, Level\n".as_bytes()).expect("Failed to write to file");

    for student in &students {
        let record = format!(
            "{}, {}, {}, {}\n", student[0], student[1], student[2], student[3]);

        file.write_all(record.as_bytes()).expect("Failed to write to file");
    }
    println!("\nStudent records saved successfully!");
}