use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    let experience = loop{
        println!("Enter your level of experience(either Experienced or Not experienced): ");

        input1.clear();
        io::stdin().read_line(&mut input1).expect("Invalid input");
        
        match input1.trim() {
            "Experienced" => break "Experienced".to_string(),
            "Not experienced" => break "Not experienced".to_string(),
            _ => continue
        }
    };

    let age: u8 = loop {
        println!("Enter your age: ");

        input2.clear();
        io::stdin().read_line(&mut input2).expect("Invalid input");

        match input2.trim().parse::<u8>(){
            Ok(value) => break value,
            Err(_) => continue,
        }
    };

        if experience == "Experienced" && age >= 40 {

            println!("Your annual incentive from the criteria being provided is N1,560,000.00");

        } else if experience == "Experienced" && age >= 30 && age <=39 {

             println!("Your annual incentive from the criteria being provided is N1,480,000.00");

        } else if experience == "Experienced" && age < 28{

             println!("Your annual incentive from the criteria being provided is N1,300,000.00");

        } else if experience == "Not experienced"{

            println!("Your annual incentive from the criteria being provided is N100,000.00");

        } else{

            println!("You have no annual incentive!!");
        };
}
