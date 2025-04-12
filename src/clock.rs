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


