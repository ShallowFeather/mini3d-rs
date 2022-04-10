mod device;
mod vector_calc;
mod transform_calc;
mod calc;
mod matrix_calc;
mod vertex;

use minifb::{InputCallback, Key, Menu, Scale, Window, WindowOptions};
use crate::calc::swap;
use crate::device::Device;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const RENDER_STATE_WIREFRAME: i32 = 1;
const RENDER_STATE_TEXTURE: i32 = 2;
const RENDER_STATE_COLOR: i32 = 4;

fn main() {
    let mut device = Device::init("owo", WIDTH, HEIGHT);
    device.camera_at_zero(3., 0., 0.);
    device.init_texture();
    device.render_state = RENDER_STATE_TEXTURE;
    while device.window.is_open() && !device.window.is_key_down(Key::Escape) {
        device.clear(1);
        device.camera_at_zero(3.5, 0., 0.);
        //device.draw_box(1.);
        //device.draw_line(40,60, 430, 320, 0xFF);
        device.draw_box(1.);
        device.window.update_with_buffer(&device.framebuf, WIDTH, HEIGHT).unwrap();
    }
}