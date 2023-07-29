mod colors;
pub fn print<T: AsRef<[U]>, U: std::fmt::Debug + std::fmt::Display>(data: T) {
    let slice: &[U] = data.as_ref();
    
    for item in slice {
        // Process the elements here
        print!("{}", item);
    }
}

// fg:100 bg:200 b:0 eff:i
pub fn text(usertext: &str, format: &str) -> String{

    // basic checks
    if  format.is_empty(){
        let retstr = String::from(usertext);
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

    // TODO: emoji
    let to_ret = format!("{}{}{}", all_effects, usertext, end_seq);

    to_ret
}