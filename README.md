# hyprclock
_An up-and-coming clock widget to complement your desktop aesthetic <3_

> [!Note] 
> This is a hyprland first project! It works best using window managers on linux!
> Still cross platform support has been implemented!

## Project News
[NEWS](./NEWS.md)

## Installation
1. Make sure you have the following...
- Rust language
- C++ Build Tools (Windows users)

2. Clone the repository and in the project root execute...
```bash
cargo run
```

## Documentation & Configuration
[DOCS](./DOCS.md)
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
