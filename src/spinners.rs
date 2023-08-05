
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

struct Spinner {
    frames: Vec<char>,
    interval: Duration,
}

impl Spinner {
    fn new() -> Self {
        Spinner {
            frames: vec!["🌕", "🌖", "🌘", "🌒", "🌓", "🌔"],
            interval: Duration::from_millis(100),
        }
    }

    fn start(&self, status: Arc<Mutex<Status>>, stop_rx: mpsc::Receiver<()>) {
        let num_frames = self.frames.len();
        let mut current_frame = 0;

        loop {
            let status = status.lock().unwrap();
            print!("\r{} {}", self.frames[current_frame], status.message);
            io::stdout().flush().unwrap();

            current_frame = (current_frame + 1) % num_frames;
            thread::sleep(self.interval);

            if stop_rx.try_recv().is_ok() {
                break;
            }
        }
    }
}

struct Status {
    message: String,
}

impl Status {
    fn new() -> Self {
        Status {
            message: String::new(),
        }
    }

    fn log(&mut self, message: &str) {
        self.message = message.to_string();
    }
}

struct Live {
    spinner: Arc<Mutex<Spinner>>,
    status: Arc<Mutex<Status>>,
}

impl Live {
    fn new() -> Self {
        let spinner = Arc::new(Mutex::new(Spinner::new()));
        let status = Arc::new(Mutex::new(Status::new()));

        Live {
            spinner,
            status,
        }
    }

    fn start(&self) {
        let spinner = Arc::clone(&self.spinner);
        let status = Arc::clone(&self.status);

        let (tx, rx) = mpsc::channel();


        let spinner_thread = thread::spawn(move || {
            let spinner = spinner.lock().unwrap();
            spinner.start(status.clone(), rx);
        });

        let status_thread = thread::spawn(move || {
            for i in 1..=5 {
                thread::sleep(Duration::from_secs(1));
                let message = format!("Processing... Step {}", i);
                let mut status = status.lock().unwrap();
                status.log(&message);
            }
            tx.send(()).unwrap();
        });

        // Wait for the main task thread to finish
        status_thread.join().unwrap();

        // Wait for the spinner thread to finish
        spinner_thread.join().unwrap();
    }
}

