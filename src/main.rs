#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::ToSharedString;

mod clock;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError>{
    let window = MainWindow::new()?;
    let mut timer = clock::Time {
        minute: 15,
        second: 0,
    };
    
    timer.dec_second();

    window.set_time(timer.to_str().to_shared_string());

    window.run()
}
