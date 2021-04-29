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
	
	let mut splited_target = target.chars();

	let char_0: String = splited_target.nth(0).unwrap().to_string();
	
	if &char_0 == "#" {
		is_hex = true;
		return is_hex;
	}
	
	is_hex
}