use richterm::text;
use richterm::print;

fn main(){
	print(&[
		&text("richterm", "fg:dark_blue bg:indian_red1 eff:b,i,u,blink,s"),
		&text("\n", ""),
		&text("richterm", "fg:rgb(0,255,0) bg:rgb(0,0,255) eff:b,i,u,blink,s"),
		&text("richterm", ""),
		&text("\n", ""),
	]);
}