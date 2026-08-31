fn main() {
	let p: f64 = 1000.0;
	let r: f64 = 1.0;
	let t: f64 = 2.0;

	//Simple interest
	let a = p * (1.0 + ((r * t)/100.0));
	println!("Amount is {}", a);
	let si = a - p;
	println!("Simple interest is {}", si);
}