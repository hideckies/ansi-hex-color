# ansi-hex-color

`anti-hex-color` is a Rust library which color (ANSI 256-color) the given text in terminal.  
Since you can specify the color with hex code, you can easily color it like CSS.

## Install

in Cargo.toml

```toml
[dependencies]
ansi-hex-color = "0.1.1"
```

## Usage

```rust
use ansi_hex_color;

fn main () {
	let foreground = "#FF0000";
	let background = "#004082";
	let txt = "Hello world";
	
	let colored_txt = ansi_hex_color::colored(
		foreground, background, txt);
	
	println!("{}", colored_txt);
}
```

## Default style

If you put "" or a string other than `hex code` in the first and second arguments, the text will be default style(white text, no background).

```rust
let colored_txt = ansi_hex_color::colored("", "", "This is a default style.");
```