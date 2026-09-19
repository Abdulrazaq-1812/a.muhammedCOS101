use std::io;

fn main() {

        let mut input_1 = String::new();
        let mut input_2 = String::new();
        let mut input_3 = String::new();

        println!("Given below the format of a quadratic equation: \n ax² + bx + c");

    let a : f64 = loop {

        println!("Enter the value of 'a': ");

        input_1.clear();
        io::stdin().read_line(&mut input_1).expect("Invalid text/number");

        match input_1.trim().parse::<f64>(){
            Ok(val_a) => break val_a,
            Err(_) => continue,
        }
    };

        println!("Enter the value of 'b': ");
        io::stdin().read_line(&mut input_2).expect("Invalid text/number");
        let b:f64 = input_2.trim().parse().expect("Value of 'b' not valid");

        println!("Enter the value of 'c': ");
        io::stdin().read_line(&mut input_3).expect("Invalid text/number");
        let c:f64 = input_3.trim().parse().expect("Value of 'c' not valid");

        calculation(a,b,c);


}

fn calculation(a:f64, b:f64, c:f64){

    let val_det:f64 = (b * b) - (4.0 * a * c);

    let sqrt_det:f64 = (val_det).powf(0.5);

    let root_1:f64 = (- b - sqrt_det) / (2.0 * a);

    let root_2:f64 = (- b + sqrt_det) / (2.0 * a);

    println!("The first root of the quadratic equation is {:.2}", root_1); // :? is used to help keep the data type of the variable consistent with the result
    println!("The second root of the quadratic equationis {:.2}", root_2); // :? is used to help keep the data type of the variable consistent with the result

}