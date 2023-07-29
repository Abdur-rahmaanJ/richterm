use richterm::text;
use richterm::print;

fn main(){

    let vec = vec![text("richterm", "fg:dark_blue bg:indian_red1 eff:b,i,u,blink,s")];
    print(vec);

    let arr = [text(" supports ", "")];
    print(arr);

    print([
        text(" many ", "bg:sea_green2 fg:black eff:i"),
        text(" features ", ""),
        text(" out of the box ", "bg:yellow fg:black"),
        text("\n", "")
        ]);
}