fn main() {
    let fullname = "Alex Victor Lawrence";
    let department = "Software Engineering";
    let uni = "Pan-Atlantic University";

    let mut school = "School of science".to_string();
    //push str
    school.push_str(" and Technology");

    println!("My name is {fullname}");
    //check length
    println!("The length of my full name is {}", fullname.replace(" ", "").len());
    println!("I am a student of {department} Department");
    println!("{school}");
    println!("{uni}");
}
