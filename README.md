# richterm

![](https://github.com/Abdur-rahmaanJ/richterm/blob/stable/richterm.gif)


```rust
use richterm::print;
use richterm::text;
use richterm::track;
use richterm::progress;

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
    
    while !tasks.finished{
        tasks.update(task1.clone(), 10.0);
        tasks.update(task2.clone(), 5.0);
        tasks.update(task3.clone(), 6.0);
    } 
}

```


# Docs


### fg and bg colors

- rgb(111,111,111)
- black, red, green, yellow, blue, magenta, cyan, white, bright_black, bright_red, bright_green, bright_yellow, bright_blue, bright_magenta, bright_cyan, bright_white, grey0, gray0, navy_blue, dark_blue, blue3, blue1, dark_green, deep_sky_blue4, dodger_blue3, dodger_blue2, green4, spring_green4, turquoise4, deep_sky_blue3, dodger_blue1, green3, spring_green3, dark_cyan, light_sea_green, deep_sky_blue2, deep_sky_blue1, spring_green2, cyan3, dark_turquoise, turquoise2, green1, spring_green1, medium_spring_green, cyan2, cyan1, dark_red, deep_pink4, purple4, purple3, blue_violet, orange4, grey37, gray37, medium_purple4, slate_blue3, royal_blue1, chartreuse4, dark_sea_green4, pale_turquoise4, steel_blue, steel_blue3, cornflower_blue, chartreuse3, cadet_blue, sky_blue3, steel_blue1, pale_green3, sea_green3, aquamarine3, medium_turquoise, chartreuse2, sea_green2, sea_green1, aquamarine1, dark_slate_gray2, dark_magenta, dark_violet, purple, light_pink4, plum4, medium_purple3, slate_blue1, yellow4, wheat4, grey53, gray53, light_slate_grey, light_slate_gray, medium_purple, light_slate_blue, dark_olive_green3, dark_sea_green, light_sky_blue3, sky_blue2, dark_sea_green3, dark_slate_gray3, sky_blue1, chartreuse1, light_green, pale_green1, dark_slate_gray1, red3, medium_violet_red, magenta3, dark_orange3, indian_red, hot_pink3, medium_orchid3, medium_orchid, medium_purple2, dark_goldenrod, light_salmon3, rosy_brown, grey63, gray63, medium_purple1, gold3, dark_khaki, navajo_white3, grey69, gray69, light_steel_blue3, light_steel_blue, yellow3, dark_sea_green2, light_cyan3, light_sky_blue1, green_yellow, dark_olive_green2, dark_sea_green1, pale_turquoise1, deep_pink3, magenta2, hot_pink2, orchid, medium_orchid1, orange3, light_pink3, pink3, plum3, violet, light_goldenrod3, tan, misty_rose3, thistle3, plum2, khaki3, light_goldenrod2, light_yellow3, grey84, gray84, light_steel_blue1, yellow2, dark_olive_green1, honeydew2, light_cyan1, red1, deep_pink2, deep_pink1, magenta1, orange_red1, indian_red1, hot_pink, dark_orange, salmon1, light_coral, pale_violet_red1, orchid2, orchid1, orange1, sandy_brown, light_salmon1, light_pink1, pink1, plum1, gold1, navajo_white1, misty_rose1, thistle1, yellow1, light_goldenrod1, khaki1, wheat1, cornsilk1, grey100, gray100, grey3, gray3, grey7, gray7, grey11, gray11, grey15, gray15, grey19, gray19, grey23, gray23, grey27, gray27, grey30, gray30, grey35, gray35, grey39, gray39, grey42, gray42, grey46, gray46, grey50, gray50, grey54, gray54, grey58, gray58, grey62, gray62, grey66, gray66, grey70, gray70, grey74, gray74, grey78, gray78, grey82, gray82, grey85, gray85, grey89, gray89, grey93, gray93, 


### effects

- b - bold
- i - italic
- u - underline
- blink - blink
- d - dim
- r - reverse

### emojis

Try github-flavoured shortcodes

### track(5, "description")

Progressbar
