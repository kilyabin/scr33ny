use anyhow::{Context, Result};
use image::RgbaImage;
use std::path::Path;

use crate::{canvas::Canvas, config::Scale};
use super::Screensaver;

pub struct ImageScreensaver {
    image: RgbaImage,
    scale: Scale,
}

impl ImageScreensaver {
    pub fn load(path: &Path, scale: Scale) -> Result<Self> {
        let img = image::open(path)
            .with_context(|| format!("opening image {}", path.display()))?
            .to_rgba8();
        Ok(Self { image: img, scale })
    }
}

impl Screensaver for ImageScreensaver {
    fn render(&mut self, canvas: &mut Canvas, _dt: f64) {
        canvas.clear();
        let (iw, ih) = (self.image.width(), self.image.height());
        let (cw, ch) = (canvas.width, canvas.height);

        match self.scale {
            Scale::Original => {
                let x = (cw as i32 - iw as i32) / 2;
                let y = (ch as i32 - ih as i32) / 2;
                canvas.draw_image(x, y, &self.image);
            }
            Scale::Fit => {
                let scale = (cw as f32 / iw as f32).min(ch as f32 / ih as f32);
                let nw = (iw as f32 * scale) as u32;
                let nh = (ih as f32 * scale) as u32;
                let x  = (cw as i32 - nw as i32) / 2;
                let y  = (ch as i32 - nh as i32) / 2;
                canvas.draw_image_scaled(x, y, nw, nh, &self.image);
            }
            Scale::Fill => {
                let scale = (cw as f32 / iw as f32).max(ch as f32 / ih as f32);
                let nw = (iw as f32 * scale) as u32;
                let nh = (ih as f32 * scale) as u32;
                let x  = (cw as i32 - nw as i32) / 2;
                let y  = (ch as i32 - nh as i32) / 2;
                canvas.draw_image_scaled(x, y, nw, nh, &self.image);
            }
        }
    }
}
