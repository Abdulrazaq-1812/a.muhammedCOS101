//use std::io::Write;
use std::io::Read;

fn main() {
  /*  
    let mut file = std::fs::File::create("welcome_message.txt").expect("Failed to create file");
    file.write_all("Welcome to this file".as_bytes()).expect("Failed to write content");
  */
    let mut file = std::fs::File::open("welcome_message.txt").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
