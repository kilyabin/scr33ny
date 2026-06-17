use fontdue::Font;
use crate::{canvas::{Canvas, Color}, config::{TextAnimation, parse_hex_color}};
use super::Screensaver;

pub struct TextScreensaver {
    content:   String,
    font:      Font,
    font_size: f32,
    color:     Color,
    animation: TextAnimation,
    time:      f64,
    lx:        f64,
    ly:        f64,
}

impl TextScreensaver {
    pub fn new(content: String, font: Font, font_size: f32, color: &str, animation: TextAnimation) -> Self {
        let (r, g, b) = parse_hex_color(color);
        Self {
            content,
            font,
            font_size,
            color: Color::rgb(r, g, b),
            animation,
            time: 0.0,
            lx: 0.0,
            ly: std::f64::consts::FRAC_PI_2,
        }
    }
}

impl Screensaver for TextScreensaver {
    fn render(&mut self, canvas: &mut Canvas, dt: f64) {
        canvas.clear();
        self.time += dt;

        let (cw, ch) = (canvas.width as f32, canvas.height as f32);
        let (tw, th) = Canvas::measure_text(&self.content, &self.font, self.font_size);
        let c        = self.color;

        let (ox, oy) = match self.animation {
            TextAnimation::Float => {
                let margin_x = (cw * 0.3) as f64;
                let margin_y = (ch * 0.3) as f64;
                let cx = cw as f64 / 2.0 - tw as f64 / 2.0;
                let cy = ch as f64 / 2.0 - th as f64 / 2.0;
                (
                    (cx + margin_x * (3.0 * self.time * 0.05 + self.lx).sin()) as i32,
                    (cy + margin_y * (2.0 * self.time * 0.05 + self.ly).sin()) as i32,
                )
            }
            TextAnimation::Typewriter | TextAnimation::Fade => (
                (cw / 2.0 - tw as f32 / 2.0) as i32,
                (ch / 2.0 - th as f32 / 2.0) as i32,
            ),
        };

        match self.animation {
            TextAnimation::Float => {
                canvas.draw_text(ox, oy + th as i32, &self.content, &self.font, self.font_size, c);
            }
            TextAnimation::Typewriter => {
                let chars: Vec<char> = self.content.chars().collect();
                let visible = ((self.time * 2.0) as usize % (chars.len() * 2 + 4)).min(chars.len());
                let partial: String = chars[..visible].iter().collect();
                canvas.draw_text(ox, oy + th as i32, &partial, &self.font, self.font_size, c);
                if ((self.time * 2.0) as u32).is_multiple_of(2) && visible < chars.len() {
                    let (pw, _) = Canvas::measure_text(&partial, &self.font, self.font_size);
                    canvas.fill_rect(ox + pw as i32, oy, 3, th, Color::rgba(c.r, c.g, c.b, 200));
                }
            }
            TextAnimation::Fade => {
                let t = self.time % 8.0;
                let alpha: u8 = if t < 3.0 { (t / 3.0 * 255.0) as u8 }
                    else if t < 4.0 { 255 }
                    else if t < 7.0 { ((7.0 - t) / 3.0 * 255.0) as u8 }
                    else { 0 };
                canvas.draw_text(ox, oy + th as i32, &self.content, &self.font, self.font_size, Color::rgba(c.r, c.g, c.b, alpha));
            }
        }
    }
}
