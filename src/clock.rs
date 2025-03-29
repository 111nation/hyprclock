use ::time::Time;

pub fn init_timer(minute: u8, second: u8) -> Result<time::Time, String> {
    use time::Time;

    match Time::from_hms(0, minute, second) {
        Ok(t) => Ok(t),
        Err(_) => Err(String::from("Could not initialize timer")),
    }

}

pub fn time_to_string(time: &time::Time) -> String {
    let minute = format!("{}{}", 
        // Add a zero to prevent cases like '5:01'
        if time.minute() < 10 { "0" } else { "" }, time.minute()).to_string();

    let second = format!("{}{}", 
        // Add a zero to prevent cases like '15:0'
        if time.second() < 10 { "0" } else { "" }, time.second()).to_string();

    format!("{}:{}", minute, second).to_string()
}

pub async fn start_clock(original_time: &time::Time) -> Result<(), String> {
    /* 
     * Updates clock structure every second until 00:00
     * Returns nothing if no error, returns error message if error
     */
    use tokio::time::{self, Duration};

    let time = original_time.clone();

    // Create an update thread
    tokio::spawn(async move {
        let duration = Duration::from_millis(1000); // 1 second
        let mut interval = time::interval(duration); // Create a 1 second interval
                                                                       
        loop {
            interval.tick().await; // Makes sure to wait for a full 1 second

            if time == Time::MIDNIGHT {
                break;
            }
        }
    });
    
    Ok(())
}
