use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    
    /*
    let mut file = std::fs::File::create("data.txt").expect("Failed to create file");
    file.write_all("Welcome".as_bytes()).expect("Failed to write");
    */

    let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
    file.write_all("\nHello Class".as_bytes()).expect("write failed");// I created a file and write a byte sequence representing "Hello Class"
    file.write_all("\nThis is the appendage to the document.".as_bytes()).expect("write failed");

    println!("file append success");

}
