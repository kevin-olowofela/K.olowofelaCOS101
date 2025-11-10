fn main() {
  let num:i32 = 5;
  mutate_num_to_zero(num);
}

fn mutate_num_to_zero(mut paran_num:i32){
	paran_num = paran_num*0;
	println!("paran_num value is :{}",paran_num );
}