# hyprclock
_An up-and-coming clock widget to complement your desktop aesthetic <3_

> [!Note] 
> This is a hyprland first project! It works best using window managers on linux!
> Still cross platform support has been implemented!

## Project News
19 05 2025:
    <p>
    For windows users we added a neat feature for you to open your configuration file using the -c file. It will open notepad. No more struggling to look for your configuration!
    </P>

16 05 2025:
    <p>
    Hope you having a great day <3, configuration of the clock is finished!!!! This project is near completion
    Now it is time to optimize and update for other operating systems. 
    TODO: Make clock display version number, and test and configure for windows
    </p>

13 05 2025:
    <p>
    We have updated the clock to support new configurations. More configurable options yet to come! Stay tuned because docs on this application are to be released in the for-seeable future! :) <3
    </p>

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

```bash
cargo run
```

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
