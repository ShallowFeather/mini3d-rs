use minifb::*;
use crate::transform_calc::Transform;

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
                    resize: false,
                    scale: Scale::X32,
                    ..WindowOptions::default()
                },).unwrap(),
            framebuf: vec![0; (width * height) as usize],
            texture: vec![0; (width * height) as usize],
            tex_width: 2,
            tex_height: 2,
            max_u: 1.0,
            max_v: 1.0,
            render_state: 0,
            background: rgb_to_hex(RGB{ R: 30, G: 20, B: 50 }),
            foreground: 0
        };
        device.texture = vec![0; (width * height) as usize];
        device.tex_width = 2;
        device.tex_height = 2;
        device.max_u = 1.0;
        device.max_v = 1.0;
        device.window.set_background_color(255, 20, 50);
        device.background = rgb_to_hex(RGB{ R: 30, G: 20, B: 50 });
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
        self.window.update_with_buffer(&buf, width, height).unwrap();
        self.framebuf = buf;
    }

    pub fn pixel(&mut self, x: i32, y: i32, color: u32) {
        let (width, height) = self.window.get_size();
        for i in 0..self.framebuf.len() {
            self.framebuf[i] = color;
        }
        self.window.update_with_buffer(&self.framebuf, width, height).unwrap();
    }
}
