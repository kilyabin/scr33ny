use fontdue::{Font, FontSettings};
use image::RgbaImage;

// ── Color ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

// ── Canvas ────────────────────────────────────────────────────────────────────
// Pixel buffer: each u32 = 0x00RRGGBB (softbuffer format)

pub struct Canvas {
    pub buf:    Vec<u32>,
    pub width:  u32,
    pub height: u32,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self { buf: vec![0u32; (width * height) as usize], width, height }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width  = width;
        self.height = height;
        self.buf.resize((width * height) as usize, 0);
    }

    pub fn clear(&mut self) {
        self.buf.fill(0);
    }

    #[inline]
    pub fn set_pixel(&mut self, x: i32, y: i32, c: Color) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 { return; }
        let idx = y as usize * self.width as usize + x as usize;
        if c.a == 255 {
            self.buf[idx] = pack(c.r, c.g, c.b);
        } else if c.a > 0 {
            let dst = self.buf[idx];
            let dr  = ((dst >> 16) & 0xff) as u8;
            let dg  = ((dst >> 8)  & 0xff) as u8;
            let db  = (dst & 0xff) as u8;
            let af  = c.a as u32;
            let or_ = ((c.r as u32 * af + dr as u32 * (255 - af)) / 255) as u8;
            let og  = ((c.g as u32 * af + dg as u32 * (255 - af)) / 255) as u8;
            let ob  = ((c.b as u32 * af + db as u32 * (255 - af)) / 255) as u8;
            self.buf[idx] = pack(or_, og, ob);
        }
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: u32, h: u32, c: Color) {
        for dy in 0..h as i32 {
            for dx in 0..w as i32 {
                self.set_pixel(x + dx, y + dy, c);
            }
        }
    }

    pub fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, c: Color) {
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx * dx + dy * dy <= radius * radius {
                    self.set_pixel(cx + dx, cy + dy, c);
                }
            }
        }
    }

    pub fn draw_ring(&mut self, cx: i32, cy: i32, outer: i32, inner: i32, c: Color) {
        for dy in -outer..=outer {
            for dx in -outer..=outer {
                let d2 = dx * dx + dy * dy;
                if d2 <= outer * outer && d2 >= inner * inner {
                    self.set_pixel(cx + dx, cy + dy, c);
                }
            }
        }
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, thickness: u32, c: Color) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;
        let (mut x, mut y) = (x0, y0);
        let t = thickness as i32;
        loop {
            for ty in -(t / 2)..=(t / 2) {
                for tx in -(t / 2)..=(t / 2) {
                    self.set_pixel(x + tx, y + ty, c);
                }
            }
            if x == x1 && y == y1 { break; }
            let e2 = 2 * err;
            if e2 > -dy { err -= dy; x += sx; }
            if e2 <  dx { err += dx; y += sy; }
        }
    }

    pub fn draw_image(&mut self, x: i32, y: i32, img: &RgbaImage) {
        for (px, py, p) in img.enumerate_pixels() {
            self.set_pixel(x + px as i32, y + py as i32, Color::rgba(p[0], p[1], p[2], p[3]));
        }
    }

    pub fn draw_image_scaled(&mut self, x: i32, y: i32, w: u32, h: u32, img: &RgbaImage) {
        let sw = img.width();
        let sh = img.height();
        for dy in 0..h {
            for dx in 0..w {
                let sx = ((dx as f32 / w as f32) * sw as f32) as u32;
                let sy = ((dy as f32 / h as f32) * sh as f32) as u32;
                let p  = img.get_pixel(sx.min(sw - 1), sy.min(sh - 1));
                self.set_pixel(x + dx as i32, y + dy as i32, Color::rgba(p[0], p[1], p[2], p[3]));
            }
        }
    }

    /// Render text; coverage from font bitmap is multiplied with `c.a`.
    /// Returns (rendered_width, rendered_height).
    pub fn draw_text(&mut self, x: i32, y: i32, text: &str, font: &Font, size: f32, c: Color) -> (u32, u32) {
        let mut cursor = x;
        let mut max_h  = 0u32;
        for ch in text.chars() {
            let (metrics, bitmap) = font.rasterize(ch, size);
            let bx = cursor + metrics.xmin;
            let by = y - metrics.height as i32 - metrics.ymin;
            for py in 0..metrics.height {
                for px in 0..metrics.width {
                    let cov = bitmap[py * metrics.width + px];
                    if cov > 0 {
                        let a = ((cov as u32 * c.a as u32) / 255) as u8;
                        self.set_pixel(bx + px as i32, by + py as i32, Color::rgba(c.r, c.g, c.b, a));
                    }
                }
            }
            cursor += metrics.advance_width as i32;
            max_h   = max_h.max(metrics.height as u32);
        }
        ((cursor - x) as u32, max_h)
    }

    pub fn measure_text(text: &str, font: &Font, size: f32) -> (u32, u32) {
        let mut w = 0u32;
        let mut h = 0u32;
        for ch in text.chars() {
            let m = font.metrics(ch, size);
            w += m.advance_width as u32;
            h  = h.max(m.height as u32);
        }
        (w, h)
    }

    pub fn draw_rounded_rect(&mut self, x: i32, y: i32, w: u32, h: u32, radius: u32, c: Color) {
        let rad = radius as i32;
        self.fill_rect(x + rad, y, w - 2 * radius, h, c);
        self.fill_rect(x, y + rad, w, h - 2 * radius, c);
        let corners = [
            (x + rad,                y + rad),
            (x + w as i32 - rad - 1, y + rad),
            (x + rad,                y + h as i32 - rad - 1),
            (x + w as i32 - rad - 1, y + h as i32 - rad - 1),
        ];
        for (cx, cy) in corners {
            self.draw_circle(cx, cy, rad, c);
        }
    }
}

#[inline]
fn pack(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | b as u32
}

// ── Font loading ─────────────────────────────────────────────────────────────

pub fn load_system_font() -> Option<Font> {
    let candidates = [
        "/usr/share/fonts/TTF/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/TTF/DejaVuSans-Bold.ttf",
        "/usr/share/fonts/noto/NotoSans-Regular.ttf",
        "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
        "/usr/share/fonts/liberation/LiberationSans-Regular.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
        "/usr/share/fonts/TTF/Hack-Regular.ttf",
        "/usr/share/fonts/truetype/freefont/FreeSans.ttf",
        "/usr/share/fonts/OTF/SFNSDisplay.otf",
        "/usr/share/fonts/TTF/Ubuntu-R.ttf",
        "/usr/share/fonts/truetype/ubuntu/Ubuntu-R.ttf",
    ];

    let home_fonts = dirs::home_dir()
        .map(|h| h.join(".local/share/fonts"))
        .unwrap_or_default();
    let user_candidates: Vec<String> = if home_fonts.exists() {
        walkdir_fonts(&home_fonts)
    } else {
        vec![]
    };

    for path in candidates.iter().map(|s| s.to_string()).chain(user_candidates) {
        if let Ok(data) = std::fs::read(&path) {
            if let Ok(font) = Font::from_bytes(data.as_slice(), FontSettings::default()) {
                log::info!("loaded font: {}", path);
                return Some(font);
            }
        }
    }
    None
}

fn walkdir_fonts(dir: &std::path::Path) -> Vec<String> {
    let mut out = vec![];
    if let Ok(rd) = std::fs::read_dir(dir) {
        for entry in rd.flatten() {
            let p = entry.path();
            if let Some(ext) = p.extension() {
                if matches!(ext.to_str(), Some("ttf") | Some("otf")) {
                    out.push(p.to_string_lossy().to_string());
                }
            }
        }
    }
    out
}
