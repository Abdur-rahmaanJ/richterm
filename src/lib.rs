mod colors;
mod emojis;
use regex::Regex;
use std::fmt::Debug;
use regex::escape;
use std::io::{self, Write};


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
pub fn text(usertext: &str, format: &str) -> String{

    // basic checks
    if  format.is_empty(){
        let retstr = replace_emoji_shortcodes(usertext);
        return retstr;
    }

    
    let escape = "\x1b";
    let end_seq = "\x1b[0m";
    let mut all_effects = String::from("");
    let ansi_codes = colors::ansi_color_codes();
    

    let parts: Vec<&str> = format.split_whitespace().collect();
    for part in parts{
        let part_all: Vec<&str> = part.split(':').collect(); // ex fg:red
        let command = part_all[0]; // ex fg
        let val = part_all[1]; // ex red

        if command == "fg"{ 
            if val.starts_with("rgb("){
                let trimmed_start = val.trim_start_matches("rgb(");
                let trimmed_end = trimmed_start.trim_end_matches(")");
                let rgb: Vec<&str> = trimmed_end.split(',').collect();
                let rgb_formatted = format!("{}[38;2;{};{};{}m", escape, rgb[0], rgb[1], rgb[2]);
                all_effects.push_str(&rgb_formatted);
            } else {
                let fg_code = ansi_codes.get(&val).expect("Cannot find color name").to_string();
                let fg_formatted = format!("{}[38;5;{}m", escape, fg_code);
                all_effects.push_str(&fg_formatted);
            }
        } else if command == "bg"{
            if val.starts_with("rgb("){
                let trimmed_start = val.trim_start_matches("rgb(");
                let trimmed_end = trimmed_start.trim_end_matches(')');
                let rgb: Vec<&str> = trimmed_end.split(',').collect();
                let rgb_formatted = format!("{}[48;2;{};{};{}m", escape, rgb[0], rgb[1], rgb[2]);
                all_effects.push_str(&rgb_formatted);
            } else {
                let bg_code = ansi_codes.get(&val).expect("Cannot find color name").to_string();
                let bg_formatted = format!("{}[48;5;{}m", escape, bg_code);
                all_effects.push_str(&bg_formatted);
            }
        } else if command == "eff"{
            let user_effects: Vec<&str> = val.split(',').collect();
            for effect in user_effects{
                match effect {
                    "i" => {
                        let e = format!("{}[3m", escape);
                        all_effects.push_str(&e);
                    },
                    "blink" => {
                        let e = format!("{}[5m", escape);
                        all_effects.push_str(&e);
                    },
                    "u" => {
                        let e = format!("{}[4m", escape);
                        all_effects.push_str(&e);
                    },
                    "b" => {
                        let e = format!("{}[1m", escape);
                        all_effects.push_str(&e);
                    },
                    "s" => {
                        let e = format!("{}[9m", escape);
                        all_effects.push_str(&e);
                    },
                    "d" => {
                        let e = format!("{}[2m", escape);
                        all_effects.push_str(&e);
                    },
                    "r" => {
                        let e = format!("{}[7m", escape);
                        all_effects.push_str(&e);
                    },
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

use std::ops::Range;
use std::iter::Iterator;


// Custom Iterator struct
pub struct MyRange<'a>{
    start: i32,
    end: i32,
    description: &'a str,
    bar_width: i32,
    display: &'a str
}

// Implement the Iterator trait for MyRange
impl Iterator for MyRange<'_>{
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

impl Drop for MyRange<'_>{
    fn drop(&mut self) {
        // Code to be executed after the last iteration.
        print([
            text("\r ", ""),
            text(self.description, ""),
            text(" ",""),
            text(&self.display.repeat(self.bar_width.try_into().unwrap()), "fg:green"),
            text(" %100 --:--:--", ""),
            text("\n",""),
            ]);
    }
}

// Idk what i wrote here
fn create_range<'a>(start: i32, end: i32, description: &'a str, bar_width: i32, display: &'a str) -> MyRange<'a> {
    MyRange { start, end, description, bar_width, display }
}

use std::time::{Duration, Instant};

fn format_duration(duration: Duration) -> String {
    let hours = duration.as_secs() / 3600;
    let minutes = (duration.as_secs() % 3600) / 60;
    let seconds = duration.as_secs() % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn track(step: i32, description: &str) -> MyRange<'_>{
    // Downloading ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 100% 0:00:00 0:00:21
    let len_itr = step;//range.start_bound() - range.end_bound();
    let bar_width = 40;
    let display = "━";
    let incr = bar_width/len_itr as i32;
    let mut step = 0;
    let start_time = Instant::now();
    for item in 1..len_itr{
        let fraction = (step/bar_width) * bar_width;
        let intfrac = fraction as i32;
        let perc = (step as f32 /bar_width as f32) * 100.0;
        let elapsed_time = start_time.elapsed();
        let elapsed_time_f = format!("{}", format_duration(elapsed_time));
        print([
            text("\r ", ""),
            text(description, ""),
            text(" ",""),
            text(&display.repeat(step.try_into().unwrap()), "fg:deep_pink3"),
            text(&display.repeat((bar_width-step).try_into().unwrap()), "fg:black"),
            text(&format!(" {}% ", perc as i32), "fg:magenta"),
            text(&elapsed_time_f, "fg:orange1")
            ]);


        step = step + incr;
    }

    

    return create_range(1, len_itr, description, bar_width, display);


}