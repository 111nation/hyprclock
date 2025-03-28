#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clock;
use clock::init_timer;
use clock::time_to_string;

use time::Time;
slint::include_modules!();
use slint::SharedString;

fn main() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;
    let timer: Time = init_timer(15, 0)?;
    
    let display = SharedString::from(time_to_string(&timer));
    window.set_time(display);

    window.run()
}
