fn main() {
	let t:f64=2.0*450_000.00;
	let m:f64=1.0*1_500_000.00;
	let hp:f64=3.0*750_000.00;
	let dell:f64=3.0*2_850_000.00;
	let acer:f64=1.0*250_000.00;
	let s=t+m+hp+dell+acer;
	println!("Sum of sales is {} ",s );
	let a=s/10.0;
	println!("average sale cost is {}",a );

}