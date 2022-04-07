use std::alloc::alloc;
use std::mem;
use std::os::raw::c_uchar;
use crate::transform_calc::Transform;
use libc::c_void;
use crate::device;

pub struct Device {
    transform: Transform,
    width: i32,
    height: i32,
    framebuffer: *const u32,
    zbuffer: *const u32,
    texture: *const u32,
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

impl Device {
    pub fn init(&mut self, width: i32, height: i32, fb: *mut c_uchar) {
        unsafe {
            let need = mem::size_of::<u32>()
                * (height * 2 + 1024) as usize
                + (width * height * 8) as usize;
            let mut ptr = libc::malloc(need + 64) as *const u32;
            let framebuf: c_void;
            let zbuf: c_void;
            //let mut j;
            self.framebuffer = ptr;
            let ptr1 = ptr;
            ptr1.add(mem::size_of::<u32>() * height as usize);
            self.zbuffer = ptr1;
            ptr.add(mem::size_of::<u32>() * (height * 2) as usize);
            self.texture = ptr;

        }
    }
}

