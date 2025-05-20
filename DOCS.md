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
military = false     # Use 24-hour time format instead of 12-hour
truncate = false     # Remove leading zeros (e.g., 01:30:00 becomes 1:30:00)

[clock.sound]
tick = "tick.mp3"    # Sound file played on each clock tick
end = "notification.mp3"  # Sound file played when timer completes

[window]
color = "#000000"    # Background color of the window (hex color)
width = 300.0        # Window width in pixels
height = 100.0       # Window height in pixels

[window.border]
color = "#ffffff"    # Border color (hex color)
width = 1.0          # Border width in pixels
radius = 5.0         # Border corner radius in pixels

[font]
color = "#ffffff"    # Text color (hex color)
weight = 400         # Font weight (100-900, 400 is normal)
family = "Arial"     # Font family name
italic = false       # Enable/disable italic style
size = 24.0          # Font size in pixels
spacing = 0.0        # Letter spacing in pixels

[font.stroke]
color = "#000000"    # Text outline color (hex color)
width = 0.0          # Text outline width in pixels
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
