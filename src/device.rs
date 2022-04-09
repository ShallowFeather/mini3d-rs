use std::mem::swap;
use minifb::*;
use crate::transform_calc::Transform;
use crate::{HEIGHT, WIDTH};
use crate::calc::CMID;

pub struct Device {
    pub transform: Transform,
    pub window: minifb::Window, // get_size = (width, height)
    pub framebuf: Vec<u32>,
    pub texture: Vec<u32>,
    pub tex_width: i32,
    pub tex_height: i32,
    pub max_u: f32,
    pub max_v: f32,
    pub render_state: i32,
    pub background: u32,
    pub foreground: u32,
}

const RENDER_STATE_WIREFRAME: i32 = 1;
const RENDER_STATE_TEXTURE: i32 = 2;
const RENDER_STATE_COLOR: i32 = 4;

pub struct RGB {
    pub(crate) R: u32,
    pub(crate) G: u32,
    pub(crate) B: u32,
}

pub fn hex_to_rgb(hex: u32) -> RGB {
    let ret = RGB {
        R: ((hex >> 16) & 0xFF)  / 255. as u32,
        G: ((hex >> 8) & 0xFF) / 255. as u32,
        B: ((hex) & 0xFF) / 255. as u32,
    };
    return ret;
}

pub fn rgb_to_hex(rgb: RGB) -> u32 {
    return (((rgb.R & 0xFF) << 16) + ((rgb.G & 0xFF) << 8) + (rgb.B & 0xFF)) as u32;
}


impl Device {
    pub fn init(name: &str, width: usize, height: usize) -> Device {
        let mut device : Device = Device {
            transform: Transform::init(width, height),
            window: minifb::Window::new(
                name,
                width,
                height,
                WindowOptions {
                    scale: minifb::Scale::X2,
                    ..WindowOptions::default()
                },).unwrap(),
            framebuf: vec![0b00000000_00000000_00000000_00000000; width * height],
            texture: vec![0; width * height],
            tex_width: 2,
            tex_height: 2,
            max_u: 1.0,
            max_v: 1.0,
            render_state: 0,
            background: 0b00000000_00000000_00000000_00000000,
            foreground: 0
        };
        device.window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        return device;
    }

    pub fn set_texture(&mut self, w: i32, h: i32) {
        self.tex_width = w;
        self.tex_height = h;
        self.max_u = (w - 1) as f32;
        self.max_v = (h - 1) as f32;
    }

    pub fn clear(&mut self, mode: i32) {
        let (width, height) = self.window.get_size();
        let mut buf: Vec<u32>= vec![0; (width * height)];
        for y in 0..height {
            let mut cc: u32 = ((height - 1 - y) * 230 / (height - 1)) as u32;
            cc = (cc << 16) | (cc << 8) | cc;
            if mode == 0 {
                cc = self.background;
            }
            for x in 0..width {
                buf[y * width + x] = cc;
            }
        }
        self.framebuf = buf;
    }

    pub fn pixel(&mut self, x: usize, y: usize, color: u32) {
        self.framebuf[y * WIDTH + x] = color;
    }

    pub fn draw_line(&mut self, mut x1: usize, mut y1: usize, mut x2: usize, mut y2: usize, color: u32) {
        if x1 == x2 && y1 == y2 {
            self.pixel(x1, y1, color);
        } else if x1 == x2 {
            let inc = match y1 < y2 {
                true => { 1 }
                false => { -1 }
            };
            let mut y = y1 as i32;
            while y as usize != y2 {
                self.pixel(x1, y as usize, color);
                y += inc;
            }
            self.pixel(x2, y2, color);
        } else if y1 == y2 {
            let inc = match x1 <= x2 {
                true => { 1 }
                false => { -1 }
            };
            let mut x= x1 as i32;
            while x as usize != x2 {
                self.pixel(x as usize, y1, color);
                x += inc;
            }
            self.pixel(x2, y2, color);
        } else {
            let dx = match x1 < x2 {
                true => { x2 - x1 }
                false => { x1 - x2 }
            };
            let dy = match y1 < y2 {
                true => { y2 - y1 }
                false => { y1 - y2 }
            };
            if dx >= dy {
                if x2 < x1 {
                    swap(&mut x1, &mut x2);
                    swap(&mut y1, &mut y2);
                }
                let mut x = x1 as i32;
                let mut y = y1 as i32;
                let mut rem = 0;
                while x <= x2 as i32{
                    self.pixel(x as usize, y as usize, color);
                    rem += dy;
                    if rem >= dx {
                        rem -= dx;
                        y += match y2 >= y1 {
                            true => { 1 }
                            false => { -1 }
                        };
                        self.pixel(x as usize, y as usize, color);
                    }
                    x += 1;
                }
                self.pixel(x2, y2, color);
            } else {
                if y2 < y1 {
                    swap(&mut x1, &mut x2);
                    swap(&mut y1, &mut y2);
                }
                let mut x = x1 as i32;
                let mut y = y1 as i32;
                let mut rem = 0;
                while y <= y2 as i32 {
                    self.pixel(x as usize, y as usize, color);
                    rem += dx;
                    if rem >= dy {
                        rem -= dy;
                        x += match x2 >= x1 {
                            true => { 1 }
                            false => { -1 }
                        };
                        self.pixel(x as usize, y as usize, color)
                    }
                }
                self.pixel(x2, y2, color);
            }
        }
    }

    pub fn texture_read(self, mut u: f32, mut v: f32) -> u32 {
        let mut x;
        let mut y;
        u *= self.max_u;
        v *= self.max_v;
        x = (u + 0.5) as i32;
        y = (v + 0.5) as i32;
        x = CMID(x, 0, self.tex_width - 1);
        y = CMID(y, 0, self.tex_height - 1);
        return self.texture[(y * self.tex_width + x) as usize];
    }
}
