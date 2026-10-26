fn main() {
    // a list of now
    let x = vec![100, 200, 300];
    borrow_vector(&x); // passing reference

    println!("Printing the value from main() x[0] = {}", x[0]);
    println!("{}", "*".repeat(39));
}

fn borrow_vector(z: &Vec<i32>) {

    println!("{}", "*".repeat(39));
    println!("Inside print_vector function {:?} \n", z);
    println!("{}", "-".repeat(39));
}