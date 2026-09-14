use std::io;

/*
Rust program to read the height of a program
and then print if the person is tall, dwarf, 
or average height person 
*/

fn main() {
    let mut input = String::new();

    println!("\nEnter your height (in centimeters): ");
    io::stdin().read_line(&mut input).expect("Invalid string");
    let height:f32 = input.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <= 170.0
    {
        println!("You are of average height person");
    }

    else if height > 170.0 && height <= 195.0
    {
        println!("You are tall");
    }

    else if height < 150.0 && height > 100.0
    {
        println!("You are dwarf");
    }

    else
    {
        println!("Abnormal height");
    }
}
