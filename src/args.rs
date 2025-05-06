use std::env;

use chrono::NaiveTime;

pub enum Command {
    Error,
    Nothing,
    CurrTime,
    Timer,
}

pub fn get_command(timer: &mut NaiveTime) -> Command {
    // Returns state code if successful clock initialization
    // Parameter returns the time data needed to start the clock
    // If error or clock feature started, timer is not populated
    use Command::*;

    let args: Vec<String> = env::args().collect();
    
    // Ensure a command exists
    let command = match args.get(1) {
        Some(dat) => dat,
        None => {
            println!("No commands was given, aborting");
            return Nothing;
        }, 
    };

    if command == "--now" || command == "-n" {
        return CurrTime;
    } else if command == "--timer" || command == "-t" {
        if !init_timer(&args, timer) {
            return Error;
        }
            
        return Timer;
    }

    println!("Invalid command!");
    Error
}

fn init_timer(args: &Vec<String>, timer: &mut NaiveTime) -> bool {
    // true -  successful timer initialization
    // false - unsuccessful timer initialization
    let (hour, min, sec) = match get_timer_args(&args) {
        Ok(dat) => dat,
        Err(err) => {
            println!("{}", err);
            return false;
        },
    };

    *timer = match NaiveTime::from_hms_opt(hour as u32, min as u32, sec as u32) {
        Some(dat) => dat,
        None => {
            println!("Failed to initialize timer!");
            return false;
        }
    };

    return true;
}

fn get_timer_args(args: &Vec<String>) -> Result<(u8, u8, u8), &str> {
    // Expects time and unit seperated with spaces for example
    // 6h 4s 5min to form a 6 hour, 5 minute and 4 second timer

    // Default to 25 minute timer if no arguments given
    if args.len() <= 2 {
        return Ok((0, 25, 0));
    }

    // Exctracts time and unit of each argument
    let mut hour: u8 = 0;
    let mut min: u8 = 0;
    let mut sec: u8 = 0;

    for arg in args.iter().skip(2) {
        let (number, unit) = match extract_magnitude_and_unit(arg) {
            Ok(dat) => dat,
            Err(err) => return Err(err),
        };

        if unit == "hour" || unit == "h" {
            hour += number;
        } else if unit == "min" || unit == "m" {
            min += number;
        } else if unit == "sec" || unit == "s" {
            sec += number;
        }
    }

    format_time(&mut hour, &mut min, &mut sec);

    Ok((hour, min, sec))
}

fn format_time(hour: &mut u8, min: &mut u8, sec: &mut u8) {
    // Ensures that there is no hour, minute, sec overflow
    if *sec > 59 {
        *min += *sec / 60u8;
        *sec = *sec % 60u8;
    }

    if *min > 59 {
        *hour += *min / 60u8;
        *min = *min % 60u8;
    }

    if *hour > 24 {
        *hour = 24;
    }
}

fn extract_magnitude_and_unit(arg: &String) -> Result<(u8, &str), &str> {
    // Gets magnitude and unit of a argument
    // For example: 6min, 6 as a magnitude and min as a unit
    let units: Vec<&str> = vec!["h", "hour", "min", "m", "sec", "s"];     

    for unit in units.iter() {
        // Attempts to get position of valid unit
        let pos = match arg.find(unit) {
            Some(dat) => dat,
            None => continue,
        };

        let number = match arg.get(..pos) {
            Some(dat) => dat,
            None => return Err("Failed to parse argument!"),
        };

        let number = match number.parse::<u8>() {
            Ok(dat) => dat,
            Err(_) =>  return Err("Invalid number and unit entered!"),
        };

        return Ok((number, unit));

    }

    return Ok((0, ""));

}
