use minifb;
use minifb::Key;
use crate::device::{Device, RGB};
use crate::transform_calc::Transform;

mod vector_calc;
mod calc;
mod matrix_calc;
mod transform_calc;
mod vertex;
mod device;



fn main() {
    let mut window = device::Device::init("owo", 100, 100);

    while window.window.is_open() && !window.window.is_key_down(Key::Escape) {
        let mut buffer: Vec<u32> = vec![0; 100 * 100];
        window.pixel(0, 1, 0xFF00E436);

    }

    println!("Hello, world!");
}
