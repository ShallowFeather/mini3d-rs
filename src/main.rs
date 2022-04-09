mod device;
mod vector_calc;
mod transform_calc;
mod calc;
mod matrix_calc;
mod vertex;

use minifb::{InputCallback, Key, Menu, Scale, Window, WindowOptions};
use crate::calc::swap;
use crate::device::Device;

const WIDTH: usize = 500;
const HEIGHT: usize = 400;


fn main() {
    let mut device = Device::init("owo", WIDTH, HEIGHT);
    let mut a = 5 as usize;
    let mut b = 4 as usize;

    while device.window.is_open() && !device.window.is_key_down(Key::Escape) {
        device.draw_line(40,60 , 430, 320, 0xff0c0c);

        device.window.update_with_buffer(&device.framebuf, WIDTH, HEIGHT).unwrap();
    }
}