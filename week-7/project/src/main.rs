use std::io;
fn trapezium(){
	let mut input_1=String::new();
	println!("What is the height");
	io::stdin().read_line(&mut input_1).expect("Invalid string");
	let height:f32= match input_1.trim().parse(){
		Ok(d)=>{
			if d <=0.0{
				println!("Number must be greater than 0");
				return;
			}
			d
		},
		Err(_)=>{
			println!("Value must be a number ");
			return;
			},
		};

    let mut input_2=String::new();
    println!("What is the first base?");
	io::stdin().read_line(&mut input_2).expect("Invalid string");
    let base1:f32=match input_2.trim().parse(){
        	Ok(b1)=>{
        		if b1 <=0.0{
				println!("Number must be greater than 0");
				return;
			}
				b1
			},
			Err(_)=>{
				println!("Value must be a number");
				return;
				
			},

        	};
        
	let mut input_3=String::new();
	println!("What is the second base?");
	io::stdin().read_line(&mut input_3).expect("Invalid string");
	let base2:f32=match input_3.trim().parse(){
		Ok(b2)=>{
			if b2 <=0.0{
				println!("Number must be greater than 0");
				return;
			}
			b2
		},
		Err(_)=>{
			println!("Value must be a number");
			return;
			
		},
	};
	let area=(height/2.0)*(base1 + base2);
	println!("Area of the trapezium is {}",area );


}
fn rhombus(){
	let mut input_1=String::new();
	println!("What is the first diagonal");
	io::stdin().read_line(&mut input_1).expect("Invalid string");
	let diagonal_1:f32=match input_1.trim().parse(){
		Ok(d1)=>{
			if d1 <=0.0{
				println!("Number must be greater than 0");
				return;
			}
			d1
		}
	    Err(_)=>{
	    	println!("Value must be a number");
	    	return;
	    	
	    },
	};
	let mut input_2=String::new();
	println!("What is the second diagonal");
	io::stdin().read_line(&mut input_2).expect("Invalid string");
	let diagonal_2:f32=match input_2.trim().parse(){
		Ok(d2)=>{
			if d2 <=0.0{
				println!("Number must be greater than 0");
				return;
			}
			d2
		},
	    Err(_)=>{
	    	println!("Value must be a number");
	    	return;
	    	
	    },
	};

	let area=0.5*diagonal_1*diagonal_2;
	println!("Area of rhombus is {}",area );


}
fn parallelogram(){
	let mut input_1=String::new();
	println!("What is the base");
	io::stdin().read_line(&mut input_1).expect("Invalid string");
	let base:f32=match input_1.trim().parse(){
		Ok(b)=>{
			if b <=0.0{
				println!("Number must be greater than 0");
				return;
			}
			b
		}
	    Err(_)=>{
	    	println!("Value must be a number");
	    	return;
	    	
	    },
	};
	let mut input_2=String::new();
	println!("What is the altitude");
	io::stdin().read_line(&mut input_2).expect("Invalid string");
	let altitude:f32=match input_2.trim().parse(){
		Ok(a)=>{
			if a <=0.0{
				println!("Number must be greater than 0");
				return;
			}
			a
		},
	    Err(_)=>{
	    	println!("Value must be a number");
	    	return;
	    	
	    },
	};

	let area = base*altitude;
	println!("Area of parallelogram is {}",area );
}
fn cube(){
	let mut input_1=String::new();
	println!("What is the length of the side");
	io::stdin().read_line(&mut input_1).expect("Invalid string");
	let length:f32=match input_1.trim().parse(){
		Ok(l)=>{
			if l <=0.0{
				println!("Number must be greater than 0");
				return;
			}
			l
		},
	    Err(_)=>{
	    	println!("Value must be a number");
	    	return;
	    	
	    },
	};
	let area = 6.0*length.powf(2.0);
	println!("Area of cube is {}",area );

}
fn cylinder(){
	let mut input_1=String::new();
	println!("What is the radius");
	io::stdin().read_line(&mut input_1).expect("Invalid string");
	let radius:f32=match input_1.trim().parse(){
		Ok(r)=>{
			if r <=0.0{
				println!("Number must be greater than 0");
				return;
			}
			r
		}
	    Err(_)=>{
	    	println!("Value must be a number");
	    	return;
	    	
	    },
	};
	let mut input_2=String::new();
	println!("What is the second diagonal");
	io::stdin().read_line(&mut input_2).expect("Invalid string");
	let height:f32=match input_2.trim().parse(){
		Ok(h)=>{
			if h <=0.0{
				println!("Number must be greater than 0");
				return;
			}
			h
		},
	    Err(_)=>{
	    	println!("Value must be a number");
	    	return;
	    	
	    },
	};
	let volume=3.14*radius.powf(2.0)*height;
	println!("volume of cylinder is {}",volume);

}
fn main() {
println!("Welcome to the geometry calculator");
	let menu=[
	("T","Area of trapezium"),
	("R","Area of rhombus"),
	("P","Area of parallelogram"),
	("CU","Area of cube"),
	("CY","Volume of cylinder"),
	];
	for (code,calculate) in menu.iter(){
		println!("code ={} calculation={}",code,calculate );
	}
	let mut input_1=String::new();
	println!("Enter code to calculate");
	io::stdin().read_line(&mut input_1).expect("Invalid string");
	let code1:String=input_1.to_uppercase().trim().to_string();
	if code1 =="T" || code1=="R"|| code1=="P"||code1=="CU"||code1=="CY"{
		println!("Valid code,Running program....");
	}
	else{
		println!("Invalid code");
		return;
	}
	if code1=="T"{
	trapezium();
}   
else if code1=="R"{
	rhombus();
}
else if code1=="P"{
	parallelogram();
}
else if code1=="CU"{
	cube();
}
else{
	cylinder();
};
}
