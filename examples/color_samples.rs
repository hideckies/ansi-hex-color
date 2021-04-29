use ansi_hex_color::*;

fn main() {
	let txt_white = colored("#FFFFFF", "", "Hello");
	let txt_b_white = colored("#999999", "#FFFFFF", "Hello");
	let txt_red = colored("#FF0000", "", "Hello");
	let txt_b_red = colored("", "#FF0000", "Hello");
	let txt_blue = colored("#0024c0", "", "Hello");
	let txt_b_blue = colored("a", "#0000Dd", "Hello");
	
	println!("{}", txt_white);
	println!("{}", txt_b_white);
	println!("{}", txt_red);
	println!("{}", txt_b_red);
	println!("{}", txt_blue);
	println!("{}", txt_b_blue);
}