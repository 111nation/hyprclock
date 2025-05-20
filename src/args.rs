use std::env;

use chrono::NaiveTime;

pub enum Command {
    Error,
    Nothing,
    CurrTime,
    Timer,
}

const VERSION: &str = "0.1.0";

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
            return CurrTime;
        }, 
    };

    if command == "--now" || command == "-n" {
        return CurrTime;
    } else if command == "--timer" || command == "-t" {
        if !init_timer(&args, timer) {
            return Error;
        }
            
        return Timer;

    } else if command == "--help" || command == "-h" {
        print_help();
        return Nothing;

    } else if command == "--config" || command == "-c" {
        // Grab editor as a second command or "" for no editor
        let editor: &str = match args.get(2) {
            Some(dat) => dat,
            None => "",
        };

        let _ = crate::config::get_config_home();

        if !open_config(editor) {
            println!("Failed to open configuration with editor '{}'!", editor);
            return Error;
        }
        
        return Nothing;

    } else if command == "--version" || command == "-v" {
        println!("hyprclock version: {VERSION}");

        return Nothing;
    }

    println!("Invalid command!");
    Error
}

fn open_config(editor: &str) -> bool {
    // Allows user to easily open configuration file using 
    // operating systems default editor
    let config_home = match crate::config::get_config_home() {
        Ok(dat) => dat,
        Err(_) => return false,
    };
    
    // Test if hyprclock.conf exists
    // Otherwise open hyprclock.toml (default)

    let file = match std::fs::exists(config_home.clone() + "/hyprclock.conf") {
        Ok(exist) => {
            if exist {
                config_home + "/hyprclock.conf"
            } else {
                config_home + "/hyprclock.toml"
            }
        }

        Err(_) => return false,
    };

    open_editor(&file, &editor)
}

fn open_editor(file: &String, editor: &str) -> bool {
    use std::process::Command;

    // Stores the editor which we will evoke a process from
    let editor= get_editor(editor);

    match Command::new(editor).args([file])
        .status() {
        Ok(_) => return true,
        Err(_) => return false,
    };

}
fn get_editor(editor: &str) -> String {
    // If user already selected an editor, just use that
    if !editor.is_empty() { 
        return editor.into();
    }

    // Open OS default editors 

    #[cfg(windows)]
    return "notepad".into();

    #[cfg(target_os = "macos")]
    return "TextEdit".into();

    #[cfg(target_os = "linux")]
    match env::var("EDITOR") {
        Ok(dat) => return dat,
        Err(_) => return "xdg-open".into(),
    }
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

    // Ensure time is not more than 23:59:59
    let total_time = *hour as f32 + (*min as f32/60f32) + (*sec as f32/360f32); 
    if total_time >= 24f32 {
        *hour = 23;
        *min = 59;
        *sec = 59;
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

        // Ensures to throw error when for example 'min5445ewfw' is entered
        // AKA: unit entered but unexpected trailing characters still present
        if arg.len()-(pos+unit.len()) > 0 {
            return Err("Invalid format!");
        }

        let number = match arg.get(..pos) {
            Some(dat) => dat,
            None => return Err("Failed to parse argument!"),
        };

        let number = match number.parse::<u8>() {
            Ok(dat) => dat,
            Err(_) =>  return Err("Invalid format or time range too big!"),
        };

        return Ok((number, unit));

    }

    return Err("Invalid format!");

}

fn print_help() {
    println!("{}", r#"
        hyprclock - A Minimalist Time Utility
        =====================================

        Usage:
          hyprclock [OPTIONS]

        Options:
          -n, --now              Show the current time
          -t, --timer [ARGS]     Start a countdown timer
                                 Format: <value><unit> [<value><unit>] ...
                                 Units: h/hour, m/min, s/sec
                                 Example: hyprclock --timer 1h 30min 10s

          -c, --config [EDITOR]  Open configuration file in the specified editor.
                                 If no editor is given, a default will be used:
                                   Linux → $EDITOR or xdg-open
                                   Windows → notepad
                                   macOS → TextEdit
                                 Example: hyprclock --config nano

          -v, --version          Show version information
          -h, --help             Show this help message

        Customization:
        - Configuration file location depends on your system:
            • Linux/macOS → $XDG_CONFIG_HOME/hypr/hyprclock.toml  
                           (fallback: $HOME/.config/hypr/hyprclock.toml)
            • Windows     → %ProgramData%\hypr\hyprclock.toml

        - You can also use a `.conf` file instead: `hyprclock.conf` in the same folder.

        Configurable Features:
        - Refer to the README.md at https://github.com/111nation/hyprclock

        Notes:
        - If no command is given, hyprclock defaults to showing the current time.
        - Timer defaults to 25 minutes if no arguments are passed after --timer.
        - Maximum supported time is 23h 59m 59s.

        Examples:
          hyprclock --now
          hyprclock -t 10min
          hyprclock --timer 1h 5m 20s
          hyprclock --config micro
    "#);
}

