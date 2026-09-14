use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Invalid String");

    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Invalid String");
    let age:u8 = input2.trim().parse().expect("Invalid number");

    if age >= 18 {
        println!("Your age is {}", age);
    } else {
        println!("Oops!!, your age is below the minimum age required to participate in the party");
    }
}
