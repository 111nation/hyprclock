#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


slint::include_modules!();


fn main() {
    let window = MainWindow::new().unwrap();

    window.run().unwrap();
}

