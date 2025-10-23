use std::io;

fn main() {
    println!("Roots of a quadratic equation");

    println!("What is a?");
    let mut a=String::new();
    io::stdin().read_line(&mut a).expect("Not a valid string");
    let a:f32=a.trim().parse().expect("Not a valid number");
   
    println!("What is b?");
    let mut b=String::new();
    io::stdin().read_line(&mut b).expect("Not a valid string");
    let b:f32=b.trim().parse().expect("Not a valid number");

    println!("What is c?");
    let mut c=String::new();
    io::stdin().read_line(&mut c).expect("Not a valid string"); 
    let c:f32=c.trim().parse().expect("Not a valid number");

    let d=b.powf(2.0)-4.0*a*c;
    
    if d > 0.0{
    	let r_one=(-b + d.sqrt() )/( 2.0*a );
        let r_two= (-b - d.sqrt() )/( 2.0*a );
    	println!("There are two real roots");
    	println!("The roots are {} and {}",r_one,r_two );
    }

    else if d == 0.0{
    	let root=-b/(2.0*a); // since sqrt of 0 is 0
    	println!("There is exactly one root");
    	println!("The root is {}",root );
    }

    else {
    	println!("There are no real roots");

    }
    
    
    
}
