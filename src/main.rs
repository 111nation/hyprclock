#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod args;
mod config;

use args::*;
use chrono::{ prelude::*, DateTime, NaiveTime, TimeDelta};
use config::load_config;

slint::include_modules!();


fn main() {
    let window = MainWindow::new().unwrap();
    let mut timer = NaiveTime::default();

    load_config(&window);

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

    window.set_time(time_to_string_naive(&timer, win.get_truncate()).into());

    window.on_clock_update(move || {
        // Sounds
        let sound= win.get_end_sound().to_string();
        let tick = win.get_tick_sound().to_string();
        std::thread::spawn(||{ play_sound(tick) });
       
        // Stop timer when reaching 00:00:00 
        if timer == NaiveTime::default() {
            // Play notification sound
            std::thread::spawn(|| {
                if !play_sound(sound) {
                    println!("Failed to play sound!");
                }
            });

            if !notification("hyprclock", "Timer complete!", 5000) {
                println!("Failed to display notification!");
            }

            win.set_clock_active(false);
            return;
        }

        // Decrements time by 1 second
        (timer, _) = timer.overflowing_sub_signed(TimeDelta::try_seconds(1).unwrap());
        win.set_time(time_to_string_naive(&timer, win.get_truncate()).into());
    });
}

fn current_time(window: &MainWindow) {
    // Function used to encapsulate all operations to initialize clock to 
    // display the current time.
    let win = window.as_weak().unwrap();
    let mut time: DateTime<Utc> = Utc::now();

    window.set_time(time_to_string(&time, win.get_military()).into());

    window.on_clock_update(move || {
        let tick = win.get_tick_sound().to_string();
        // Display time
        win.set_time(time_to_string(&time, win.get_military()).into());
        std::thread::spawn(||{ play_sound(tick) });
        time = Utc::now();
    });
}

fn time_to_string(time: &DateTime<Utc>, military: bool) -> String {
    if military {
        return format!("{}", time.format("%H:%M:%S"));
    }

    format!("{}", time.format("%I:%M:%S%P"))
}

fn time_to_string_naive(time: &NaiveTime, truncate: bool) -> String {
    if truncate == true {
        let hour: &str = if time.hour() > 0 { "%H:".into() } else { "".into() };
        let minute: &str = if time.minute() > 0 || !hour.is_empty() { "%M:".into() } else { "".into() };
        let formating = format!("{hour}{minute}%S");
        
        return format!("{}", time.format(formating.as_str()));
    }

    format!("{}", time.format("%H:%M:%S"))
}

fn play_sound(file_location: String) -> bool {
    use std::fs::File;
    use std::io::BufReader;
    use rodio::{Decoder, OutputStream, Sink};

    // Get the output device
    let (_stream, output_dev) = match OutputStream::try_default() {
        Ok(dat) => dat,
        Err(_) => return false,
    };

    // Attatch the sound device to the playback controller
    let sink = match Sink::try_new(&output_dev) {
        Ok(dat) => dat,
        Err(_) => return false,
    };

    // Get the sound file
    let file = match File::open(format!("{}", file_location)) {
        Ok(dat) => dat,
        Err(_) => return false,
    };
    
    let file = BufReader::new(file); // Load chunks into memory

    // Attatch the music file to the playback
    let source = match Decoder::new(file) {
        Ok(dat) => dat,
        Err(_) => return false,
    };

    sink.append(source);
    sink.sleep_until_end();

    true
}

fn notification(title: &str, description: &str, duration: u64) -> bool {
    use notify_rust::Notification;
    use notify_rust::Timeout;
    use std::time::Duration;

    match Notification::new()
        .summary(title)
        .body(description)
        .timeout(Timeout::from(Duration::from_millis(duration)))
        .show() {
    
        Ok(_) => return true,
        Err(_) => return false,

    };
}
