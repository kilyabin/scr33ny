use anyhow::{Context, Result};
use image::{AnimationDecoder, codecs::gif::GifDecoder, RgbaImage};
use std::{fs::File, io::BufReader, path::Path, time::Duration};

use crate::{canvas::Canvas, config::Scale};
use super::Screensaver;

pub struct GifFrame {
    pub image:  RgbaImage,
    pub delay:  Duration,
}

pub struct GifScreensaver {
    frames:      Vec<GifFrame>,
    current:     usize,
    elapsed:     f64,
    scale:       Scale,
}

impl GifScreensaver {
    pub fn load(path: &Path, scale: Scale) -> Result<Self> {
        let file    = BufReader::new(
            File::open(path).with_context(|| format!("opening GIF {}", path.display()))?
        );
        let decoder = GifDecoder::new(file)
            .context("decoding GIF header")?;
        let frames  = decoder.into_frames()
            .collect_frames()
            .context("collecting GIF frames")?;

        let frames = frames
            .into_iter()
            .map(|f| {
                let delay_ms = f.delay().numer_denom_ms();
                let delay = Duration::from_millis(
                    (delay_ms.0 as u64 * 1000) / delay_ms.1.max(1) as u64,
                );
                GifFrame { image: f.into_buffer(), delay }
            })
            .collect();

        Ok(Self { frames, current: 0, elapsed: 0.0, scale })
    }
}

impl Screensaver for GifScreensaver {
    fn render(&mut self, canvas: &mut Canvas, dt: f64) {
        if self.frames.is_empty() { return; }

        canvas.clear();

        let frame = &self.frames[self.current];
        let (iw, ih) = (frame.image.width(), frame.image.height());
        let (cw, ch) = (canvas.width, canvas.height);

        match self.scale {
            Scale::Original => {
                let x = (cw as i32 - iw as i32) / 2;
                let y = (ch as i32 - ih as i32) / 2;
                canvas.draw_image(x, y, &frame.image);
            }
            Scale::Fit => {
                let scale = (cw as f32 / iw as f32).min(ch as f32 / ih as f32);
                let nw    = (iw as f32 * scale) as u32;
                let nh    = (ih as f32 * scale) as u32;
                let x     = (cw as i32 - nw as i32) / 2;
                let y     = (ch as i32 - nh as i32) / 2;
                canvas.draw_image_scaled(x, y, nw, nh, &frame.image);
            }
            Scale::Fill => {
                let scale = (cw as f32 / iw as f32).max(ch as f32 / ih as f32);
                let nw    = (iw as f32 * scale) as u32;
                let nh    = (ih as f32 * scale) as u32;
                let x     = (cw as i32 - nw as i32) / 2;
                let y     = (ch as i32 - nh as i32) / 2;
                canvas.draw_image_scaled(x, y, nw, nh, &frame.image);
            }
        }

        self.elapsed += dt;
        if self.elapsed >= frame.delay.as_secs_f64() {
            self.elapsed = 0.0;
            self.current = (self.current + 1) % self.frames.len();
        }
    }
}
