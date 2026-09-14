/*
Rust program to calculate the area of
triangle for a given base and height
*/

use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the base of a triangle");
    io::stdin().read_line(&mut input1).expect("Invalid string");
    let b:f32 = input1.trim().parse().expect("Invalid value for the base");

    println!("Enter the height of a triangle");
    io::stdin().read_line(&mut input2).expect("Invalid string");
    let h:f32 = input2.trim().parse().expect("Invalid value for the height");

    if b > 0.0 && h > 0.0 { 
        let area:f32 = (b * h) / 2.0;
        println!("The area of the triangle is {}cm", area);
    } else{
        println!("Invalid input(s)");
    };
}
