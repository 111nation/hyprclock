use std::env;
use std::option::Option;
use crate::clock;
use clock::Time;

pub fn parse_args (timer: &mut Time) -> bool {
    // true - successful
    // false - unsuccesful

    let args: Vec<String> = env::args().collect();
    
    // If valid command continue
    let command = match args.get(1) {
        Some(arg) => arg,
        None => return false,
    };

    if command == "--timer" || command == "-t" {
        return parse_timer_args(&args, timer);
    }

    return true;
}

fn parse_timer_args(args: &Vec<String>, timer: &mut Time) -> bool {
    timer.minute = 0;
    timer.second = 0;
    // No specified timer length == Default to 5min timer
    if args.len() - 2 <= 0 {
        timer.minute =  5;
        timer.second = 0;

        return true;
    }


    for arg in args.iter().skip(2) {
        let (unit, pos) = match get_unit(arg) {
            Some(dat) => dat, 
            None => return false, 
        };

        // Delete everything from the unit to the end
        // And attempt to convert to time
        let time = match arg.get(..pos) {
            Some(dat) => dat,
            None => return false,
        };

        // Attempt to parse the time passed in
        let time: f32 = match time.parse() {
            Ok(num) => num,
            Err(_) => return false,
        };

        if !is_valid_time(time) {
            return false;
        }

        let time: u8 = time as u8;

        // Update timer
        if unit == "min" || unit == "m" {
            timer.minute +=  if time > 60 { 60 } else { time };
        } else if unit == "sec" || unit == "s" {
            // Prevent overflowing of seconds
            timer.minute += time / 60;
            timer.second += time % 60;
        }
    }

    return true;
}

fn get_unit (arg: &String) -> Option<(&str, usize)>  {
    // Check if valid time measurement 
    // Minute - min, m
    // Seconds - sec, s

    let valid_units = vec!["min", "m", "sec", "s"];
    
    // Search if a valid unit exists within that argument
    // Against all valid units
    for unit in valid_units.iter() {
        match arg.find(unit) {
            Some(pos) => return Some((unit, pos)),
            None => continue,
        }
    }

    return None;
}

fn is_valid_time(time: f32) -> bool {
    if time <= 0.0 { return false; }
    // Checks for invalid decimals
    if time % 1.0 > 0.0 { return false; } 
    
    return true;
}
