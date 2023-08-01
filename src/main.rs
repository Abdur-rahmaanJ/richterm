use richterm::print;
use richterm::progress;
use richterm::text;
use richterm::track;

use std::{thread, time};

fn main() {
    let vec = vec![text(
        "richterm",
        "fg:dark_blue bg:indian_red1 eff:b,i,u,blink,s",
    )];
    print(vec);

    let arr = [text(" supports ", "")];
    print(arr);

    print([
        text(" many ", "bg:sea_green2 fg:black eff:i"),
        text(" features ", ""),
        text(" out of the box", "bg:yellow fg:black"),
        text("\n", ""),
    ]);

    print([
        text("Even", ""),
        text(" some ", "eff:b"),
        text(
            "emojis like :+1: :apple: :bar_chart: :airplane_departure: :baguette_bread: :minibus:",
            "",
        ),
        text("\n", ""),
    ]);

    for _item in track(5, "Downloading ...") {
        let ten_millis = time::Duration::from_millis(5);
        thread::sleep(ten_millis);
    }

    let mut tasks = progress();
    let task1 = tasks.add_task("Descr1", 50);
    let task2 = tasks.add_task("Descr2", 50);
    let task3 = tasks.add_task("Descr3", 50);

    while !tasks.finished {
        tasks.update(task1.clone(), 10.0);
        tasks.update(task2.clone(), 5.0);
        tasks.update(task3.clone(), 6.0);
    }
}
