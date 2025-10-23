use std::io;

fn main() {
    println!("Annual incentives of employees");

    println!("Are you experienced?");
    let mut experienced=String::new();
    io::stdin().read_line(&mut experienced).expect("Not a valid string");
    let experienced:String=experienced.trim().parse().expect("Invalid answer");
    


    println!("How old are you");
    let mut age=String::new();
    io::stdin().read_line(&mut age).expect("Not a valid string");
    let age:i32=age.trim().parse().expect("Not a valid age");

    let amount_1:u32=1_560_00;
    let amount_2:u32=1_480_000;
    let amount_3:u32=1_300_000;
    let amount_4:u32=100_000;

    if experienced=="yes" && age>=40{
     println!("Incentive is N{} per month",amount_1 );
    }
    else if experienced=="yes" && age>=30 && age <40{
    	println!("Incentive is N{} per month",amount_2);
    }
    else if experienced=="yes" && age <28 && age >0{
    	println!("Incentive is N{} per month",amount_3 );
    }
    else if experienced=="no"{
    	println!("Incentive is N{} per month ",amount_4);
    }
    else {
    	println!("Invalid experience value and/or age.\nPlease try again");
    }
}
