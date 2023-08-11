mod colors;
mod emojis;
mod uuid;
use regex::escape;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Write};
use terminal_size::{Width, Height, terminal_size};

pub fn print<T: AsRef<[U]>, U: std::fmt::Debug + std::fmt::Display>(data: T) {
    let slice: &[U] = data.as_ref();

    for item in slice {
        // Process the elements here
        print!("{}", item);
        io::stdout().flush().unwrap();
    }
}

// Function to replace emoji shortcodes in a given text
fn replace_emoji_shortcodes(text: &str) -> String {
    let emoji_map = emojis::emoji_shortcodes();
    let mut replaced_text = String::from(text);

    for (shortcode, emoji) in emoji_map.iter() {
        let regex_str = format!(":{}:", escape(shortcode));
        let regex = Regex::new(&regex_str).unwrap();

        replaced_text = regex.replace_all(&replaced_text, *emoji).to_string();
    }

    replaced_text
}

// fg:100 bg:200 b:0 eff:i
pub fn text(usertext: &str, format: &str) -> String {
    // basic checks
    if format.is_empty() {
        let retstr = replace_emoji_shortcodes(usertext);
        return retstr;
    }

    let escape = "\x1b";
    let end_seq = "\x1b[0m";
    let mut all_effects = String::from("");
    let ansi_codes = colors::ansi_color_codes();

    let parts: Vec<&str> = format.split_whitespace().collect();
    for part in parts {
        let part_all: Vec<&str> = part.split(':').collect(); // ex fg:red
        let command = part_all[0]; // ex fg
        let val = part_all[1]; // ex red

        if command == "fg" {
            if val.starts_with("rgb(") {
                let trimmed_start = val.trim_start_matches("rgb(");
                let trimmed_end = trimmed_start.trim_end_matches(")");
                let rgb: Vec<&str> = trimmed_end.split(',').collect();
                let rgb_formatted = format!("{}[38;2;{};{};{}m", escape, rgb[0], rgb[1], rgb[2]);
                all_effects.push_str(&rgb_formatted);
            } else {
                let fg_code = ansi_codes
                    .get(&val)
                    .expect("Cannot find color name")
                    .to_string();
                let fg_formatted = format!("{}[38;5;{}m", escape, fg_code);
                all_effects.push_str(&fg_formatted);
            }
        } else if command == "bg" {
            if val.starts_with("rgb(") {
                let trimmed_start = val.trim_start_matches("rgb(");
                let trimmed_end = trimmed_start.trim_end_matches(')');
                let rgb: Vec<&str> = trimmed_end.split(',').collect();
                let rgb_formatted = format!("{}[48;2;{};{};{}m", escape, rgb[0], rgb[1], rgb[2]);
                all_effects.push_str(&rgb_formatted);
            } else {
                let bg_code = ansi_codes
                    .get(&val)
                    .expect("Cannot find color name")
                    .to_string();
                let bg_formatted = format!("{}[48;5;{}m", escape, bg_code);
                all_effects.push_str(&bg_formatted);
            }
        } else if command == "eff" {
            let user_effects: Vec<&str> = val.split(',').collect();
            for effect in user_effects {
                match effect {
                    "i" => {
                        let e = format!("{}[3m", escape);
                        all_effects.push_str(&e);
                    }
                    "blink" => {
                        let e = format!("{}[5m", escape);
                        all_effects.push_str(&e);
                    }
                    "u" => {
                        let e = format!("{}[4m", escape);
                        all_effects.push_str(&e);
                    }
                    "b" => {
                        let e = format!("{}[1m", escape);
                        all_effects.push_str(&e);
                    }
                    "s" => {
                        let e = format!("{}[9m", escape);
                        all_effects.push_str(&e);
                    }
                    "d" => {
                        let e = format!("{}[2m", escape);
                        all_effects.push_str(&e);
                    }
                    "r" => {
                        let e = format!("{}[7m", escape);
                        all_effects.push_str(&e);
                    }
                    "hc" => {
                        let e = format!("{}[?25l", escape);
                        all_effects.push_str(&e);
                    }
                    "sc" => {
                        let e = format!("{}[?25h", escape);
                        all_effects.push_str(&e);
                    }
                    &_ => {
                        all_effects.push_str("");
                    }
                }
            }
        }
    }

    let usert = replace_emoji_shortcodes(usertext);
    // TODO: emoji
    let to_ret = format!("{}{}{}", all_effects, usert, end_seq);

    to_ret
}

// === BrogressBar ===
trait Show {
    fn show(&self);
}

