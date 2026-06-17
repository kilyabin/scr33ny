mod clock;
mod date;
mod weather;

pub use clock::ClockWidget;
pub use date::DateWidget;
pub use weather::WeatherWidget;

use crate::canvas::Canvas;

pub trait Widget: Send {
    fn render(&mut self, canvas: &mut Canvas, dt: f64);
}
