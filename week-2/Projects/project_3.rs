fn main() {
	let p: f64 = 210000.0;
	let r: f64 = 5.0;
	let n: f64 = 3.0;

	let a = p * (1.0 - (r / 100.0)).powf(n);

	println!("Original value: #{:.2}", p);
	println!("Depreciation rate: {}% per annum", r);
	println!("Number of years: {}", n);
	println!("Value of TV after {} years: #{:.2}", n, a);
}