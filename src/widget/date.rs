use chrono::Local;
use fontdue::Font;

use crate::{
    canvas::{Canvas, Color},
    config::{Position, parse_hex_color},
};
use super::Widget;

pub struct DateWidget {
    font:      Font,
    font_size: f32,
    color:     Color,
    position:  Position,
    format:    String,
}

impl DateWidget {
    pub fn new(font: Font, font_size: f32, color: &str, position: Position, format: String) -> Self {
        let (r, g, b) = parse_hex_color(color);
        Self { font, font_size, color: Color::rgb(r, g, b), position, format }
    }
}

impl Widget for DateWidget {
    fn render(&mut self, canvas: &mut Canvas, _dt: f64) {
        let text     = Local::now().format(&self.format).to_string();
        let (tw, th) = Canvas::measure_text(&text, &self.font, self.font_size);
        let x        = self.position.x.resolve(canvas.width, tw);
        let y        = self.position.y.resolve(canvas.height, th) + th as i32;
        canvas.draw_text(x, y, &text, &self.font, self.font_size, self.color);
    }
}
