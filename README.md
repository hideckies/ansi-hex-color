# ansi-hex-color

`anti-hex-color` is a library coloring the given text.  
Since you can specify the color with hex code, you can easily color it like CSS.

## Install

in Cargo.toml

```
[dependencies]
ansi_hex_color = "0.1.0"
```

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