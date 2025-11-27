use::std::io::Write;

fn main(){

	let categories = "Lager        Stout      Non-Alcoholic\n";
	let drinks     =  "33 Export    Legend      Maltina \n
	                  Desperados   Turbo king  Amstel Malta \n,
	                  Goldberg     Williams    Malta Gold \n,
	                  Gulder          -         Fayrouz \n,
	                  Heineken        -           - \n,
	                  Star            -           -\n";

	let mut file = std::fs::File::create("Drinks.txt").expect("create failed");
	file.write_all(categories.to_string().as_bytes()).expect("Write failed");
	file.write_all(drinks.to_string().as_bytes()).expect("write failed");
	println!("Data written to file");
}
