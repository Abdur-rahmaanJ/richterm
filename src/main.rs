use richterm::text;
use richterm::print;
use richterm::track;

use std::{thread, time};


fn main(){

    // let vec = vec![text("richterm", "fg:dark_blue bg:indian_red1 eff:b,i,u,blink,s")];
    // print(vec);

    // let arr = [text(" supports ", "")];
    // print(arr);

    // print([
    //     text(" many ", "bg:sea_green2 fg:black eff:i"),
    //     text(" features ", ""),
    //     text(" out of the box", "bg:yellow fg:black"),
    //     text("\n", "")
    //     ]);

    // print([
    //     text("Even", ""),
    //     text(" some ", "eff:b"),
    //     text("emojis like :+1: :apple: :bar_chart: :airplane_departure: :baguette_bread: :minibus:", ""),
    //     text("\n", "")
    // ]);

    let mut sum = 0;
    for item in track(13, "Downloading ..."){
        let ten_millis = time::Duration::from_millis(500);
        let now = time::Instant::now();

        thread::sleep(ten_millis);
        sum = sum + 1;
    }

    println!("{:?}", sum);
}