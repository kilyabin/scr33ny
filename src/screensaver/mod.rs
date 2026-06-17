mod blank;
mod gif;
mod image_ss;
mod text;

pub use blank::BlankScreensaver;
pub use gif::GifScreensaver;
pub use image_ss::ImageScreensaver;
pub use text::TextScreensaver;

use crate::canvas::Canvas;

pub trait Screensaver: Send {
    /// Called every frame. `dt` is elapsed seconds since last frame.
    fn render(&mut self, canvas: &mut Canvas, dt: f64);
}
