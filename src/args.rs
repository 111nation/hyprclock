use std::env;


pub mod command {
    pub const ERR: u8 = 0;
    pub const NONE: u8 = 0;
    pub const CURR_TIME: u8 = 1;
    pub const TIMER: u8 = 1;
}

pub fn get_command() -> u8 {
    use command::*;

    let args: Vec<String> = env::args().collect();
    
    // Ensure a command exists
    let command = match args.get(1) {
        Some(dat) => dat,
        None => {
            println!("No commands was given, aborting");
            return NONE;
        }, 
    };

    if command == "--now" || command == "-n" {
        return CURR_TIME;
    } 

    println!("Invalid command!");
    ERR
}
