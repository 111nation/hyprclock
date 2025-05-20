use serde::Deserialize;
use std::fs::File;

use std::fs::OpenOptions;

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

use std::io::prelude::*;
use crate::MainWindow; // Use the main window in the main file
use csscolorparser;

#[derive(Deserialize, Debug, Default)]
struct Config {
    #[serde(default)]
    clock: Clock,
    #[serde(default)]
    window: Window,
    #[serde(default)]
    font: Font,
}

#[derive(Deserialize, Debug, Default)]
struct Clock {
    #[serde(default)]
    sound: Sound,
    #[serde(default)]
    military: bool,
    #[serde(default)]
    truncate: bool,
}

#[derive(Deserialize, Debug, Default)]
struct Sound {
    #[serde(default)]
    tick: String,
    #[serde(default)]
    end: String,
}

#[derive(Deserialize, Debug, Default)]
struct Window {
    #[serde(default)]
    color: String,
    #[serde(default)]
    border: Border,
    #[serde(default)]
    width: f32,
    #[serde(default)]
    height: f32,
}

#[derive(Deserialize, Debug, Default)]
struct Border {
    #[serde(default)]
    color: String,
    #[serde(default)]
    width: f32,
    #[serde(default)]
    radius: f32,
}

#[derive(Deserialize, Debug, Default)]
struct Font {
    #[serde(default)]
	color: String,
    #[serde(default)]
	weight: i32,
    #[serde(default)]
	family: String,
    #[serde(default)]
    italic: bool,
    #[serde(default)]
    size: f32,
    #[serde(default)]
    spacing: f32,
    #[serde(default)]
    stroke: Stroke,
}

#[derive(Deserialize, Debug, Default)]
struct Stroke {
    #[serde(default)]
    color: String,
    #[serde(default)]
    width: f32,
}


pub fn load_config(window: &MainWindow) -> bool {
    // Loads clock and styles configuration
    // military indicates if 24hour or 12 hour format must be used for clock
    // truncate indicates if unecissary zeros must be scrapped 00:00:20 -> 20
    let config = match get_config_struct() {
        Ok(dat) => dat,
        Err(err) => {
            println!("{}", err);
            return false;
        },
    };

    // Window config
    window.set_window_color(get_color(&config.window.color));
    window.set_border_color(get_color(&config.window.border.color));
    window.set_border_width(config.window.border.width);
    window.set_border_radius(config.window.border.radius);
    window.set_window_width(config.window.width);
    window.set_window_height(config.window.height);
    
    // Clock config
    window.set_tick_sound(config.clock.sound.tick.into());
    window.set_end_sound(config.clock.sound.end.into());
    window.set_military(config.clock.military); 
    window.set_truncate(config.clock.truncate);

    // Font config
    window.set_font_family(config.font.family.into());
    window.set_font_color(get_color(&config.font.color));
    window.set_font_weight(config.font.weight);
    window.set_font_italic(config.font.italic);
    window.set_font_size(config.font.size);
    window.set_font_spacing(config.font.spacing);
    // Strokes
    window.set_font_stroke_width(config.font.stroke.width);
    window.set_font_stroke_color(get_color(&config.font.stroke.color));     

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
        Err(e) => {
            return Err(format!("{}", e));
        },
    };

    Ok(config)
}

fn get_default_config() -> Result<File, String> {
    // Attempts to load default config file 
    // stored at $XDG_CONFIG_HOME/hyprclock/
    // or fallback to $HOME/.config/hyprclock/
    // on windows try %programdata% or %appdata%
    // Does not create the directories if not found
    //
    // Attemps to load .toml counterpart otherwise .conf used

    let config_home = match get_config_home() {
        Ok(dat) => dat,
        Err(err) => return Err(err),
    };

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

pub fn get_config_home () -> Result<String, String> {
    use std::env;

    #[cfg(unix)]
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

    #[cfg(windows)]
    let config_home = match env::var("programdata") {
        Ok(dat) => dat,
        Err(_) => {
           match env::var("appdata") {
                Ok(dat) => dat + "/.config",
                Err(_) => return Err("Could not get configuration home!".into()),
           }
        },
    };

    let config_home = config_home + "/hypr";

    // Try create directory if config direction doesnt exist
    match std::fs::exists(config_home.clone()) {
        Ok(exists) => {
            if !exists {
               let _ = std::fs::create_dir_all(config_home.clone());
                // Create configuration file
                let _ = File::create(config_home.clone() + "/hyprclock.toml");
            }

        },
        Err(_) => {
            return Err("Attempted to find config directory, but failed!".into()); 
        }
    }

    Ok(config_home)
}

fn open_read_mode(file_name: String) -> Result<File, String> {
    #[cfg(windows)]
    return match OpenOptions::new()
        .read(true)
        .open(file_name) {
         Ok(dat) => Ok(dat),
         Err(_) => Err("Failed to open configuration file!".into()),
    };

    #[cfg(unix)]
    return match OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(file_name) {
         Ok(dat) => Ok(dat),
         Err(_) => Err("Failed to open configuration file!".into()),
    };
}
