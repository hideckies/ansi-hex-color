//! A library which color text from hex code.
//!
//! ## Usage
//!
//! ```rust
//!	use ansi_hex_color;
//!
//! fn main() {
//! 	let foreground = "#FF0000";
//!		let background = "#004082";
//! 	let txt = "Hello world";
//1
//! 	let colored_txt = ansi_hex_color::colored(
//! 		foreground, background, txt);
//!
//! 	println!("{}", colored_txt);
//! }
//! ```

use raster::Color;
use regex::Regex;
use std::str;

mod ansi;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn colored(foreground_hex: &str, background_hex: &str, txt: &str) -> String {
	let result_txt: String;
	
	let rgb_foreground: Color;
	let rgb_background: Color;
	
	let ansi_background: String;
	let ansi_foreground: String;
	
	// Validate hex
	let is_hex_foreground: bool = validate_hex(foreground_hex);
	let is_hex_background: bool = validate_hex(background_hex);
	
	if is_hex_foreground {
		foreground_hex.to_lowercase();
		rgb_foreground = Color::hex(foreground_hex).unwrap();
		ansi_foreground =
			ansi::FOREGROUND.to_string() +
			&rgb_foreground.r.to_string() + ";" +
			&rgb_foreground.g.to_string() + ";" +
			&rgb_foreground.b.to_string() + "m";
	} else {
		ansi_foreground = "".to_string();
	}
	if is_hex_background {
		background_hex.to_lowercase();
		rgb_background = Color::hex(background_hex).unwrap();
		ansi_background =
			ansi::BACKGROUND.to_string() +
			&rgb_background.r.to_string() + ";" +
			&rgb_background.g.to_string() + ";" +
			&rgb_background.b.to_string() + "m";
	} else {
		ansi_background = "".to_string();
	}
	
	result_txt = ansi_background.to_string() + &ansi_foreground.to_string() + &txt + &(ansi::RESET);
	result_txt
}

fn validate_hex(target: &str) -> bool {
	let mut is_hex: bool = false;
	
	if target == "" {
		is_hex = false;
		return is_hex;
	}
	if target.len() != 7 {
		is_hex = false;
		return is_hex;
	}
	
	let splited_target = target.chars();
	
	// Regex
	let re = Regex::new(r"[0-9a-f]+").unwrap();
	
	// The counter for loop
	let mut i: u8 = 0;
	
	for c in splited_target {
		let c_string = c.to_string().to_lowercase();
		
		if i == 0 { 
			if &c_string == "#" {
				is_hex = true;
			} else {
				is_hex = false;
				return is_hex;
			}
		}
		else {
			if re.is_match(&c_string) {
				is_hex = true;
			} else {
				is_hex = false;
				return is_hex;
			}
		}
		
		i += 1;
	}
	
	is_hex
}