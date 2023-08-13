
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;


use std::collections::HashMap;

pub fn frame_data() -> HashMap<String, Vec<&'static str>> {
    let frames = HashMap::from([
        ("simple".to_string(), vec!["-", "/", "|", "\\"]),
        ("moon".to_string(), vec!["🌕", "🌖", "🌘", "🌒", "🌓", "🌔"]),
        ]);

    frames
}

pub struct Spinner {
    frames: Vec<String>,
    interval: Duration,
    type_: String,
    message: String,
    finished: bool,
    thread_handle: Option<thread::JoinHandle<()>>
}

impl Spinner {
    pub fn new(type_: &str, message: &str) -> Self {
        Spinner {
            frames: vec!["🌕".to_string(), "🌖".to_string(), "🌘".to_string(), "🌒".to_string(), "🌓".to_string(), "🌔".to_string()],
            interval: Duration::from_millis(100),
            type_: type_.to_string(),
            message: message.to_string(),
            finished: false,
            thread_handle: None,
        }
    }

    pub fn start(&mut self) {
        let num_frames = self.frames.len();
        let mut current_frame = 0;


        if !self.finished {
            self.finished = false;
            let handle = thread::spawn(move || {
                while !self.finished {
                    print!("\r{} {}", self.frames[current_frame], self.message);
                    io::stdout().flush().unwrap();

                    current_frame = (current_frame + 1) % num_frames;
                    thread::sleep(self.interval);
                }
            });
            self.thread_handle = Some(handle);
        }
        
        
    }


    pub fn log(&self, message: &str) {
        println!("{}", message); // TODO: change to printer protocol
    }

    fn stop(&mut self) {
        self.finished = true;
        if let Some(handle) = self.thread_handle.take() {
            handle.join().expect("Failed to join the thread.");
        }
    }
}
