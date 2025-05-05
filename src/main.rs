#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod args;

use args::*;
use chrono::{Utc, prelude::*};

slint::include_modules!();


fn main() {
    let window = MainWindow::new().unwrap();

    match get_command() {
       command::CURR_TIME => current_time(&window),
        command::TIMER => start_timer(&window, 
        _ => return,
    };

    window.set_clock_active(true);
    window.run().unwrap();
}

fn current_time(window: &MainWindow) {
    // Function used to encapsulate all operations to initialize clock to 
    // display the current time.
    let win = window.as_weak().unwrap();
    let mut time: DateTime<Local> = Local::now();

    window.on_clock_update(move || {
        // Display time
        win.set_time(time_to_string(&time).into());
        // Update time
        if time == DateTime::<Utc>::MIN_UTC { 
            win.set_clock_active(false);
        }

        time = Local::now();
    });

}

fn time_to_string(time: &DateTime<Local>) -> String {
    format!("{}", time.format("%H:%M:%S"))
}

