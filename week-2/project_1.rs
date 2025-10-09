fn main() {
	let p:f64=520_000_000;
	let r:f64=10;
	let n:f64=5;

	let A=p * ( r / 100 )*n;
	 println!("Amount is {}","A" );
	let I=A-p;
	 println!("Compound interest is {}","I" );
	();
}