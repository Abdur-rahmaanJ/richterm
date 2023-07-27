use richterm::text;
use richterm::print;

fn main(){
	let msg = &[
		&text("richterm", "fg:violet bg:violet b:0 eff:blink"),
		&text("\nwefwef", "")
		];
	print(msg) 
}
