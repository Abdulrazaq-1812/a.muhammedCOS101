use::std::io::Write;

fn main() {
    
    let breweries_brands:Vec<[&str; 3]> = vec!
    [
        ["Lager", "Stout", "Non-Alcoholic"]
    ];

    let products:Vec<[&str; 3]> = vec!
    [
        ["33 Export", "Legend", "Maltina"],
        ["Desperados", "Turbo King", "Amstel Malta"],
        ["Goldberb", "Williams", "Malta Gold"],
        ["Gulder", " ", "Fayrouz"],
        ["Heineken", " ", " "],
        ["Star", " ", " "],
    ];

    let mut file = std::fs::File::create("Breweries_brands.csv").expect("Failed to create a file");

    for brand in &breweries_brands {
        writeln!(file, "{}, {}, {}", brand[0], brand[1], brand[2]).unwrap();
    }


    for product in &products {
        writeln!(file, "{}, {}, {}", product[0], product[1], product[2]).unwrap();
    }

    println!("\nDrinks categories has been saved to the file");
}

