#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod args;

use args::*;
use chrono::{prelude::*, NaiveTime, TimeDelta, DateTime};

slint::include_modules!();


fn main() {
    let window = MainWindow::new().unwrap();
    let mut timer = NaiveTime::default();

    match get_command(&mut timer) {
        Command::CurrTime => current_time(&window),
        Command::Timer => start_timer(&window, &timer),
        _ => return,
    };

    window.set_clock_active(true);
    window.run().unwrap();
}

fn start_timer(window: &MainWindow, duration: &NaiveTime) {
    // Makes the clock application start the timer function
    let win = window.as_weak().unwrap();
    let mut timer = duration.clone();

    window.on_clock_update(move || {
        win.set_time(time_to_string_naive(&timer).into());
        
        // Stop timer when reaching 00:00:00 
        if timer == NaiveTime::default() {
            win.set_clock_active(false);
        }

        // Decrements time by 1 second
        (timer, _) = timer.overflowing_sub_signed(TimeDelta::try_seconds(1).unwrap());
    });
}

fn current_time(window: &MainWindow) {
    // Function used to encapsulate all operations to initialize clock to 
    // display the current time.
    let win = window.as_weak().unwrap();
    let mut time: DateTime<Utc> = Utc::now();

    window.on_clock_update(move || {
        // Display time
        win.set_time(time_to_string(&time).into());
        time = Utc::now();
    });

}

fn time_to_string(time: &DateTime<Utc>) -> String {
    format!("{}", time.format("%H:%M:%S"))
}

fn time_to_string_naive(time: &NaiveTime) -> String {
    format!("{}", time.format("%H:%M:%S"))
}