trait ShowCompleted {
    fn show_completed(&self);
}

// We at richterm promote the community.
// Pro-gress implies individualistic excellence,
// which denotes a group of elites in their craft and
// average people.
// This is why we at richterm promote bro-gress.
// We operate, despite the  differences in levels,
// as siblings. This increases the rate of achieving the
// targetted objective drastically.
// Note: Summary for the lazy:
//   We simply operate as bros, rather than pros.
#[derive(Debug, Clone)]
pub struct BrogressBar {
    description: String,
    steps: i32,
    steps_done: f32,
    start_time: Instant,
    display: String,
    bar_width: i32,
    time_taken_f: String,
    is_finished: bool,
}

impl BrogressBar {
    pub fn new(steps: i32, description: &str) -> BrogressBar {
        BrogressBar {
            description: description.to_string(),
            steps: steps,
            steps_done: 0.0,
            start_time: Instant::now(),
            display: "━".to_string(),
            bar_width: 40,
            time_taken_f: "".to_string(),
            is_finished: false,
        }
    }

    pub fn update(&mut self, step: f32) {
        let elapsed_time = self.start_time.elapsed();
        let elapsed_time_f = format!("{} ", format_duration(elapsed_time));

        if (self.steps_done + step) >= self.steps as f32 {
            self.steps_done = self.steps as f32;
            self.is_finished = true;
            if self.time_taken_f == "" {
                self.time_taken_f = elapsed_time_f;
            }
        } else {
            self.steps_done = self.steps_done + step;
        }
    }

    pub fn finished(&self) -> bool {
        if (self.steps_done / self.steps as f32) == 1.0 {
            return true;
        }
        return false;
    }

    pub fn get_show_texts(&self) -> Vec<String> {
        let perc = (self.steps_done as f64 / self.steps as f64) * 100.0;
        let done = ((self.steps_done as f64 / self.steps as f64) * self.bar_width as f64) as i32;
        let undone = self.bar_width - done;
        let elapsed_time = self.start_time.elapsed();
        let elapsed_time_f = format!("{} ", format_duration(elapsed_time));

        let mut v = vec![];
        if self.is_finished {
            v.extend(vec![
                text("\r ", ""),
                text(&self.description, ""),
                text(" ", ""),
                text(
                    &self.display.repeat(self.bar_width.try_into().unwrap()),
                    "fg:green",
                ),
                text(" 100% ", "fg:magenta"),
                text(&self.time_taken_f, "fg:orange1"),
            ]);
        } else {
            v.extend(vec![
                text("\r ", "eff:hc"),
                text(&self.description, ""),
                text(" ", ""),
                text(
                    &self.display.repeat(done.try_into().unwrap()),
                    "fg:deep_pink3",
                ),
                text(&self.display.repeat(undone.try_into().unwrap()), "fg:black"),
                text(&format!(" {}% ", perc as i32), "fg:magenta"),
                text(&elapsed_time_f, "fg:steel_blue eff:sc"),
            ]);
        }

        v
    }
}

impl Show for BrogressBar {
    fn show(&self) {
        print([
            text("\r ", ""),
            text(&self.description, ""),
            text(" ", ""),
            text(
                &self.display.repeat(self.bar_width.try_into().unwrap()),
                "fg:green",
            ),
            text(" 100% ", "fg:magenta"),
            text(&self.time_taken_f, "fg:orange1"),
        ]);
    }
}

impl ShowCompleted for BrogressBar {
    fn show_completed(&self) {
        print([
            text("\r ", ""),
            text(&self.description, ""),
            text(" ", ""),
            text(
                &self.display.repeat(self.bar_width.try_into().unwrap()),
                "fg:green",
            ),
            text(" 100% ", "fg:magenta"),
            text(&self.time_taken_f, "fg:orange1"),
            text("\n", ""),
        ]);
    }
}

use std::iter::Iterator;

// Custom Iterator struct
pub struct MyRange {
    start: i32,
    end: i32,
    brogress_bar: BrogressBar,
}

// Implement the Iterator trait for MyRange
impl Iterator for MyRange {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start <= self.end {
            let current = self.start;
            self.start += 1;
            Some(current)
        } else {
            None
        }
    }
}

impl Drop for MyRange {
    fn drop(&mut self) {
        // Code to be executed after the last iteration.
        self.brogress_bar.show_completed();
    }
}

// Idk what i wrote here
fn create_range<'a>(start: i32, end: i32, brogress_bar: BrogressBar) -> MyRange {
    MyRange {
        start,
        end,
        brogress_bar,
    }
}

