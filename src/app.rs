use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;

use anyhow::Result;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Fullscreen, WindowBuilder},
};

use crate::{
    canvas::{Canvas, load_system_font},
    config::{Config, ScreensaverConfig, WidgetConfig},
    screensaver::{BlankScreensaver, GifScreensaver, ImageScreensaver, Screensaver, TextScreensaver},
    widget::{ClockWidget, DateWidget, WeatherWidget, Widget},
};

// ── Anti-burn-in pixel shift ─────────────────────────────────────────────────

struct BurnShift {
    x: i32, y: i32,
    elapsed: f64, interval: f64,
    amount: i32, step: u8,
}

impl BurnShift {
    fn new(amount: u32, interval: u64) -> Self {
        Self { x: 0, y: 0, elapsed: 0.0, interval: interval as f64, amount: amount as i32, step: 0 }
    }

    fn update(&mut self, dt: f64) {
        self.elapsed += dt;
        if self.elapsed >= self.interval {
            self.elapsed = 0.0;
            self.step = (self.step + 1) % 4;
            (self.x, self.y) = match self.step {
                0 => (0,             0),
                1 => (self.amount,   0),
                2 => (self.amount,   self.amount),
                _ => (0,             self.amount),
            };
        }
    }
}

// ── Entry point ──────────────────────────────────────────────────────────────

pub fn run(config: Config, monitor_idx: usize) -> Result<()> {
    let font = load_system_font().ok_or_else(|| anyhow::anyhow!(
        "No system font found. Install DejaVu, Noto, or Liberation fonts."
    ))?;

    let mut screensaver: Box<dyn Screensaver> = build_screensaver(&config, font.clone())?;
    let mut widgets: Vec<Box<dyn Widget>>     = build_widgets(&config, font)?;

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let monitors: Vec<_> = event_loop.available_monitors().collect();
    let monitor = monitors.get(monitor_idx).or_else(|| monitors.first()).cloned();

    // Rc<Window> so softbuffer can hold a reference without borrowing stack var
    let window = Rc::new(
        WindowBuilder::new()
            .with_title("scr33ny")
            .with_fullscreen(Some(Fullscreen::Borderless(monitor)))
            .with_decorations(false)
            .with_resizable(false)
            .build(&event_loop)?
    );
    window.set_cursor_visible(false);

    let context = softbuffer::Context::new(Rc::clone(&window))
        .map_err(|e| anyhow::anyhow!("softbuffer context: {e}"))?;
    let mut surface = softbuffer::Surface::new(&context, Rc::clone(&window))
        .map_err(|e| anyhow::anyhow!("softbuffer surface: {e}"))?;
    let mut canvas  = Canvas::new(window.inner_size().width, window.inner_size().height);

    let mut last_frame = Instant::now();
    let mut burn = BurnShift::new(config.display.burn_shift, config.display.shift_interval);

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),

                    WindowEvent::KeyboardInput { event: KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyQ),
                        state: ElementState::Pressed, ..
                    }, .. } => elwt.exit(),

                    WindowEvent::KeyboardInput { event: KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        state: ElementState::Pressed, ..
                    }, .. } => elwt.exit(),

                    WindowEvent::Resized(size) => {
                        canvas.resize(size.width, size.height);
                    }

                    WindowEvent::RedrawRequested => {
                        let now = Instant::now();
                        let dt  = now.duration_since(last_frame).as_secs_f64().min(0.1);
                        last_frame = now;

                        burn.update(dt);
                        canvas.clear();

                        screensaver.render(&mut canvas, dt);
                        for widget in &mut widgets {
                            widget.render(&mut canvas, dt);
                        }

                        if let (Some(w), Some(h)) = (
                            NonZeroU32::new(canvas.width),
                            NonZeroU32::new(canvas.height),
                        ) {
                            if surface.resize(w, h).is_ok() {
                                if let Ok(mut buf) = surface.buffer_mut() {
                                    apply_shifted(&canvas, &mut buf, burn.x, burn.y);
                                    let _ = buf.present();
                                }
                            }
                        }
                    }

                    _ => {}
                }
            }

            Event::AboutToWait => window.request_redraw(),

            _ => {}
        }
    })?;

    Ok(())
}

/// Copy canvas pixels to surface buffer with burn-in shift
fn apply_shifted(canvas: &Canvas, buf: &mut [u32], sx: i32, sy: i32) {
    let w = canvas.width as i32;
    let h = canvas.height as i32;
    for y in 0..h {
        for x in 0..w {
            let src_x = x - sx;
            let src_y = y - sy;
            buf[(y * w + x) as usize] = if src_x >= 0 && src_y >= 0 && src_x < w && src_y < h {
                canvas.buf[(src_y * w + src_x) as usize]
            } else {
                0
            };
        }
    }
}

// ── Screensaver factory ──────────────────────────────────────────────────────

fn build_screensaver(config: &Config, font: fontdue::Font) -> Result<Box<dyn Screensaver>> {
    Ok(match &config.screensaver {
        ScreensaverConfig::Blank => Box::new(BlankScreensaver),

        ScreensaverConfig::Gif { path, scale } => {
            let p = shellexpand::tilde(path).to_string();
            Box::new(GifScreensaver::load(std::path::Path::new(&p), scale.clone())?)
        }

        ScreensaverConfig::Image { path, scale } => {
            let p = shellexpand::tilde(path).to_string();
            Box::new(ImageScreensaver::load(std::path::Path::new(&p), scale.clone())?)
        }

        ScreensaverConfig::Text { content, font_size, color, animation } => {
            Box::new(TextScreensaver::new(
                content.clone(), font, *font_size, color, animation.clone(),
            ))
        }
    })
}

// ── Widget factory ───────────────────────────────────────────────────────────

fn build_widgets(config: &Config, font: fontdue::Font) -> Result<Vec<Box<dyn Widget>>> {
    let mut out: Vec<Box<dyn Widget>> = vec![];
    for wc in &config.widgets {
        let w: Box<dyn Widget> = match wc {
            WidgetConfig::Clock { position, color, font_size, style, show_seconds } =>
                Box::new(ClockWidget::new(font.clone(), *font_size, color, position.clone(), style.clone(), *show_seconds)),
            WidgetConfig::Date { position, color, font_size, format } =>
                Box::new(DateWidget::new(font.clone(), *font_size, color, position.clone(), format.clone())),
            WidgetConfig::Weather { position, api_key, location, units, color, font_size } =>
                Box::new(WeatherWidget::new(font.clone(), *font_size, color, position.clone(), api_key.clone(), location.clone(), units.clone())),
        };
        out.push(w);
    }
    Ok(out)
}
