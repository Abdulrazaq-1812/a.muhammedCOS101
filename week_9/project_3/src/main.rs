use std::io::Write;

fn main() {
    
    let (header, commissioners_detail) = convicted_ministers_header();

    let mut file = std::fs::File::create("The EFCC Merge.csv").expect("Failed to create a file");

    for  commissioner in &commissioners_detail{
        let _saved_file = writeln!(file, "{}, {}, {}", commissioner[0], commissioner[1], commissioner[2]);
    }

    for element in &header{
        let _saved_file = writeln!(file, "{}, {}, {}", element[0], element[1], element[2]);
    }

    println!("The record of the convicted minister has be saved to file");
    
}

fn convicted_ministers_header() -> (Vec<[&'static str ; 3]>, Vec<[&'static str ; 3]>) {
    let header: Vec<[&str; 3]> = vec![
        ["Commissioner", "Ministry", "Zone"]
    ];
    let commissioners_detail: Vec<[&str; 3]> = vec![

        ["Aigbogun Alamba Dauda", "Internal Affairs", "South West"],
        ["Murtala Afeez Bendu", "Justice", "North East"],
        ["Okorocha Calistus Ogbona", "Defense", "South South"],
        ["Adewale Jimoh Akanbe", "Power & Steel", "South West"],
        ["Osazuwa Faith Etieye", "Petroleum", "South East"]

    ];

    (header, commissioners_detail)
}
