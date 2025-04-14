#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::ToSharedString;
slint::include_modules!();
mod clock;


fn main() -> Result<(), slint::PlatformError>{
    let window = MainWindow::new()?;
    let mut timer = clock::Time {
        minute: 1,
        second: 0,
    };
    let duration = timer;
    
    update(&window, &timer, &duration);
    window.on_refresh({
        // Weak handle due to getting reference within itself
        // Prevents circular references
        let win = window.as_weak().unwrap();
    
        move || {
            if !clock::is_midnight(&timer) {
                timer.dec_second();
                update(&win, &timer, &duration);
            } else {
                win.set_clock_active(false);
            }
        }
    });


    window.set_clock_active(true);
    window.run()
}

fn update(window: &MainWindow, curr_time: &clock::Time, duration: &clock::Time) {
    window.set_time(curr_time.to_str().to_shared_string());
    window.set_progress(100f32 - clock::to_progress(&curr_time, &duration));
}
