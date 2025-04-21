#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clock::set_timer;

slint::include_modules!();
mod clock;
mod args;


fn main() -> Result<(), slint::PlatformError>{
    let mut timer = clock::Time { minute: 0, second: 0 };

    if !args::parse_args(&mut timer) {
        return Ok(());
    }

    let duration = timer;
    let window = MainWindow::new()?;
    
    set_timer(&window, timer, duration);
    clock::update(&window, &timer, &duration);

    println!("{}", timer.minute);

    window.set_clock_active(true);
    window.run()
}

