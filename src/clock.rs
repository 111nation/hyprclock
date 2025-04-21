slint::include_modules!();
use crate::slint_generatedMainWindow::MainWindow as Window;
use slint::ToSharedString;

#[derive(Copy, Clone)] // Allow copying a time struct
pub struct Time {
    pub minute: u8,
    pub second: u8
}

impl Time {
    pub fn dec_second(&mut self) -> bool {
        // true - successful decrementation
        // false - second is already zero
        
        if self.second > 0 {
            self.second -= 1;
            return true;

        } else if self.minute > 0 && self.second == 0 {
            // Decrement to the next minute
            self.minute -= 1;
            self.second = 59;
            return true;
        }

        return false;
    }

    pub fn to_str(&self) -> String {
        let minute = format!("{}{}",
            if self.minute < 10 { "0" } else { "" }, self.minute);

        let second= format!("{}{}",
            if self.second < 10 { "0" } else { "" }, self.second);

        format!("{}:{}", minute, second)
    }

}

pub fn is_midnight(time: &Time) -> bool {
    time.minute == 0 && time.second == 0
}

pub fn to_progress(curr_time: &Time, end_time: &Time) -> f32 {
    let curr_minute: f32 = (curr_time.minute as f32 + (curr_time.second as f32/ 60f32)).into();
    let end_minute: f32 = (end_time.minute as f32 + (end_time.second as f32 / 60f32)).into();
    
    (curr_minute / end_minute) * 100f32
}

pub fn update(window: &Window, curr_time: &Time, duration: &Time) {
    window.set_time(curr_time.to_str().to_shared_string());
    window.set_progress(100f32 - to_progress(&curr_time, &duration));
}

pub fn set_timer(window: &Window, mut timer: Time, duration: Time) {
        // Weak handle due to getting reference within itself
        // Prevents circular references
        let win = window.as_weak().unwrap();
    
        window.on_refresh({
            move || {
                if !is_midnight(&timer) {
                    timer.dec_second();
                    update(&win, &timer, &duration);
                } else {
                    win.set_clock_active(false);
                }
            }
        });
 }
