mod colors;
mod emojis;
use ::emojis::get_by_shortcode;
use colors::find_color;
use regex::{escape, Regex};
use std::{
    io::{self, Write},
    mem::replace,
};

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
    let replaced_text = String::from(escape(text));

    let regex = Regex::new(r"(?m)(?:[^:])(?:[^:])+").unwrap(); // to match word between two ':'
    let mut ids = vec![];
    let mut toreplace = vec![];
    if regex.is_match(&replaced_text) {
        let result = regex.captures_iter(&replaced_text);

        let _ = &result.for_each(|f| {
            f.iter().for_each(|x: Option<regex::Match<'_>>| {
                match get_by_shortcode(x.unwrap().as_str()) {
                    Some(emoji) => {
                        let _ = replace(&mut emoji.as_str(), x.unwrap().as_str());
                        ids.push(emoji.as_str());
                        toreplace.push(x.unwrap().as_str());
                    }
                    None => {}
                }
            })
        });
    }
    let mut i = 0;
    let mut output_text = text.to_string();
    for str in toreplace {
        output_text = output_text.replace(&format!(":{}:", str), ids[i]);
        i += 1;
    }
    //print!("REPLACED TEXT:{}", output_text);
    output_text
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
    let mut all_effects: String = Default::default();

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
                let fg_code = find_color(val).to_string();

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
                let bg_code = find_color(val).to_string();
                let bg_formatted = format!("{}[48;5;{}m", escape, bg_code);
                all_effects.push_str(&bg_formatted);
            }
        } else if command == "eff" {
            let user_effects: Vec<&str> = val.split(',').collect();
            for effect in user_effects {
                match effect {
                    "i" => {
                        let e = format!("{}[3m", escape); // TODO instead of writing format and push_str in all cases create fn with arg
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

trait Show {
    fn show(&self, step_now: i32);
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
#[derive(Debug)]
struct BrogressBar<'a> {
    description: &'a str,
    steps: i32,
    start_time: Instant,
    display: &'a str,
    bar_width: i32,
}

impl BrogressBar<'_> {
    fn new(steps: i32, description: &str) -> BrogressBar {
        BrogressBar {
            description,
            steps,
            start_time: Instant::now(),
            display: "━",
            bar_width: 40,
        }
    }
}

impl Show for BrogressBar<'_> {
    fn show(&self, step_now: i32) {
        let perc = (step_now as f32 / self.steps as f32) * 100.0;
        let done = ((step_now as f64 / self.steps as f64) * self.bar_width as f64) as i32;
        let undone = self.bar_width - done;
        let elapsed_time = self.start_time.elapsed();
        let elapsed_time_f = format!("{} ", format_duration(elapsed_time));

        print([
            text("\r ", "eff:hc"),
            text(self.description, ""),
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
}

impl ShowCompleted for BrogressBar<'_> {
    fn show_completed(&self) {
        let elapsed_time = self.start_time.elapsed();
        let elapsed_time_f = format!("{} ", format_duration(elapsed_time));
        print([
            text("\r ", ""),
            text(self.description, ""),
            text(" ", ""),
            text(
                &self.display.repeat(self.bar_width.try_into().unwrap()),
                "fg:green",
            ),
            text(" 100% ", "fg:magenta"),
            text(&elapsed_time_f, "fg:orange1"),
            text("\n", ""),
        ]);
    }
}

use std::iter::Iterator;

// Custom Iterator struct
pub struct MyRange<'a> {
    start: i32,
    end: i32,
    brogress_bar: BrogressBar<'a>,
}

// Implement the Iterator trait for MyRange
impl Iterator for MyRange<'_> {
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

impl Drop for MyRange<'_> {
    fn drop(&mut self) {
        // Code to be executed after the last iteration.
        self.brogress_bar.show_completed();
    }
}

// Idk what i wrote here
fn create_range<'a>(start: i32, end: i32, brogress_bar: BrogressBar<'a>) -> MyRange<'a> {
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

pub fn track(steps: i32, description: &str) -> MyRange<'_> {
    // Downloading ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 100% 0:00:21

    let mut step_now = 1;
    let brogress_bar = BrogressBar::new(steps, description);
    for _item in 1..steps {
        brogress_bar.show(step_now);

        step_now = step_now + 1;
    }
    return create_range(1, steps, brogress_bar);
}
