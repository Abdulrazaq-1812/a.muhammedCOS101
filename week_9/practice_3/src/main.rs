use::std::io::Write;
use::std::fs;

fn main() {
    let mut _file = std::fs::File::create("data.txt");

    fs::remove_file("data.txt").expect("could not remove file");
    println!("file is removed"); 
}
