use minifb::*;
use crate::transform_calc::Transform;

pub struct Device {
    transform: Transform,
    window: minifb::Window, // get_size = (width, height)
    texture: [[u32; 1024]; 1024],
    tex_width: i32,
    tex_height: i32,
    max_u: f32,
    max_v: f32,
    render_state: i32,
    background: u32,
    foreground: u32,
}

const RENDER_STATE_WIREFRAME: i32 = 1;
const RENDER_STATE_TEXTURE: i32 = 2;
const RENDER_STATE_COLOR: i32 = 4;

pub struct RGB {
    R: i32,
    G: i32,
    B: i32,
}

pub fn hex_to_rgb(hex: i32) -> RGB {
    let ret = RGB {
        R: ((hex >> 16) & 0xFF) / 255.0,
        G: ((hex >> 8) & 0xFF) / 255.0,
        B: ((hex) & 0xFF) / 255.0,
    };
    return et;
}

impl Device {
    pub fn init(&mut self, width: i32, height: i32) {
        self.window = minifb::Window::new(
            "Mini3D-rs",
            width as usize,
            height as usize,
            Default::default()
        ).unwrap();
        self.tex_width = 2;
        self.tex_height = 2;
        self.max_u = 1.0;
        self.max_v = 1.0;
        self.window.set_background_color(30, 20, 50);
    }

    pub fn set_texture(&mut self, w: i32, h: i32) {
        self.tex_width = w;
        self.tex_height = h;
        self.max_u = (w - 1) as f32;
        self.max_v = (h - 1) as f32;
    }

    pub fn clear(&self, mode: i32) {
        let (width, height) = self.window.get_size();
        let width= width as i32;
        let height = height as i32;
        let buffer = [[u32; 1024]; 1024];
        for y in 0..width {
            let buf = buffer[y];
            let mut cc = (height - 1 - y) * 230 / (height - 1);
            cc = (cc << 16) | (cc << 8) | cc;
            if mode == 0 {
                cc = self.background as i32;
            }
            for x in width..0 {
                buf[0] = cc;
                buf += 1;
            }
        }
    }

}
