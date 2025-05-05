#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use chrono::{Duration, Utc, prelude::*};

slint::include_modules!();


fn main() {
    let window = MainWindow::new().unwrap();
    let mut time: DateTime<Local> = Local::now();

    let win = window.as_weak().unwrap();
    window.on_clock_update(move || {
        // Display time
        win.set_time(time_to_string(&time).into());
        // Update time
        if time == DateTime::<Utc>::MIN_UTC { 
            win.set_clock_active(false);
        }

        time = Local::now();
    });

    window.set_clock_active(true);
    window.run().unwrap();
}

fn time_to_string(time: &DateTime<Local>) -> String {
    format!("{}", time.format("%H:%M:%S"))
}

