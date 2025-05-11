# Hyprclock
_An up-and-coming clock widget to complement your desktop aesthetic_

> [!Note] 
> This is a hyprland first project! It works best using window managers on linux!
> Still cross platform support has been implemented!

## Project News
11 05 2025:
    <p>
    Happy Mother's day!
    The clock has now been updated to be configurable, this feature is still in alpha and a lot of configurations are due to be released in the coming days to weeks!
    </p>

07 05 2025:
    <p>
    Timer Implementation has been shipped and now you are able to use the application as a basic study timer!
    I am yet to add the ability of the clock to ping, send a notification or ring when the timer has ended!
    A new help page has been created with the help with AI, allowing you to fully understand the utility. 
    I am yet to test the clock more on Windows and Linux.
    </p>

05 05 2025:
    <p>
    Implemented 24 hour current time clock feature
    </p>

04 05 2025:
    <p>
    The project has undergone rewriting because the previous code was unsuitable and hard to debug
    </p>

## Installation
_Make sure you have the Rust language properly installed_

On any operating system, enter the project root and execute

## Configuration
All configurations are either stored in `$XDG_CONFIG_HOME/hypr/hyprclock.conf` or `~/.config/hypr/hyprclock.conf`, the clock app also accepts the `.toml` counterpart!

Configuration takes place in .toml form

```toml
[window]
color = "rgba(0, 0, 0, 0.8)"

[font.active]
weight = 700
name = "Montserrat"
color = "white"

[font.inactive]
color = "red"
```


```bash
cargo run
```
