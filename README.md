# ansi-hex-color

## Usage

```
use ansi_hex_color;

fn main () {
	let foreground = "#FF000000";
	let background = "#004082";
	let txt = "Hello world.";
	
	let colored_txt = ansi_hex_color::colored(
		foreground, background, txt);
	
	println!("{}", );
}
```