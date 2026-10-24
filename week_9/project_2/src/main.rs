use std::io::Write;

fn main() {
    students_information();
}

fn students_information(){

    let variables: Vec<[&str ; 4]> = vec![
        [
            "Student Name", "Matric. Number", "Department", "Level"
        ],
    ];

    let students_detail: Vec<(&str, &str, &str, u32)> = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EE11020202", "Electrical", 200),
        ("Blance Edemoh", "MEE10202001", "Mechanical", 100),
    ];

    let mut file = std::fs::File::create("Students_detail.csv").expect("Failed to create the file");

    for variable in &variables {
        writeln!(file, "{}, {}, {} {}", variable[0], variable[1], variable[2], variable[3]).expect("Failed to save to file");
    }

    for student in &students_detail {
        writeln!(file, "{}, {}, {}, {}", student.0, student.1, student.2, student.3).expect("Failed to save to file");
    }

    println!("The student's detials has been saved to file!\n");

    println!("{} Below are the list of students and their respective detials {}\n", "=".repeat(9), "=".repeat(9));

    for (index, student) in students_detail.iter().enumerate() {
        println!("{}. Student Name is {}\n   Matric. Number is {}\n   Department is {}\n   In {} level\n", index + 1, student.0, student.1, student.2, student.3);
    }
}