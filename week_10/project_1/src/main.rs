struct Brand<'a> {
    name: &'a str,
    quantity: u32,
    price_each: u32
}

impl<'a> Brand<'a> {
    fn total_cost(&self) -> u32 {
        self.price_each * 3
    }
}

fn main() {
    let hp = Brand {
        name: "HP",
        quantity: 10,
        price_each: 650_000
    };

    let ibm = Brand {
        name: "IBM",
        quantity: 6,
        price_each: 755_000
    };

    let toshiba = Brand {
        name: "Toshiba",
        quantity: 10,
        price_each: 550_000
    };

    let dell = Brand {
        name: "Dell",
        quantity: 4,
        price_each: 850_000
    };

    let total_cost = hp.total_cost() + ibm.total_cost() + toshiba.total_cost() + dell.total_cost();

    println!("The total cost for the purchase of 3 laptops from each brand is N{}", total_cost);
}
