use raster::Color;
use std::str;

mod ansi;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn colored(foreground_hex_code: &str, background_hex_code: &str, txt: &str) -> String {
	let mut result_txt: String;
	
	let mut rgb_foreground: Color;
	let mut rgb_background: Color;
	
	let mut ansi_background: String;
	let mut ansi_foreground: String;
	
	// Validate hex
	let is_hex_foreground: bool = validate_hex(foreground_hex_code);
	let is_hex_background: bool = validate_hex(background_hex_code);
	
	if is_hex_foreground {
		foreground_hex_code.to_lowercase();
		rgb_foreground = Color::hex(foreground_hex_code).unwrap();
		ansi_foreground=
		ansi::FOREGROUND.to_string() + &rgb_foreground.r.to_string() + ";" + &rgb_foreground.g.to_string() + ";" + &rgb_foreground.b.to_string() + "m";
	} else {
		ansi_foreground = "".to_string();
	}
	if is_hex_background {
		rgb_background = Color::hex(background_hex_code).unwrap();
		ansi_background =
		ansi::BACKGROUND.to_string() + &rgb_background.r.to_string() + ";" + &rgb_background.g.to_string() + ";" + &rgb_background.b.to_string() + "m";
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
	
	let mut splited_target = target.chars();

	let mut char_0: String = splited_target.nth(0).unwrap().to_string();
	
	if &char_0 == "#" {
		is_hex = true;
		return is_hex;
	}
	
	is_hex
}