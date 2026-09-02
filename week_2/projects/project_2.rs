fn main(){
	let toshiba_a = 450_000.00;
	let mac_a = 1_500_000.00;
	let hp_a = 750_000.00;
	let dell_a = 2_850_000.00;
	let acer_a = 250_000.00;

	let toshiba_q = 2.0;
	let mac_q = 1.0;
	let hp_q = 3.0;
	let dell_q = 3.0;
	let acer_q = 1.0;
    
    let total_q = toshiba_q + mac_q + hp_q + dell_q + acer_q;
	let sum = toshiba_a + mac_a + hp_a + dell_a + acer_a;
	let average = sum / total_q;

	println!("The sum of the sales record is {}", sum);
	println!("The average of the sales record is {:.2}", average);
	} 