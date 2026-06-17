use fontdue::Font;
use serde::Deserialize;
use std::sync::{Arc, Mutex};

use crate::{
    canvas::{Canvas, Color},
    config::{Position, WeatherUnits, parse_hex_color},
};
use super::Widget;

#[derive(Deserialize)]
struct OWMResponse {
    main:    OWMMain,
    weather: Vec<OWMWeather>,
    name:    String,
}

#[derive(Deserialize)]
struct OWMMain {
    temp:     f32,
    humidity: u32,
}

#[derive(Deserialize)]
struct OWMWeather {
    description: String,
    icon:        String,
}

struct WeatherData {
    temp:        f32,
    humidity:    u32,
    description: String,
    icon:        String,
    city:        String,
}

pub struct WeatherWidget {
    font:       Font,
    font_size:  f32,
    color:      Color,
    position:   Position,
    api_key:    String,
    location:   String,
    units:      WeatherUnits,
    data:       Arc<Mutex<Option<WeatherData>>>,
    refresh_in: f64,
}

impl WeatherWidget {
    pub fn new(font: Font, font_size: f32, color: &str, position: Position, api_key: String, location: String, units: WeatherUnits) -> Self {
        let (r, g, b) = parse_hex_color(color);
        let widget = Self {
            font,
            font_size,
            color: Color::rgb(r, g, b),
            position,
            api_key,
            location,
            units,
            data: Arc::new(Mutex::new(None)),
            refresh_in: 0.0,
        };
        widget.fetch_async();
        widget
    }

    fn fetch_async(&self) {
        let data     = Arc::clone(&self.data);
        let api_key  = self.api_key.clone();
        let location = self.location.clone();
        let units    = match self.units { WeatherUnits::Metric => "metric", WeatherUnits::Imperial => "imperial" };

        tokio::spawn(async move {
            let url = format!(
                "https://api.openweathermap.org/data/2.5/weather?q={location}&appid={api_key}&units={units}"
            );
            match reqwest::get(&url).await {
                Ok(resp) => match resp.json::<OWMResponse>().await {
                    Ok(json) => {
                        let w = json.weather.into_iter().next();
                        *data.lock().unwrap() = Some(WeatherData {
                            temp:        json.main.temp,
                            humidity:    json.main.humidity,
                            description: w.as_ref().map(|x| x.description.clone()).unwrap_or_default(),
                            icon:        w.as_ref().map(|x| x.icon.clone()).unwrap_or_default(),
                            city:        json.name,
                        });
                    }
                    Err(e) => log::warn!("weather parse error: {e}"),
                },
                Err(e) => log::warn!("weather fetch error: {e}"),
            }
        });
    }
}

impl Widget for WeatherWidget {
    fn render(&mut self, canvas: &mut Canvas, dt: f64) {
        self.refresh_in -= dt;
        if self.refresh_in <= 0.0 {
            self.refresh_in = 1800.0;
            self.fetch_async();
        }

        let unit_sym = match self.units { WeatherUnits::Metric => "°C", WeatherUnits::Imperial => "°F" };
        let lock     = self.data.lock().unwrap();
        let text     = match lock.as_ref() {
            None    => "Loading weather…".to_string(),
            Some(d) => format!(
                "{} {} {:.0}{unit_sym}  H:{:.0}%  {}",
                d.city, icon_char(&d.icon), d.temp, d.humidity, d.description,
            ),
        };

        let (tw, th) = Canvas::measure_text(&text, &self.font, self.font_size);
        let pad      = (self.font_size * 0.4) as u32;
        let x        = self.position.x.resolve(canvas.width, tw);
        let y        = self.position.y.resolve(canvas.height, th) + th as i32;

        canvas.draw_rounded_rect(
            x - pad as i32,
            y - th as i32 - pad as i32,
            tw + pad * 2,
            th + pad * 2,
            (self.font_size * 0.25) as u32,
            Color::rgba(10, 10, 10, 200),
        );
        canvas.draw_text(x, y, &text, &self.font, self.font_size, self.color);
    }
}

fn icon_char(icon: &str) -> &'static str {
    match icon {
        s if s.starts_with("01") => "☀",
        s if s.starts_with("02") => "⛅",
        s if s.starts_with("03") | s.starts_with("04") => "☁",
        s if s.starts_with("09") => "🌧",
        s if s.starts_with("10") => "🌦",
        s if s.starts_with("11") => "⛈",
        s if s.starts_with("13") => "❄",
        s if s.starts_with("50") => "🌫",
        _                         => "~",
    }
}
