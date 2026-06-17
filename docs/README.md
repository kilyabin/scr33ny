# scr33ny

OLED-friendly screensaver for Linux / Wayland with Material 3 Expressive widgets.

## Features

- **Pure black default** — all OLED pixels off, zero burn-in
- **Anti burn-in shift** — content subtly shifts every N minutes
- **Screensavers**: blank, GIF animation, image, text (float / typewriter / fade)
- **Widgets**: digital/analog clock, date, weather (OpenWeatherMap)
- **Wayland-native** via winit + softbuffer (no Xorg dependency)
- **TOML config** — human-readable, well-documented

## Install

```bash
cargo install --path .
```

Or build release binary:

```bash
cargo build --release
# binary at: target/release/scr33ny
```

## Quick start

```bash
# Run with defaults (black screen, no widgets)
scr33ny

# Use a config file
scr33ny --config ~/my-config.toml

# List monitors
scr33ny monitors

# Display on monitor 1
scr33ny --monitor 1

# Show config path
scr33ny config-path
```

## Configuration

Copy the example config:

```bash
mkdir -p ~/.config/scr33ny
cp config.toml.example ~/.config/scr33ny/config.toml
```

See [config.md](config.md) for full reference.

## Keyboard shortcuts

| Key | Action |
|-----|--------|
| `Q` | Exit screensaver |
| `Esc` | Exit screensaver |

## Idle daemon

For automatic activation on idle, use the daemon mode.

### With swayidle (recommended for Sway/wlroots)

```bash
# In your Sway config:
exec swayidle -w \
  timeout 300 'scr33ny' \
  resume   'killall scr33ny'
```

### Built-in daemon

```toml
# In config.toml:
[daemon]
enabled = true
idle_timeout = 300
```

```bash
scr33ny daemon
```

The daemon tries `swayidle` first; falls back to a simple timer if not found.

## Screensaver types

### blank

Pure black. Optimal for OLED — consumes near-zero power.

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
scale = "fit"   # fit | fill | original
```

### image

Static image (PNG, JPEG, BMP, WebP).

```toml
[screensaver]
type  = "image"
path  = "~/.config/scr33ny/bg.png"
scale = "fill"
```

### text

Text with animation.

```toml
[screensaver]
type      = "text"
content   = "hello"
font_size = 96.0
color     = "#ffffff"
animation = "float"   # float | typewriter | fade
```

**Animations:**
- `float` — Lissajous curve orbit, maximum anti-burn-in protection
- `typewriter` — types characters one by one
- `fade` — fade in → hold → fade out cycle

## Widget reference

See [config.md](config.md#widgets) for full widget documentation.

## System fonts

scr33ny searches common paths automatically:

- `/usr/share/fonts/TTF/DejaVuSans.ttf`
- `/usr/share/fonts/noto/NotoSans-Regular.ttf`
- `/usr/share/fonts/liberation/LiberationSans-Regular.ttf`
- `~/.local/share/fonts/` (scanned recursively)

If no font is found:

```bash
# Arch Linux
sudo pacman -S ttf-dejavu

# Ubuntu / Debian
sudo apt install fonts-dejavu
```
