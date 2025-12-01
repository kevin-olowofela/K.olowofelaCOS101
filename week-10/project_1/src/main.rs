struct Cost{
	hp:u32,
	ibm:u32,
	toshiba:u32,
	dell:u32
}
impl Cost{
	fn t_cost(&self)->u32{
		(self.hp+self.ibm+self.toshiba+self.dell)*3
	}
}
fn main() {
	let cost1=Cost{
		hp:650_000,
		ibm:755_000,
		toshiba:550_000,
		dell:850_000
	};
    println!("Total cost for 3 each is {}",cost1.t_cost());
}
