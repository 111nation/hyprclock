# hyprclock Documentation <3

## Overview
A minimalist time utility that brings both clock and timer functionality to your desktop! ✨

## Quick Start
```bash
# Show current time
hyprclock --now

# Start a 25-minute timer
hyprclock --timer

# Custom timer (1h 30m 45s)
hyprclock --timer 1h 30m 45s
```

## Features
- 🕒 Real-time clock display (12/24h format)
- ⏰ Countdown timer with notifications
- 🎨 Fully customizable appearance
- 🔊 Sound effects & system notifications
- 💫 Cross-platform (Linux, Windows, macOS)

## Configuration
Config file location:
- Linux: `$XDG_CONFIG_HOME/hyprclock/` or `$HOME/.config/hyprclock/`
- Windows: `%programdata%` or `%appdata%`
- macOS: `$HOME/.config/hyprclock/`

### Basic Config
```toml
[clock]
military = false  # 24h format
truncate = false  # Remove leading zeros

[clock.sound]
tick = "tick.mp3"
end = "notification.mp3"

[window]
color = "#000000"
width = 300.0
height = 100.0

[font]
color = "#ffffff"
size = 24.0
family = "Arial"
```

## Commands
- `-n, --now`: Show current time
- `-t, --timer`: Start timer
- `-h, --help`: Show help
- `-v, --version`: Show version
- `-c, --config`: Edit config

## Timer Syntax
```
hyprclock --timer [time][unit] [time][unit] ...
```
Units: `h`/`hour`, `m`/`min`, `s`/`sec`

## Troubleshooting
1. No sound? Check:
   - Sound files exist
   - Audio system works
   - File permissions

2. Config issues? Verify:
   - Config file exists
   - Valid TOML syntax
   - File permissions

3. Timer problems? Check:
   - Time format
   - Valid ranges
   - Command syntax
