use time::Time;

pub fn init_timer(minute: u8, second: u8) -> Result<Time, String> {
    use time::Time;

    match Time::from_hms(0, minute, second) {
        Ok(t) => Ok(t),
        Err(_) => Err(String::from("Could not initialize timer")),
    }

}

pub fn time_to_string(time: &Time) -> String {
    let minute = format!("{}{}", 
        // Add a zero to prevent cases like '5:01'
        if time.minute() < 10 { "0" } else { "" }, time.minute()).to_string();

    let second = format!("{}{}", 
        // Add a zero to prevent cases like '15:0'
        if time.second() < 10 { "0" } else { "" }, time.second()).to_string();

    format!("{}:{}", minute, second).to_string()
}
