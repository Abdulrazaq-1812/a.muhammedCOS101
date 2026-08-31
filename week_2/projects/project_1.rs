fn main() {
	let p: f64 = 520_000_000.00;
	let t: f64 = 5.0;
	let r: f64 = 10.0;

	//Amount
	let x = r / 100.0;
	let a = p * (1.0 + x).powf(t);

	//Compount interest
	let ci = a - p;

	println!("The compound interest for 5 years at 10% per annum  on a loan of N520M is {}",ci);
}