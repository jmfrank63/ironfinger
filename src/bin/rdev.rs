extern crate rdev;
#[macro_use]
extern crate lazy_static;

use crossterm::terminal;
use rdev::{listen, Event, EventType, Key};
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::SystemTime;

static TERMINAL_WIDTH: AtomicUsize = AtomicUsize::new(0);

lazy_static! {
    static ref KEY_PRESS_TIMES: Mutex<HashMap<Key, SystemTime>> = Mutex::new(HashMap::new());
}

fn main() {
    let (width, _) = terminal::size().unwrap_or((80, 24)); // Default to 80x24 if unable to get terminal size
    TERMINAL_WIDTH.store(width as usize, Ordering::SeqCst);
    match listen(callback) {
        Ok(_) => println!("Listening for key events..."),
        Err(error) => println!("Error: {:?}", error),
    }
}

fn callback(event: Event) {
    let width = TERMINAL_WIDTH.load(Ordering::SeqCst);
    let spaces = " ".repeat(width);
    print!("\r{}\r", spaces);
    match event.event_type {
        EventType::KeyPress(key) => {
            let mut key_press_times = KEY_PRESS_TIMES.lock().unwrap();
            key_press_times.insert(key, event.time);
        }
        EventType::KeyRelease(key) => {
            let mut key_press_times = KEY_PRESS_TIMES.lock().unwrap();
            if let Some(press_time) = key_press_times.remove(&key) {
                let duration = event.time.duration_since(press_time).unwrap_or_default();
                let duration_ms = duration.as_millis();
                println!("Key {:?} was pressed for {} ms", key, duration_ms);
            }
        }
        _ => (),
    }
}
