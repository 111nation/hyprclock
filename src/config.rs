use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use crate::MainWindow; // Use the main window in the main file

#[derive(Deserialize)]
struct Config {
    font: String,
    color: String,
}

pub fn load_config(window: &MainWindow) -> bool {
    let config = match get_config_struct() {
        Ok(dat) => dat,
        Err(err) => {
            println!("{}", err);
            return false;
        },
    };

    true
}

fn get_config_struct() -> Result<Config, String> {
    //Return true if successfully loaded configuration
    let mut config_file = match get_default_config() {
        Ok(dat) => dat,
        Err(err) => {
            return Err(err);
        }
    };

    let mut buffer = String::new();
    match config_file.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(_) => return Err("Failed to read from config file!".into()),
    };

    let config: Config = match toml::from_str(&buffer) {
        Ok(dat) => dat,
        Err(_) => return Err("Error loading confg file contents!".into()),
    };

    Ok(config)
}

fn get_default_config() -> Result<File, String> {
    // Attempts to load default config file 
    // stored at $XDG_CONFIG_HOME/hyprclock/
    // or fallback to $HOME/.config/hyprclock/
    // Does not create the directories if not found
    //
    // Attemps to load .toml counterpart otherwise .conf used
    use std::env;

    // Try load $XDG_CONFIG_HOME otherwise try load $HOME/.config
    // TODO: Fix this!!!!
    let config_home = match env::var("XDG_CONFIG_HOME") {
        Ok(dat) => dat,
        Err(_) => {
           match env::var("HOME") {
                Ok(dat) => dat + "/.config",
                Err(_) => return Err("Could not get configuration home!".into()),
           }
        },
    };

    // Try load the .toml config first otherwise the .conf file
    match File::open(config_home.clone() + "/hyprclock.toml") {
        Ok(dat) => return Ok(dat),
        _ => (),
    };

    match File::open(config_home.clone() + "/hyprclock.conf") {
        Ok(dat) => return Ok(dat),
        _ => return Err("Could not locate configuration file!".into()),
    };


}

