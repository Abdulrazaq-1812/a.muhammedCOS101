use std::io;

fn main() {
    println!(
        "
        Hey, there!
        Welcome to today's interview
        "
    );

    let mut interviewee_info: Vec<(String, u8)> = Vec::new();

    loop {
        println!("\nDo you want to add an interviewee? (yes or no)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid input");

        if input.trim().eq_ignore_ascii_case("no") {
            break;
        } else if input.trim().eq_ignore_ascii_case("yes") {
            println!("Please enter your full name:");

            let mut input1 = String::new();
            io::stdin()
                .read_line(&mut input1)
                .expect("Invalid input");

            let name = input1.trim().to_string();

            println!("Please enter your years of programming experience:");

            let mut input2 = String::new();
            io::stdin()
                .read_line(&mut input2)
                .expect("Invalid input");

            let year: u8 = match input2.trim().parse() {
                Ok(value) => {
                    if value == 0 {
                        println!("Please enter a positive whole number.");
                        continue;
                    }
                    value
                }
                Err(_) => {
                    println!("Invalid input. Please enter a whole number.");
                    continue;
                }
            };

            // Add the new interviewee to the collection.
            interviewee_info.push((name, year));

            // Print the entire collection so far.
            println!("\nAll interviewees recorded so far:");

            for (index, (name, year)) in interviewee_info.iter().enumerate() {
                println!(
                    "{}. Name: {}, Experience: {} years",
                    index + 1,
                    name,
                    year
                );
            }
        } else {
            println!("Please enter yes or no.");
        }
    }

    // Print the final collection.
    println!("\n========== FINAL COLLECTION ==========");

    if interviewee_info.is_empty() {
        println!("No records found.");
    } else {
        for (index, (name, year)) in interviewee_info.iter().enumerate() {
            println!(
                "{}. Name: {}, Experience: {} years",
                index + 1,
                name,
                year
            );
        }
    }

    // Find the interviewee with the most experience.
    match interviewee_info.iter().max_by_key(|person| person.1) {
        Some(person) => {
            println!(
                "\nThe most experienced interviewee is {} with {} years of experience.",
                person.0, person.1
            );
        }
        None => {
            println!("No interviewees are available to compare.");
        }
    }
}
