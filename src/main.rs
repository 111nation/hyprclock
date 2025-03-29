#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clock;

use time::{Duration, Time};

slint::include_modules!();
use slint::SharedString;

use clock::*;

fn main() -> Result<(), slint::PlatformError> {
    use std::thread;

    let window = MainWindow::new()?;
    let mut timer: Time = init_timer(15, 0)?;
    
    thread::spawn(move || {
        while timer != Time::MIDNIGHT {
            timer -= Duration::SECOND;
            thread::sleep(std::time::Duration::from_millis(1000));
        }
    });

    display(&timer, &window);
    window.run()
}

fn display(timer: &Time, window: &MainWindow) {
    let display = SharedString::from(time_to_string(&timer));
    window.set_time(display);
}

// std::time::Duration
/*
 * Represent span of time ranging from seconds to nanoseconds
 * Used for timing, delays and measuring elapsed time
 */ 
