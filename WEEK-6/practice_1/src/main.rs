fn main() {
    let name = "Alex Lawrence";
    let uni: &str = "Pan-Atlantic University";
    let addr: &str = "Km 52 Lekki-Epe Expressway, Ibej-Lekki, Lagos";

    println!("Name: {name}");
    println!("University: {uni}, \nAddress: {addr}");

    let department: &'static str = "Software Engineering";
    let school: &'static str = "School of Science and Technology";
    println!("Department: {department}, \nSchool: {school}");
    
}
