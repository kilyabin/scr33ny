use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

// ── Top-level ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    pub screensaver: ScreensaverConfig,
    pub daemon:      DaemonConfig,
    pub widgets:     Vec<WidgetConfig>,
    pub display:     DisplayConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            screensaver: ScreensaverConfig::Blank,
            daemon:      DaemonConfig::default(),
            widgets:     vec![],
            display:     DisplayConfig::default(),
        }
    }
}

// ── Screensaver ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ScreensaverConfig {
    Blank,
    Gif   { path: String, #[serde(default)] scale: Scale },
    Image { path: String, #[serde(default)] scale: Scale },
    Text  {
        content: String,
        #[serde(default = "default_font_size")] font_size:  f32,
        #[serde(default = "default_white")]     color:      String,
        #[serde(default)]                       animation:  TextAnimation,
    },
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum Scale {
    #[default] Fit,
    Fill,
    Original,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum TextAnimation {
    #[default] Float,
    Typewriter,
    Fade,
}

// ── Widgets ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum WidgetConfig {
    Clock {
        #[serde(default)]                       position:  Position,
        #[serde(default = "default_white")]     color:     String,
        #[serde(default = "default_font_size")] font_size: f32,
        #[serde(default)]                       style:     ClockStyle,
        #[serde(default)]                       show_seconds: bool,
    },
    Date {
        #[serde(default)]                       position:  Position,
        #[serde(default = "default_white")]     color:     String,
        #[serde(default = "default_font_size")] font_size: f32,
        #[serde(default = "default_date_fmt")]  format:    String,
    },
    Weather {
        #[serde(default)]                   position: Position,
        api_key:  String,
        location: String,
        #[serde(default)]                   units:    WeatherUnits,
        #[serde(default = "default_white")] color:    String,
        #[serde(default = "default_font_size")] font_size: f32,
    },
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum ClockStyle {
    #[default] Digital,
    Analog,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum WeatherUnits {
    #[default] Metric,
    Imperial,
}

// ── Positioning ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct Position {
    #[serde(default = "default_center")] pub x: Anchor,
    #[serde(default = "default_center")] pub y: Anchor,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: default_center(), y: default_center() }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Anchor {
    Named(NamedAnchor),
    Percent(f32),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum NamedAnchor {
    Left, Center, Right, Top, Bottom,
}

impl Anchor {
    pub fn resolve(&self, total: u32, size: u32) -> i32 {
        let v = match self {
            Anchor::Named(NamedAnchor::Left | NamedAnchor::Top)   => 0.0,
            Anchor::Named(NamedAnchor::Center)                    => 0.5,
            Anchor::Named(NamedAnchor::Right | NamedAnchor::Bottom) => 1.0,
            Anchor::Percent(p)                                    => *p / 100.0,
        };
        ((total as f32 * v) as i32) - (size as i32 / 2)
    }
}

// ── Display ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct DisplayConfig {
    /// Monitor index (0 = primary)
    pub monitor: usize,
    /// Anti-burn-in pixel shift in pixels, applied every shift_interval seconds
    pub burn_shift:     u32,
    pub shift_interval: u64,
    /// Target frames per second. Lower = less CPU. Blank/image need only 1–2.
    pub fps: u32,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self { monitor: 0, burn_shift: 2, shift_interval: 120, fps: 30 }
    }
}

// ── Daemon ───────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct DaemonConfig {
    pub enabled:      bool,
    /// Idle timeout in seconds before screensaver activates
    pub idle_timeout: u64,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self { enabled: false, idle_timeout: 300 }
    }
}

// ── Loading ──────────────────────────────────────────────────────────────────

pub fn load(path: Option<&Path>) -> Result<Config> {
    let path = match path {
        Some(p) => p.to_path_buf(),
        None    => default_config_path(),
    };

    if !path.exists() {
        log::info!("no config at {}, using defaults", path.display());
        return Ok(Config::default());
    }

    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("reading config {}", path.display()))?;

    toml::from_str(&text)
        .with_context(|| format!("parsing config {}", path.display()))
}

pub fn default_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("scr33ny")
        .join("config.toml")
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn default_font_size() -> f32  { 48.0 }
fn default_white()    -> String { "#ffffff".to_string() }
fn default_date_fmt() -> String { "%A, %B %d".to_string() }
fn default_center()   -> Anchor { Anchor::Named(NamedAnchor::Center) }

pub fn parse_hex_color(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let n   = u32::from_str_radix(hex, 16).unwrap_or(0xffffff);
    (((n >> 16) & 0xff) as u8, ((n >> 8) & 0xff) as u8, (n & 0xff) as u8)
}
