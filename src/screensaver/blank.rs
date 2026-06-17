use crate::canvas::Canvas;
use super::Screensaver;

/// Pure black — all OLED pixels off, zero power draw
pub struct BlankScreensaver;

impl Screensaver for BlankScreensaver {
    fn render(&mut self, canvas: &mut Canvas, _dt: f64) {
        canvas.clear();
    }
}
