# Config reference

Default location: `~/.config/scr33ny/config.toml`

Override: `scr33ny --config /path/to/config.toml`

---

## [screensaver]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `type` | string | `"blank"` | `blank` \| `gif` \| `image` \| `text` |

### type = "gif"

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `path` | string | — | Path to GIF file. `~` is expanded. |
| `scale` | string | `"fit"` | `fit` \| `fill` \| `original` |

### type = "image"

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `path` | string | — | Path to image (PNG/JPEG/BMP/WebP). `~` is expanded. |
| `scale` | string | `"fit"` | `fit` \| `fill` \| `original` |

### type = "text"

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `content` | string | — | Text to display |
| `font_size` | float | `48.0` | Font size in pixels |
| `color` | string | `"#ffffff"` | Hex color |
| `animation` | string | `"float"` | `float` \| `typewriter` \| `fade` |

---

## [display]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `monitor` | int | `0` | Monitor index. `scr33ny monitors` to list. |
| `burn_shift` | int | `2` | Pixel offset applied for OLED burn-in prevention |
| `shift_interval` | int | `120` | Seconds between shifts |

---

## [daemon]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `false` | Enable idle daemon |
| `idle_timeout` | int | `300` | Seconds of inactivity before activation |

---

## [[widgets]]

Each widget is declared as a `[[widgets]]` array entry.

### Common fields

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `type` | string | — | Widget type |
| `font_size` | float | `48.0` | Font size in pixels |
| `color` | string | `"#ffffff"` | Hex color |
| `[widgets.position]` | table | center/center | Positioning (see below) |

### Position

```toml
[widgets.position]
x = "center"   # left | center | right | <0-100 percent>
y = "center"   # top  | center | bottom | <0-100 percent>
```

### type = "clock"

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `style` | string | `"digital"` | `digital` \| `analog` |
| `show_seconds` | bool | `false` | Show second hand / seconds in digital mode |

### type = "date"

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `format` | string | `"%A, %B %d"` | [strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) format |

### type = "weather"

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `api_key` | string | — | OpenWeatherMap API key |
| `location` | string | — | City name or `"lat,lon"` |
| `units` | string | `"metric"` | `metric` (°C) \| `imperial` (°F) |

Weather data is fetched on start and refreshed every 30 minutes.

---

## Color format

All color fields accept hex strings: `"#rrggbb"` or `"rrggbb"`.

Examples: `"#ffffff"`, `"#ff6b35"`, `"aaaaaa"`

---

## Full example

```toml
[screensaver]
type      = "text"
content   = "scr33ny"
font_size = 96.0
color     = "#ffffff"
animation = "float"

[display]
monitor        = 0
burn_shift     = 2
shift_interval = 120

[daemon]
enabled      = false
idle_timeout = 300

[[widgets]]
type         = "clock"
style        = "digital"
font_size    = 72.0
color        = "#ffffff"
show_seconds = true

[widgets.position]
x = "center"
y = "center"

[[widgets]]
type      = "date"
font_size = 28.0
color     = "#888888"
format    = "%A, %B %d"

[widgets.position]
x = "center"
y = 62.0
```
