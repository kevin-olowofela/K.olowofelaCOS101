use std::io;

fn main() {
	let p:f32=3200.0;
	let f:f32=3000.0;
	let a:f32=2500.0;
	let e:f32=2000.0;
	let w:f32=2500.0;
	println!("Welcome to kevin's restaurant\n Here is the menu");
	println!("       Item                Code  Price");
    println!("Poundo Yam/Edinkaiko Soup   p     N{}",p);
    println!("Fried rice & Chicken        f     N{}",f );
    println!("Amala & Ewedu Soup          a     N{}",a );
    println!("Eba & Egusi Soup            e     N{}",e );
    println!("White Rice & Stew           w     N{}",w );

   println!("Input code:");
   let mut code= String::new();
   io::stdin().read_line(&mut code).expect("Invalid string");
   let code:String=code.to_lowercase().trim().parse().expect("Invalid code");

   println!("Input quantity:");
   let mut qty=String::new();
   io::stdin().read_line(&mut qty).expect("Invalid string");
   let qty:f32=qty.trim().parse().expect("Invalid quantity");

   let mut tp:f32=0.0;

   if code=="p" && qty >0.0{
   	 tp=p*qty;
   	 println!("Total cost is N{}",tp );
   }
   else if code=="f" &&qty >0.0{
   	tp=f*qty;
   	println!("Total cost is N{}",tp );
   }
   else if code=="a" &&qty >0.0{
   	 tp=a*qty;
   	 println!("Total cost is N{}",tp );
   }
   else if code=="e" &&qty >0.0{
   	tp=e*qty;
    println!("Total cost is N{}",tp );
   }
   else if code=="w" &&qty >0.0{
   	 tp=w*qty;
   	 println!("Total cost is N{}",tp );
   }
   else{
   	println!("Invalid code or quantity ");
   }

   let discount_price:f32=tp-(0.05*tp);
   if tp>10000.0{
   	let new_price:f32=discount_price;
   	println!("Discounted Total cost is N{}",new_price);
   }
}

