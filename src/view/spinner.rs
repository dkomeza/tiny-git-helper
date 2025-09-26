use super::*;
use std::sync::{Arc, Mutex};

const SPINNER_FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub struct Spinner<'a> {
    message: &'a str,

    thread_handle: Option<std::thread::JoinHandle<()>>,
    should_stop: Arc<Mutex<bool>>,
}

impl<'a> Spinner<'a> {
    pub fn new(message: &'a str) -> Self {
        let mut spinner = Spinner {
            message,
            thread_handle: None,
            should_stop: Arc::new(Mutex::new(false)),
        };
        spinner.start();
        spinner
    }

    fn start(&mut self) {
        let message = self.message.to_string();
        let should_stop = Arc::clone(&self.should_stop);
        self.thread_handle = Some(std::thread::spawn(move || {
            let mut i = 0;
            loop {
                if *(should_stop.lock().unwrap()) {
                    break;
                }

                let frame = SPINNER_FRAMES[i % SPINNER_FRAMES.len()];
                printer(format!("{} {}", frame, message));
                std::io::Write::flush(&mut stdout()).unwrap();
                print!("\r");
                std::thread::sleep(std::time::Duration::from_millis(75));
                i += 1;
            }
        }));
    }

    pub fn stop(&mut self) {
        if let Some(handle) = &self.thread_handle {
            *(self.should_stop.lock().unwrap()) = true;
        }
    }

    pub fn stop_with_message(&mut self, message: &str) {
        self.stop();
        clear_line();
        printer(message.to_string());
        if (message.ends_with('\n')) == false {
            print!("\n");
        }
    }
}
