use std::collections::HashMap;

pub fn frame_data() -> HashMap<&'static str, Vec<&'static str>> {
    let frames = HashMap::from([
    	("simple", vec!["-", "/", "|", "\\"]),
    	("moon", vec!["🌕", "🌖", "🌘", "🌒", "🌓", "🌔"]),
    	]);

    frames
}


struct Spinner {
    frames: Vec<&'static str>,
    current_frame: usize,
}

impl Spinner {
    fn new() -> Self {
        Self {
            frames: vec!["🌕", "🌖", "🌘", "🌒", "🌓", "🌔"],
            current_frame: 0,
        }
    }

    fn spin(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.frames.len();
    }

    fn update(&mut self) {
        print!("\r{}", self.frames[self.current_frame]);
        io::stdout().flush().unwrap();
    }

    fn start(&mut self) {
        loop {
            self.update();
            self.spin();
            thread::sleep(Duration::from_millis(100));
        }
    }

    fn stop(&self) {
        println!();
    }
}
