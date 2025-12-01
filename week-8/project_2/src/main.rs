use std::io;
fn main() {
    let mut candidates: Vec<u32> = Vec::new();
    println!("How many candidates do you want to interview");
     let mut input_1 = String::new();
     io::stdin().read_line(&mut input_1).expect("invalid string");
     let array_len:u32 = match input_1.trim().parse(){
     	Ok(d)=>{
     		if d>0{
     			println!("value recorded");
     		}
     			else{
     				println!("Value must be a counting number");
     			}
     		d

     	},
     	Err(_)=>{
     		println!("invalid input");
     		return;
     	},
     };
     for i in 1..=array_len{
     	println!("Enter year of experience for candidate {}",i );
     	let mut input_2=String::new();
     	io::stdin().read_line(&mut input_2).expect("invalid string");
     	match input_2.trim().parse::<u32>(){
     		Ok(exp)=>{
     		       candidates.push(exp);
     				println!("Value recorded");
     				continue;
     		},
     		Err(_)=>{
     			println!("invalid details");
     			return;
     		},
     	};
     	

     };
     let max_exp=candidates.iter().max();
     println!("The max experience is {:?}",max_exp );
}
