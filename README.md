# richterm


```rust
use richterm::text;
use richterm::print;

fn main(){
	let msg = &[
		&text("richterm", "fg:dark_blue bg:indian_red1 b:0 eff:blink"),
		];
	print(msg) 
}
```