use std::io::Read;
use std::io;

fn main() {
    
    let mut file1 = std::fs::File::open("globacom_db.sql").unwrap();
    let mut file2 = std::fs::File::open("customer_table.sql").unwrap();
    let mut file3 = std::fs::File::open("employees.sql").unwrap();
    let mut file4 = std::fs::File::open("department.sql").unwrap();
    let mut file5 = std::fs::File::open("project.sql").unwrap();
    let mut file6 = std::fs::File::open("data_plan.sql").unwrap();


    let mut content1 = String::new();
    let mut content2 = String::new();
    let mut content3 = String::new();
    let mut content4 = String::new();
    let mut content5 = String::new();
    let mut content6 = String::new();

    file1.read_to_string(&mut content1).unwrap();
    file2.read_to_string(&mut content2).unwrap();
    file3.read_to_string(&mut content3).unwrap();
    file4.read_to_string(&mut content4).unwrap();
    file5.read_to_string(&mut content5).unwrap();
    file6.read_to_string(&mut content6).unwrap();

    println!("\nWelcome to Globacom Ltd!\nIt's a pleasure to be part of the management of this prestigious organization");

    println!("\nWhat is your position in the management?\n\n1. Administrator\n2. Project Manager\n3. Employee\n4. Customer\n5. Vendor\n");

    loop {

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().eq_ignore_ascii_case("Administrator") {
            println!("\nCongratulation! As an administrator you have access to all the tables in globacom database.\nBelow are the tables\n\n");
            println!("{}", content1);
            break;

        }else if input.trim().eq_ignore_ascii_case("project manager") {
            println!("\nCongratulation! As a project manager you only have access to the project table of Globacom database. \nBelow is the project table\n\n");
            println!("{}", content5);
            break;

        }else if input.trim().eq_ignore_ascii_case("Employee") {
            println!("\nCongratulation! As an employee you only have access to the employee table of Globacom database. \nBelow is the employee table\n\n");
            println!("{}", content3);
            break;

        }else if input.trim().eq_ignore_ascii_case("Customer") {
            println!("\nCongratulation! As a customer you only have access to the customer table of Globacom database. \nBelow is the customer table\n\n");
            println!("{}", content2);
            break;

        }else if input.trim().eq_ignore_ascii_case("Vendor") {
            println!("\nCongratulation! As a vendor you only have access to the data_plan table of Globacom database. \nBelow is the data_plan table\n\n");
            println!("{}", content6);
            break;

        }else{
            println!("Please, enter a valid position");
            continue;
        }
    }

}
