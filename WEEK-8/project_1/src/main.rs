use std::io;

fn main() {
    let professions = vec![
        "Public Servant",
        "Office Admin",
        "Academic",
        "Lawyer",
        "Teacher",
    ];

    let aps_levels = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
        "EL2 10-13",
        "SES",
    ];

    let office_admin = vec![
        ("Intern", "APS 1-2"),
        ("Administrator", "APS 3-5"),
        ("Senior Administrator", "APS 5-8"),
        ("Office Manager", "EL1 8-10"),
        ("Director", "EL2 10-13"),
        ("CEO", "SES"),
    ];

    let academic = vec![
        ("Research Assistant", "APS 3-5"),
        ("PhD Candidate", "APS 5-8"),
        ("Post-Doc Researcher", "EL1 8-10"),
        ("Senior Lecturer", "EL2 10-13"),
        ("Dean", "SES"),
    ];

    let lawyer = vec![
        ("Paralegal", "APS 1-2"),
        ("Junior Associate", "APS 3-5"),
        ("Associate", "APS 5-8"),
        ("Senior Associate 1-2", "EL1 8-10"),
        ("Senior Associate 3-4", "EL2 10-13"),
        ("Partner", "SES"),
    ];

    let teacher = vec![
        ("Placement", "APS 1-2"),
        ("Classroom Teacher", "APS 3-5"),
        ("Senior Teacher", "APS 5-8"),
        ("Leading Teacher", "EL1 8-10"),
        ("Deputy Principal", "EL2 10-13"),
        ("Principal", "SES"),
    ];

    println!("========================================");
    println!("           APS LEVEL CHECKER");
    println!("========================================");

    println!("\nAvailable professions:");

    for profession in &professions {
        println!("- {}", profession);
    }

    println!("\nAvailable APS levels:");

    for level in &aps_levels {
        println!("- {}", level);
    }

    println!("\nEnter your profession:");

    let mut profession_input = String::new();

    io::stdin()
        .read_line(&mut profession_input)
        .expect("Failed to read input");

    let profession_input = profession_input.trim().to_lowercase();

    println!("\nAvailable job titles:");

    match profession_input.as_str() {
        "public servant" => {
            println!("Public Servant levels are listed above.");
            println!("Please select one of the APS levels listed above.");
            println!("Enter an APS level:");
        }

        "office admin" => {
            for (job, level) in &office_admin {
                println!("- {} ({})", job, level);
            }
        }

        "academic" => {
            for (job, level) in &academic {
                println!("- {} ({})", job, level);
            }
        }

        "lawyer" => {
            for (job, level) in &lawyer {
                println!("- {} ({})", job, level);
            }
        }

        "teacher" => {
            for (job, level) in &teacher {
                println!("- {} ({})", job, level);
            }
        }

        _ => {
            println!("Invalid profession.");
            return;
        }
    }

    let mut job_input = String::new();

    io::stdin()
        .read_line(&mut job_input)
        .expect("Failed to read input");

    let job_input = job_input.trim().to_lowercase();

    let mut found = false;

    match profession_input.as_str() {
        "public servant" => {
            for level in &aps_levels {
                if level.to_lowercase() == job_input {
                    println!("\nProfession: Public Servant");
                    println!("APS Level: {}", level);
                    found = true;
                }
            }
        }

        "office admin" => {
            for (job, level) in &office_admin {
                if job.to_lowercase() == job_input {
                    println!("\nProfession: Office Admin");
                    println!("Job title: {}", job);
                    println!("APS Level: {}", level);
                    found = true;
                }
            }
        }

        "academic" => {
            for (job, level) in &academic {
                if job.to_lowercase() == job_input {
                    println!("\nProfession: Academic");
                    println!("Job title: {}", job);
                    println!("APS Level: {}", level);
                    found = true;
                }
            }
        }

        "lawyer" => {
            for (job, level) in &lawyer {
                if job.to_lowercase() == job_input {
                    println!("\nProfession: Lawyer");
                    println!("Job title: {}", job);
                    println!("APS Level: {}", level);
                    found = true;
                }
            }
        }

        "teacher" => {
            for (job, level) in &teacher {
                if job.to_lowercase() == job_input {
                    println!("\nProfession: Teacher");
                    println!("Job title: {}", job);
                    println!("APS Level: {}", level);
                    found = true;
                }
            }
        }

        _ => {
            println!("Invalid profession.");
        }
    }

    if !found {
        println!("\nNo matching job title or APS level was found.");
        println!("Please check your selection and try again.");
    }

    println!("========================================");
}