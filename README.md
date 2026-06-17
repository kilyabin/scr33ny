# scr33ny

> [!WARNING]
> This project was primarily built for personal use. Expect rough edges, missing features, and breaking changes without notice. That said, I'm open to changing anything if you want to contribute.

OLED-friendly screensaver for Linux / Wayland with customization support.

```
scr33ny
```

The screen goes black. OLED pixels turn off. Burn-in protection kicks in.

---

## Features

- **Pure black by default** — all OLED pixels off, near-zero power draw
- **Anti burn-in shift** — content shifts by a few pixels every N minutes
- **Screensavers** — blank · GIF animation · static image · text animation
- **Widgets** — digital/analog clock · date · weather
- **Wayland-native** — winit + softbuffer, no Xorg required
- **TOML config** — one readable file, no boilerplate

---

## Install

### Pre-built binary (recommended)

Download the latest binary from [Releases](https://github.com/kilyabin/scr33ny/releases/latest):

```bash
# x86_64 (most desktops/laptops)
curl -Lo scr33ny https://github.com/kilyabin/scr33ny/releases/latest/download/scr33ny-x86_64-linux
chmod +x scr33ny
sudo mv scr33ny /usr/local/bin/

# ARM64 (Raspberry Pi 4+, etc.)
curl -Lo scr33ny https://github.com/kilyabin/scr33ny/releases/latest/download/scr33ny-aarch64-linux
chmod +x scr33ny
sudo mv scr33ny /usr/local/bin/
```

**Runtime requirements:** Wayland compositor, `libwayland-client`, `libxkbcommon`.  
These are present on any Wayland desktop (Sway, Hyprland, GNOME Wayland, KDE Wayland).

### Build from source

Requires a Rust toolchain (`rustup.rs`) and Wayland dev headers:

```bash
# Arch Linux
sudo pacman -S wayland libxkbcommon pkgconf

# Ubuntu / Debian
sudo apt install libwayland-dev libxkbcommon-dev pkg-config
```

```bash
git clone https://github.com/kilyabin/scr33ny
cd scr33ny
cargo install --path .
```

---

## Usage

```bash
scr33ny                            # launch with default config
scr33ny --config ~/my.toml         # custom config path
scr33ny --monitor 1                # display on monitor 1
scr33ny monitors                   # list connected monitors
scr33ny config-path                # print default config location
scr33ny daemon                     # start idle-based auto-launch daemon
```

**Exit:** press `Q` or `Esc`.

---

## Quick config

```bash
mkdir -p ~/.config/scr33ny
cp config.toml.example ~/.config/scr33ny/config.toml
$EDITOR ~/.config/scr33ny/config.toml
```

---

## Screensaver types

### blank *(default)*

Pure black. Best for OLED.

```toml
[screensaver]
type = "blank"
```

### gif

Animated GIF, looped.

```toml
[screensaver]
type  = "gif"
path  = "~/.config/scr33ny/cat.gif"
scale = "fit"                        # fit | fill | original
```

### image

Static image — PNG, JPEG, BMP, WebP.

```toml
[screensaver]
type  = "image"
path  = "~/.config/scr33ny/wallpaper.png"
scale = "fill"
```

### text

Text with animation.

```toml
[screensaver]
type      = "text"
content   = "scr33ny"
font_size = 96.0
color     = "#ffffff"
animation = "float"                  # float | typewriter | fade
```

| Animation | Description |
|-----------|-------------|
| `float` | Lissajous orbit — smooth, non-repeating, maximum OLED protection |
| `typewriter` | Types characters one by one, then restarts |
| `fade` | Fade in → hold → fade out cycle |

---

## Widgets

Each widget is a `[[widgets]]` block in your config.

### Clock

```toml
[[widgets]]
type         = "clock"
style        = "digital"             # digital | analog
font_size    = 72.0
color        = "#ffffff"
show_seconds = true

[widgets.position]
x = "center"                         # left | center | right | <0-100>
y = "center"                         # top  | center | bottom | <0-100>
```

### Date

```toml
[[widgets]]
type      = "date"
font_size = 28.0
color     = "#888888"
format    = "%A, %B %d"              # strftime format

[widgets.position]
x = "center"
y = 62.0                             # 62% from top
```

### Weather

Requires a free [OpenWeatherMap](https://openweathermap.org/api) API key.

```toml
[[widgets]]
type      = "weather"
api_key   = "YOUR_KEY_HERE"
location  = "Moscow"
units     = "metric"                 # metric (°C) | imperial (°F)
font_size = 22.0
color     = "#cccccc"

[widgets.position]
x = "right"
y = "top"
```

Weather refreshes every 30 minutes in the background.

---

## Anti burn-in

Two layers of protection:

1. **Screensaver content moves** — `float` animation follows a Lissajous curve so the same pixel is never lit for long.
2. **Global pixel shift** — the entire frame is offset by a few pixels every N seconds, cycling through 4 corner positions.

```toml
[display]
burn_shift     = 2     # pixels to shift
shift_interval = 120   # seconds between shifts
```

---

## Idle daemon

### With swayidle/hypridle/other idle (recommended)

Example:

```bash
# ~/.config/sway/config
exec swayidle -w \
  timeout 300 'scr33ny' \
  resume   'killall scr33ny'
```

### Built-in daemon

```toml
[daemon]
enabled      = true
idle_timeout = 300
```

```bash
scr33ny daemon
```

Uses `swayidle` if available; falls back to a simple sleep loop otherwise.

---

## Fonts

scr33ny searches common system font paths automatically (DejaVu, Noto, Liberation, Ubuntu, Hack, FreeSans, user fonts in `~/.local/share/fonts/`).

If no font is found:

```bash
# Arch Linux
sudo pacman -S ttf-dejavu

# Ubuntu / Debian
sudo apt install fonts-dejavu
```

---

## Full config reference

See [docs/config.md](docs/config.md).

---

## Project structure

```
src/
├── main.rs              CLI entry point
├── config.rs            TOML config (serde)
├── canvas.rs            Pixel buffer, drawing primitives, font loading
├── app.rs               Event loop, render orchestration
├── screensaver/
│   ├── blank.rs         Pure black
│   ├── gif.rs           GIF animation (frame timing)
│   ├── image_ss.rs      Static image
│   └── text.rs          Text animations
└── widget/
    ├── clock.rs         Digital and analog clock
    ├── date.rs          Date display
    └── weather.rs       OpenWeatherMap widget (async)
```

---

## License

MIT
