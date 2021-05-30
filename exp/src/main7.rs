
fn some_fn(){
	()
}
#![feature(never_type)]
fn main(){
	let a: () = some_fn();
	println!("This function returns and you cna see ");
	println!("{:?}",a);
	let x: ! = panic!("This call never returns.");
	println!("You will never see this line!");

}