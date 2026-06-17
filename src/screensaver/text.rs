use fontdue::Font;
use crate::{canvas::Canvas, config::{TextAnimation, parse_hex_color}};
use super::Screensaver;

pub struct TextScreensaver {
    content:   String,
    font:      Font,
    font_size: f32,
    color:     (u8, u8, u8),
    animation: TextAnimation,
    time:      f64,
    // Lissajous orbit for anti-burn-in floating
    lx: f64,
    ly: f64,
}

impl TextScreensaver {
    pub fn new(content: String, font: Font, font_size: f32, color: &str, animation: TextAnimation) -> Self {
        let color = parse_hex_color(color);
        // random-ish starting phase so it doesn't start center every time
        Self {
            content,
            font,
            font_size,
            color,
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

        let (ox, oy) = match self.animation {
            TextAnimation::Float => {
                // Lissajous curve: a=3, b=2 gives a nice non-repeating pattern
                let margin_x = (cw * 0.3) as f64;
                let margin_y = (ch * 0.3) as f64;
                let cx = cw as f64 / 2.0 - tw as f64 / 2.0;
                let cy = ch as f64 / 2.0 - th as f64 / 2.0;
                let x  = cx + margin_x * (3.0 * self.time * 0.05 + self.lx).sin();
                let y  = cy + margin_y * (2.0 * self.time * 0.05 + self.ly).sin();
                (x as i32, y as i32)
            }
            TextAnimation::Typewriter => {
                let cx = (cw / 2.0 - tw as f32 / 2.0) as i32;
                let cy = (ch / 2.0 - th as f32 / 2.0) as i32;
                (cx, cy)
            }
            TextAnimation::Fade => {
                let cx = (cw / 2.0 - tw as f32 / 2.0) as i32;
                let cy = (ch / 2.0 - th as f32 / 2.0) as i32;
                (cx, cy)
            }
        };

        let (r, g, b) = self.color;

        match self.animation {
            TextAnimation::Float => {
                canvas.draw_text(ox, oy + th as i32, &self.content, &self.font, self.font_size, r, g, b);
            }
            TextAnimation::Typewriter => {
                // show chars one by one, cycling every 3 seconds per char
                let chars: Vec<char> = self.content.chars().collect();
                let visible = ((self.time * 2.0) as usize % (chars.len() * 2 + 4)).min(chars.len());
                let partial: String = chars[..visible].iter().collect();
                canvas.draw_text(ox, oy + th as i32, &partial, &self.font, self.font_size, r, g, b);
                // blinking cursor
                if (self.time * 2.0) as u32 % 2 == 0 && visible < chars.len() {
                    let (pw, _) = Canvas::measure_text(&partial, &self.font, self.font_size);
                    canvas.fill_rect(ox + pw as i32, oy, 3, th, r, g, b, 200);
                }
            }
            TextAnimation::Fade => {
                // fade in/out cycle: 3s in, 1s hold, 3s out, 1s black
                let cycle = 8.0_f64;
                let t = self.time % cycle;
                let alpha: u8 = if t < 3.0 {
                    (t / 3.0 * 255.0) as u8
                } else if t < 4.0 {
                    255
                } else if t < 7.0 {
                    ((7.0 - t) / 3.0 * 255.0) as u8
                } else {
                    0
                };
                // draw to temp layer
                let ar = (r as u32 * alpha as u32 / 255) as u8;
                let ag = (g as u32 * alpha as u32 / 255) as u8;
                let ab = (b as u32 * alpha as u32 / 255) as u8;
                canvas.draw_text(ox, oy + th as i32, &self.content, &self.font, self.font_size, ar, ag, ab);
            }
        }
    }
}
