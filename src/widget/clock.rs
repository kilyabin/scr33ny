use chrono::Local;
use fontdue::Font;
use std::f64::consts::PI;

use crate::{
    canvas::Canvas,
    config::{ClockStyle, Position, parse_hex_color},
};
use super::Widget;

pub struct ClockWidget {
    font:         Font,
    font_size:    f32,
    color:        (u8, u8, u8),
    position:     Position,
    style:        ClockStyle,
    show_seconds: bool,
}

impl ClockWidget {
    pub fn new(font: Font, font_size: f32, color: &str, position: Position, style: ClockStyle, show_seconds: bool) -> Self {
        Self {
            font,
            font_size,
            color: parse_hex_color(color),
            position,
            style,
            show_seconds,
        }
    }
}

impl Widget for ClockWidget {
    fn render(&mut self, canvas: &mut Canvas, _dt: f64) {
        let now = Local::now();
        let (r, g, b) = self.color;

        match self.style {
            ClockStyle::Digital => render_digital(self, canvas, &now, r, g, b),
            ClockStyle::Analog  => render_analog(self, canvas, &now, r, g, b),
        }
    }
}

fn render_digital(w: &ClockWidget, canvas: &mut Canvas, now: &chrono::DateTime<chrono::Local>, r: u8, g: u8, b: u8) {
    let text = if w.show_seconds {
        now.format("%H:%M:%S").to_string()
    } else {
        now.format("%H:%M").to_string()
    };

    let (tw, th) = Canvas::measure_text(&text, &w.font, w.font_size);
    let x = w.position.x.resolve(canvas.width, tw);
    let y = w.position.y.resolve(canvas.height, th) + th as i32;

    // Material 3 card background — very slightly lifted from black
    let pad = (w.font_size * 0.4) as u32;
    canvas.draw_rounded_rect(
        x - pad as i32,
        y - th as i32 - pad as i32,
        tw + pad * 2,
        th + pad * 2,
        (w.font_size * 0.25) as u32,
        10, 10, 10, 200,
    );

    canvas.draw_text(x, y, &text, &w.font, w.font_size, r, g, b);
}

fn render_analog(w: &ClockWidget, canvas: &mut Canvas, now: &chrono::DateTime<chrono::Local>, r: u8, g: u8, b: u8) {
    use chrono::Timelike;

    let size   = (w.font_size * 3.0) as i32;
    let cx     = w.position.x.resolve(canvas.width, size as u32 * 2) + size;
    let cy     = w.position.y.resolve(canvas.height, size as u32 * 2) + size;
    let radius = size;

    // Face ring
    canvas.draw_ring(cx, cy, radius, radius - 4, r / 3, g / 3, b / 3, 180);

    // Hour ticks
    for i in 0..12 {
        let angle = i as f64 * PI / 6.0 - PI / 2.0;
        let inner = (radius as f64 * 0.85) as i32;
        let outer = radius - 2;
        let x0 = cx + (angle.cos() * inner as f64) as i32;
        let y0 = cy + (angle.sin() * inner as f64) as i32;
        let x1 = cx + (angle.cos() * outer as f64) as i32;
        let y1 = cy + (angle.sin() * outer as f64) as i32;
        canvas.draw_line(x0, y0, x1, y1, 2, r / 2, g / 2, b / 2, 255);
    }

    // Hands
    let h = now.hour() as f64 % 12.0;
    let m = now.minute() as f64;
    let s = now.second() as f64;

    let hour_angle   = (h / 12.0 + m / 720.0) * 2.0 * PI - PI / 2.0;
    let minute_angle = (m / 60.0 + s / 3600.0) * 2.0 * PI - PI / 2.0;
    let second_angle = (s / 60.0) * 2.0 * PI - PI / 2.0;

    let hr  = (radius as f64 * 0.55) as i32;
    let mr  = (radius as f64 * 0.78) as i32;
    let sr  = (radius as f64 * 0.88) as i32;

    // hour hand
    canvas.draw_line(cx, cy,
        cx + (hour_angle.cos() * hr as f64) as i32,
        cy + (hour_angle.sin() * hr as f64) as i32,
        4, r, g, b, 255);
    // minute hand
    canvas.draw_line(cx, cy,
        cx + (minute_angle.cos() * mr as f64) as i32,
        cy + (minute_angle.sin() * mr as f64) as i32,
        3, r, g, b, 220);
    // second hand (accent color — Material 3 tertiary)
    if w.show_seconds {
        canvas.draw_line(cx, cy,
            cx + (second_angle.cos() * sr as f64) as i32,
            cy + (second_angle.sin() * sr as f64) as i32,
            2, 255, 120, 80, 220);
    }

    // center dot
    canvas.draw_circle(cx, cy, 5, r, g, b, 255);
}
