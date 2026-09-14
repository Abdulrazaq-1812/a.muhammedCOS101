// Rust program to count numbers

use std::io;

fn main() {
    
    let mut input1 = String::new();

    println!("Enter a number");
    io::stdin().read_line(&mut input1).expect("Invalid string");
    let mut num:i32 = input1.trim().parse().expect("Failed to input");

    while num < 10 {
        println!("Inside loop number value is {}", num);
        num+=1;
    }
        println!("Outside loop number value is {}", num);

}
