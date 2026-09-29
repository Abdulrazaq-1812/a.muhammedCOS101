use std::io;

fn main() {
    println!("\t Welcome to Pan-Atlantic University Resturant");

    println!("ID \t FOOD TYPE \t \t PRICES(N) \n");

    println!("P \t Poundo Yam / Edinkaiko Soup \t 3,200");

    println!("F \t Fried Rice & Chicken \t \t 3,000");

    println!("A \t Amala & Ewedu Soup \t \t 2,500");

    println!("E \t Eba & Egusi Soup \t \t 2,000");

    println!("W \t White Rice & Stew \t \t 2,500");

    
    let (purchase, price) = loop{

        println!("Enter the ID of your choice of food: ");

        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Invalid ID");

        let food_id = input1.trim();

        match food_id {
            "F" => break (" Fried Rice & Chicken", 3000.00),
            "P" => break ("Poundo Yam / Edinkaiko Soup", 3200.00),
            "A" => break ("Amala & Ewedu Soup", 2500.00),
            "E" => break ("Eba & Egusi Soup", 2000.00),
            "W" => break ("White Rice & Stew", 2500.00),
            _ => {
                println!("Invalid food ID.");
                continue;
            }  
        }
    };


    let quan:f32 = loop{

        println!("Enter quantity ( a discount of 5% will be given if total cost is above N10,000: ");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");

        match input2.trim().parse(){
            Ok(number) => break number,
            Err(_) => {
                println!("Invalid quantity. Please enter a number.");
                continue;
            }
        }
    };

    println!("You choosed: {} for quantity: {}", purchase, quan);

    let mut total = quan * price;

    if total > 10_000.00 {
        total = total * 0.95;
        println!("The cost of your purchase is {}", total);
    } 
}