use std::time::{Duration, Instant};

fn format_duration(duration: Duration) -> String {
    let hours = duration.as_secs() / 3600;
    let minutes = (duration.as_secs() % 3600) / 60;
    let seconds = duration.as_secs() % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

#[derive(Clone)]
pub struct Brogress {
    pub finished: bool,
    tasks: HashMap<String, BrogressBar>,
    inserts_order: Vec<String>,
}

impl Brogress {
    fn new() -> Self {
        Brogress {
            finished: false,
            tasks: HashMap::new(),
            inserts_order: Vec::new(),
        }
    }

    pub fn add_task(&mut self, description: &str, steps: i32) -> String {
        let x = uuid::generate_random_uuid();
        let brogress_bar = BrogressBar::new(steps, description);
        self.tasks.insert(x.clone(), brogress_bar);
        self.inserts_order.push(x.clone());
        x.to_string()
    }

    pub fn update(&mut self, uuid: String, step: f32) {
        if self.finished {
            print([
                text("", "eff:sc"),
                text(&format!("\x1B[{}B", self.tasks.len()), ""),
            ]);
            return;
        }
        if !self.tasks.contains_key(&uuid) {
            println!("Cannot find tasks");
        }

        // let brogress_bar = self.tasks.get_mut(uuid).as_mut();
        let Some(brogress_bar) = self.tasks.get_mut(&uuid) else { todo!() };
        brogress_bar.update(step);
        // brogress_bar.show();
        // let moveup = "\x1B[1F";
        // let movedown = "\x1B[2B";
        // let repeated_chars: String = std::iter::repeat(moveuperase).take(self.tasks.len()).collect();
        let mut sum_finished = 0;
        // let mut index = 0;

        let mut to_print = vec![text("", "eff:hc")];

        for uuid in &self.inserts_order {
            let brogress_bar = self.tasks.get(uuid);
            if brogress_bar.expect("REASON").finished() {
                sum_finished += 1;
            }
            // brogress_bar.expect("REASON").show();
            to_print.extend(brogress_bar.expect("REASON").get_show_texts());
            to_print.push("\n".to_string());
        }

        print(to_print);

        print!("\x1B[{}F", self.tasks.len());

        if sum_finished == self.tasks.len() {
            self.finished = true;
        }
    }
}

// impl Drop for Brogress {
//     fn drop(&mut self) {
//         // This block will be executed when the object goes out of scope
//         println!("Cleaning up resources for data: {}", self.data);
//         // Perform any necessary cleanup or resource release here
//     }
// }


/*
v1 no text colors
    TODO: 
    Add Text struct {
        styles: [];
        
        pub get_raw(){
            [styles, text]
        }
    }
        So that textlength works well

------ title -------- - 1
| this is some text | - 2
| not so long       | 
---------------------

* 1 - 
    leftbarw = width - (title.len + 2) / 2
    leftbar = char.repeat(leftbarw)
    rightbar = char.repeat(leftbarw)
    final = leftbarw + space + title + rightbar

* 2 -
    available_space = width - (len(pipe+space) * 2)
    buffer = []
    loop char over text:
        buffer.add(char)
        if buffer.len >= available space:
            buffer = []
            line = pipe + space + buffer + space + pipe
            print(line)

    if buffer.not_empty:
        line = pipe + space + buffer + space + pipe
        print(line)
    
    print(closingline)

*/
#[derive(Clone)]
pub struct Panel {
    text: Vec<String>,
    title: String,
    style: String
}

impl Panel{
    fn new(text: Vec<String>, title: String, style: &str) -> Self {
        Panel {
            text,
            title, // text()
            style: style.to_string(),
        }
    }
}
impl std::fmt::Display for Panel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let size = terminal_size();
        let Some((Width(w), Height(h))) = size;

        let title_len = self.title.len();
        let bar_width = w - ((title_len + 2) / 2) as i32;
        let bar = "-"::repeat(bar_width);

        print(
            format!("{} {} {}", bar, self.title, bar)
            )
        

    }
}

pub fn track(steps: i32, description: &str) -> MyRange {
    // Downloading ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 100% 0:00:21

    let mut brogress_bar = BrogressBar::new(steps, description);
    for _item in 1..steps {
        brogress_bar.update(1.0);
        brogress_bar.show();
    }
    return create_range(1, steps, brogress_bar);
}

pub fn progress() -> Brogress {
    let p = Brogress::new();

    p
}
