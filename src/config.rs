use serde::Deserialize;
use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::io::prelude::*;
use crate::MainWindow; // Use the main window in the main file
use csscolorparser;

#[derive(Deserialize, Debug)]
struct Config {
    window: Window,
    font: Font,
}

#[derive(Deserialize, Debug)]
struct Font {
    active: FontStyle,
    inactive: FontStyle,
}

#[derive(Deserialize, Debug)]
struct FontStyle {
    name: String,
    color: String,
    weight: i32,
}

#[derive(Deserialize, Debug)]
struct Window {
    color: String,
}

pub fn load_config(window: &MainWindow) -> bool {
    // Loads clock and styles configuration
    let config = match get_config_struct() {
        Ok(dat) => dat,
        Err(err) => {
            println!("{}", err);
            return false;
        },
    };

    // Set configurations
    window.set_window_color(get_color(&config.window.color));

    // FONTS
    // Active
    window.set_font_weight_active(config.font.active.weight);
    window.set_font_color_active(get_color(&config.font.active.color));
    window.set_font_active(config.font.active.name.into());
    // Inactive

    window.set_font_weight_inactive(config.font.inactive.weight);
    window.set_font_color_inactive(get_color(&config.font.inactive.color));
    window.set_font_active(config.font.inactive.name.into());

    true
}

fn get_color(color: &String) -> slint::Color {
    let color = match csscolorparser::parse(color) {
        Ok(dat) => dat,
        Err(_) => {
            // Make default color if an error
            csscolorparser::Color {
                r: 255f32,
                g: 255f32,
                b: 255f32,
                a: 255f32,
            }
        },
    };


    let color = color.to_rgba8();

    slint::Color::from_argb_u8(color[3], color[0], color[1], color[2])
}

fn get_config_struct() -> Result<Config, String> {
    // Loads and returns a struct of all configuration of the clock
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
        Err(_) => {
            return Err("Error loading config file contents!".into());
        },
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

    let config_home = config_home + "/hypr";

    // Try load the .toml config first otherwise the .conf file
    // Attempt to force open even though other programs
    match open_read_mode(config_home.clone() + "/hyprclock.toml") {
        Ok(dat) => return Ok(dat),
        _ => (),
    };


    match open_read_mode(config_home.clone() + "/hyprclock.conf") {
        Ok(dat) => return Ok(dat),
        _ => return Err("Could not locate configuration file!".into()),
    };
}

fn open_read_mode(file_name: String) -> Result<File, String> {
    match OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(file_name) {
         Ok(dat) => Ok(dat),
         Err(_) => Err("Failed to open configuration file!".into()),
    }
}
