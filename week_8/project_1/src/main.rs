use std::io;


fn main() {
    let public_services = vec!
    [
        "Office Admin",
        "Academic",
        "Lawyer",
        "Teacher",
    ];

    let office_ad_positions = vec! 
    [
        ("Intern", "APS 1-2"),
        ("Administrator", "APS 3-5"),
        ("Senior Administrator", "APS 5-8"),
        ("Office Manager", "EL1 8-10"),
        ("Director", "EL2 10-13"),
        ("CEO", "SES"),
    ];

    let academic_positions = vec! 
    [
        ("Research Assistant", "APS 3-5"), 
        ("PhD Candidate", "APS 5-8"),
        ("Post-Doc Researcher", "EL1 8-10"),
        ("Senior Lecturer", "EL2 10-13"),
        ("Dean", "SES"),
    ];

    let lawyer_positions = vec!
    [
        ("Paralegal", "APS 1-2"),
        ("Junior Associate", "APS 3-5"),
        ("Associate", "APS 5-8"),
        ("Senior Associate 1-2", "EL1 8-10"),
        ("Senior Associate 3-4", "EL2 10-13"),
        ("Partner", "SES"),
    ];

    let teacher_positions = vec! 
    [
        ("Placement", "APS 1-2"),
        ("Classroom Teacher", "APS 3-5"),
        ("Senior Teacher", "APS 5-8"),
        ("Leading Teacher", "EL1 8-10"),
        ("Deputy Principal", "EL2 10-13"),
        ("Principal", "SES"),
    ];

    let (service, choosen_position, ap_level) = loop {

        println!("What public service do you offer?");
            for public_service in &public_services 
            {
                println!("- {}", public_service);
            }


        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Invalid input");
        let selected_service = input1.trim();

            if public_services.eq_ignore_ascii_case(&selected_service){
                match selected_service.as_str() {
                    "Office Admin" => {

                        let (cs_position, a_level) = choose_position(&office_ad_positions);
                        break (selected_service, cs_position, a_level);
                    }

                     "Academic" => {

                        let (cs_position, a_level) = choose_position(&academic_positions);
                        break (selected_service, cs_position, a_level);
                    }

                     "Lawyer" => {

                        let (cs_position, a_level) = choose_position(&lawyer_positions);
                        break (selected_service, cs_position, a_level);
                    }

                     "Teacher" => {

                        let (cs_position, a_level) = choose_position(&teacher_positions);
                        break (selected_service, cs_position, a_level);
                    }
                    _ => {
                        println!("Invalid position entry. Try again");
                        continue;
                    }
                }
            }

    };

    println!("The public service you chose to offer is '{}' for the position of '{}'. Hence your APS level is '{}'", service, choosen_position, ap_level);
}


fn choose_position(positions: &[(&str, &str)]) -> (String, String) {
    loop {
        println!("Which position do you hold?");

        for (position, _ ) in positions {
            println!("- {}", position);
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input");

        let choice = input.trim();

       match positions.iter().find(|(position, _)| *position == choice) {
            Some((position, ap_level)) => {
                break (position.to_string(), ap_level.to_string());
            }

            None => println!("Invalid position. Try again."),
        }
    }
}