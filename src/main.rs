mod device;
mod vector_calc;
mod transform_calc;
mod calc;
mod matrix_calc;
mod vertex;

use std::thread::sleep;
use std::time::Duration;
use minifb::{InputCallback, Key, Menu, Scale, Window, WindowOptions};
use minifb::Key::K;
use crate::calc::swap;
use crate::device::Device;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const RENDER_STATE_WIREFRAME: i32 = 1;
const RENDER_STATE_TEXTURE: i32 = 2;
const RENDER_STATE_COLOR: i32 = 4;

fn main() {
    let mut device = Device::init("owo", WIDTH, HEIGHT);
    let arr = [RENDER_STATE_WIREFRAME, RENDER_STATE_TEXTURE, RENDER_STATE_COLOR];
    device.init_texture();
    device.camera_at_zero(3., 0., 0.);
    device.render_state = RENDER_STATE_WIREFRAME;
    let mut a = 0;
    let mut pos = 3.5;
    let mut alpha = 1.;

    let mut kbhit = 0;
    let mut indicator = 0;

    while device.window.is_open() && !device.window.is_key_down(Key::Escape) {
        device.clear(1);
        device.camera_at_zero(pos, 0., 0.);
        if device.window.is_key_down(Key::Up) {
            pos -= 0.01;
        }
        if device.window.is_key_down(Key::Down) {
            pos += 0.01;
        }
        if device.window.is_key_down(Key::Left) {
            alpha += 0.01;
        }
        if device.window.is_key_down(Key::Right) {
            alpha -= 0.01;
        }

        if device.window.is_key_down(Key::Space) {
            if kbhit == 0 {
                kbhit += 1;
                indicator += 1;
                if indicator >= 3 {
                    indicator = 0;
                }
                device.render_state = arr[indicator];
            }
            else {
                kbhit = 0;
            }
        }
        device.draw_box(alpha);
        device.window.update_with_buffer(&device.framebuf, WIDTH, HEIGHT).unwrap();
    }
}