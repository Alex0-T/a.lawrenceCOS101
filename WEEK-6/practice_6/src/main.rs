fn main() {
    let n1 = "ELectrical".to_string();
    let n2 = " Electronic".to_string();
    let n3 = " Engineering".to_string();
    let n4 = n1 + &n2 + &n3; //n2 & n3 reference is passed

    //About Electrical/Electronic
    println!("The {n4} is informed by the aspiration to train electrical/electronis engineering professionals
in the areas of design, building and maintenance of electrical control systems");
    let w1 = "Computer".to_string();
    let w2 = " Science".to_string();
    let w3 = w1 + &w2; //w2 reference is passed
    println!();
    println!(
        "{w3} is aimed at developing competent, creative, innovative, entrepreneurial
and ethically-minded persons, capable of creating value in the diverse  fields of
Computer Science"
    );
}
