use std::borrow::BorrowMut;
use std::mem::swap;
use minifb::*;
use crate::transform_calc::Transform;
use crate::{calc, HEIGHT, WIDTH};
use crate::calc::{CMID, Color, Scanline, Texcoord, Trapezoid, trapezoid_edge_interp, trapezoid_init, trapezoid_init_scan_line, trapezoid_init_triangle};
use crate::matrix_calc::Matrix4f;
use crate::vector_calc::Vector4f;
use crate::vertex::{Edge, Vertex};

pub struct Device {
    pub transform: Transform,
    pub window: minifb::Window, // get_size = (width, height)
    pub framebuf: Vec<u32>,
    pub texture: Vec<Vec<u32>>,
    pub zbuffer: Vec<Vec<f32>>,
    pub tex_width: i32,
    pub tex_height: i32,
    pub max_u: f32,
    pub max_v: f32,
    pub render_state: i32,
    pub background: u32,
    pub foreground: u32,
    pub mesh: Vec<Vertex>
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
            transform: Transform::init(),
            window: minifb::Window::new(
                name,
                width,
                height,
                WindowOptions {
                    scale: minifb::Scale::X1,
                    ..WindowOptions::default()
                },).unwrap(),
            framebuf: vec![0b00000000_00000000_00000000_00000000; width * height],
            texture: vec![vec![0; 256]; 256],
            zbuffer: vec![vec![0.; height]; width],
            tex_width: 2,
            tex_height: 2,
            max_u: 1.0,
            max_v: 1.0,
            render_state: 0,
            background: 0b00000000_00000000_00000000_00000000,
            foreground: 0,
            mesh: vec![
                Vertex {
                    pos: Vector4f { x: -1.0, y: -1.0, z: 1.0, w: 1.0 },
                    tc: Texcoord { u: 0.0, v: 0.0 },
                    color: Color { r: 1.0, g: 0.2, b: 0.2 }, rhw: 1.0
                },
                Vertex {
                    pos: Vector4f { x: 1.0, y: -1.0, z: 1.0, w: 1.0 },
                    tc: Texcoord { u: 0.0, v: 1.0 },
                    color: Color { r: 0.2, g: 1.0, b: 0.2 }, rhw: 1.0
                },
                Vertex {
                    pos: Vector4f { x: 1.0, y: 1.0, z: 1.0, w: 1.0 },
                    tc: Texcoord { u: 1.0, v: 1.0 },
                    color: Color { r: 0.2, g: 0.2, b: 1.0 }, rhw: 1.0
                },
                Vertex {
                    pos: Vector4f { x: -1.0, y: 1.0, z: 1.0, w: 1.0 },
                    tc: Texcoord { u: 1.0, v: 0.0 },
                    color: Color { r: 1.0, g: 0.2, b: 1.0 }, rhw: 1.0
                },
                Vertex {
                    pos: Vector4f { x: -1.0, y: -1.0, z: -1.0, w: 1.0 },
                    tc: Texcoord { u: 0.0, v: 0.0 },
                    color: Color { r: 1.0, g: 1.0, b: 0.2 }, rhw: 1.0
                },
                Vertex {
                    pos: Vector4f { x: 1.0, y: -1.0, z: -1.0, w: 1.0 },
                    tc: Texcoord { u: 0.0, v: 1.0 },
                    color: Color { r: 0.2, g: 1.0, b: 1.0 }, rhw: 1.0
                },
                Vertex {
                    pos: Vector4f { x: 1.0, y: 1.0, z: -1.0, w: 1.0 },
                    tc: Texcoord { u: 1.0, v: 1.0 },
                    color: Color { r: 1.0, g: 0.3, b: 0.3 }, rhw: 1.0
                },
                Vertex {
                    pos: Vector4f { x: -1.0, y: 1.0, z: -1.0, w: 1.0 },
                    tc: Texcoord { u: 1.0, v: 0.0 },
                    color: Color { r: 0.2, g: 1.0, b: 0.3 }, rhw: 1.0
                },
            ]
        };
        return device;
    }

    pub fn set_texture(&mut self, w: i32, h: i32) {
        self.tex_width = w;
        self.tex_height = h;
        self.max_u = (w - 1) as f32;
        self.max_v = (h - 1) as f32;
    }

    pub fn clear(&mut self, mode: i32) {
        let mut buf: Vec<u32>= vec![0; (WIDTH * HEIGHT)];
        for y in 0..HEIGHT {
            let mut cc = ((HEIGHT - 1 - y) * 230 / (HEIGHT - 1)) as u32;
            cc = (cc << 16) | (cc << 8) | cc;
            if mode == 0 {
                cc = self.background;
            }
            for x in 0..WIDTH {
                buf[y * WIDTH + x] = cc;
            }
        }
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.zbuffer[x][y] = 0.;
            }
        }
        self.framebuf = buf;
    }

    pub fn pixel(&mut self, x: usize, y: usize, color: u32) {
        if y < WIDTH && x < HEIGHT {
            self.framebuf[y * WIDTH + x] = color;
        }
    }


    pub fn draw_line(&mut self, mut x1: usize, mut y1: usize, mut x2: usize, mut y2: usize, color: u32) {
        if x1 == x2 && y1 == y2 {
            self.pixel(x1, y1, color);
        } else if x1 == x2 {
            let inc = match y1 <= y2 {
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
                    y += 1;
                }
                self.pixel(x2, y2, color);
            }
        }
    }

    pub fn texture_read(&self, mut u: f32, mut v: f32) -> u32 {
        let mut x;
        let mut y;
        u *= self.max_u;
        v *= self.max_v;
        x = (u + 0.5) as i32;
        y = (v + 0.5) as i32;
        x = CMID(x, 0, self.tex_width - 1);
        y = CMID(y, 0, self.tex_height - 1);
        return self.texture[y as usize][x as usize];
    }
    //????????????
    pub fn draw_scanline(&mut self, mut scanline: Scanline) {
        let mut x = scanline.x;
        let y = scanline.y as usize;
        let mut w = scanline.w;

        let render_state = self.render_state;
        while w > 0 {
            if x >= 0 && x < HEIGHT as i32 {
                let rhw = scanline.v.rhw;
                let w1 = 1. / rhw;

                if rhw >= self.zbuffer[y as usize][x as usize] as f32 {
                    self.zbuffer[y as usize][x as usize] = rhw;
                    if render_state & RENDER_STATE_COLOR > 0 {
                        let R = scanline.v.color.r * w1;
                        let G = scanline.v.color.g * w1;
                        let B = scanline.v.color.b * w1;
                        let mut r = (R * 255. as f32) as i32;
                        let mut g = (G * 255. as f32) as i32;
                        let mut b = (B * 255. as f32) as i32;
                        r = CMID(r, 0, 255);
                        g= CMID(g, 0, 255);
                        b = CMID(b, 0, 255);
                        self.framebuf[y * WIDTH + x as usize] = ((r << 16) | (g << 8) | (b)) as u32;
                    }
                    if render_state & RENDER_STATE_TEXTURE > 0 {
                        let u = scanline.v.tc.u * w1;
                        let v = scanline.v.tc.v * w1;
                        self.framebuf[y * WIDTH + x as usize] = self.texture_read(u, v);
                    }
                }
            }
            scanline.v.add(scanline.step);
            if x >= HEIGHT as i32 {
                break;
            }
            w -= 1;
            x += 1;
        }
    }

    pub fn render_trap(&mut self, trap: &mut Trapezoid) {
        let mut scanline = Scanline {
            v: Vertex {
                pos: Vector4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
                tc: Texcoord { u: 0.0, v: 0.0 },
                color: Color { r: 0.0, g: 0.0, b: 0.0 },
                rhw: 0.0
            },
            step: Vertex {
                pos: Vector4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
                tc: Texcoord { u: 0.0, v: 0.0 },
                color: Color { r: 0.0, g: 0.0, b: 0.0 },
                rhw: 0.0
            }, x: 0, y: 0, w: 0
        };
        let top = (trap.top + 0.5) as i32;
        let bottom = (trap.bottom + 0.5) as i32;
        //println!("{} {}", top, bottom);
        for j in top..bottom {
            if j >= 0 && j < HEIGHT as i32 {
                trapezoid_edge_interp(trap, (j as f32 + 0.5));
                trapezoid_init_scan_line(trap, &mut scanline, j);
                self.draw_scanline(scanline);
            }
            if j >= HEIGHT as i32 {
                break;
            }
        }
    }

    pub fn draw_primitive(&mut self, v1: &mut Vertex, v2: &mut Vertex, v3: &mut Vertex) {
        let mut p1: Vector4f = Vector4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
        let mut p2: Vector4f = Vector4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
        let mut p3: Vector4f = Vector4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
        let mut c1: Vector4f = Vector4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
        let mut c2: Vector4f = Vector4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
        let mut c3: Vector4f = Vector4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };

        let render_state = self.render_state;
        self.transform.apply(&mut c1, v1.pos);
        self.transform.apply(&mut c2, v2.pos);
        self.transform.apply(&mut c3, v3.pos);
        if Transform::check_cvv(c1) != 0 {
            return;
        }
        if Transform::check_cvv(c2) != 0 {
            return;
        }
        if Transform::check_cvv(c3) != 0 {
            return;
        }
        self.transform.homogenize(&mut p1, c1);
        self.transform.homogenize(&mut p2, c2);
        self.transform.homogenize(&mut p3, c3);
        if (render_state & (RENDER_STATE_TEXTURE | RENDER_STATE_COLOR)) > 0 {
            let mut t1 = *v1;
            let mut t2 = *v2;
            let mut t3 = *v3;
            let traps: &mut [Trapezoid; 2] = &mut [trapezoid_init(); 2];
            t1.pos = p1.clone();
            t2.pos = p2.clone();
            t3.pos = p3.clone();
            t1.pos.w = c1.w;
            t2.pos.w = c2.w;
            t3.pos.w = c3.w;
            //println!("{} {} {} {}", v1.pos.x, v1.pos.y, v1.pos.z, v1.pos.w);
            t1.rhw_init();
            t2.rhw_init();
            t3.rhw_init();
            let mut n = 0;
            unsafe {
                n = trapezoid_init_triangle(traps, t1, t2, t3);
            }
            if n >= 1 {
                self.render_trap(&mut traps[0]);
            }
            if n >= 2 {
                self.render_trap(&mut traps[1]);
            }
        }
        if render_state & RENDER_STATE_WIREFRAME > 0 {
            self.draw_line(p1.x as usize, p1.y as usize,
                           p2.x as usize, p2.y as usize, self.foreground);
            self.draw_line(p1.x as usize, p1.y as usize,
                           p3.x as usize, p3.y as usize, self.foreground);
            self.draw_line(p3.x as usize, p3.y as usize,
                           p2.x as usize, p2.y as usize, self.foreground);
        }
    }

    pub fn draw_plane(&mut self, a: usize, b: usize, c: usize, d: usize) {
        let mut p1 = self.mesh[a as usize];
        let mut p2 = self.mesh[b as usize];
        let mut p3 = self.mesh[c as usize];
        let mut p4 = self.mesh[d as usize];
        p1.tc.u = 0.; p1.tc.v = 0.;
        p2.tc.u = 0.; p2.tc.v = 1.;
        p3.tc.u = 1.; p3.tc.v = 1.;
        p4.tc.u = 1.; p4.tc.v = 0.;
        self.draw_primitive(&mut p1,
                            &mut p2,
                            &mut p3,
        );
        self.draw_primitive(&mut p3,
                            &mut p4,
                            &mut p1,
        );
    }

    pub fn draw_box(&mut self, theta: f32) {
        let mut m = Matrix4f::new();
        m.set_rotation(-1., -0.5, 1., theta);
        self.transform.world = m;

        self.transform.update();

        self.draw_plane(0, 1, 2, 3);
        self.draw_plane(7, 6, 5, 4);
        self.draw_plane(0, 4, 5, 1);
        self.draw_plane(1, 5, 6, 2);
        self.draw_plane(2, 6, 7, 3);
        self.draw_plane(3, 7, 4, 0);
    }

    pub fn camera_at_zero(&mut self, x: f32, y: f32, z: f32) {
        let eye = Vector4f {
            x,
            y,
            z,
            w: 1.0
        };
        let at = Vector4f {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 1.,
        };
        let up = Vector4f {
            x: 0.,
            y: 0.,
            z: 1.,
            w: 1.,
        };
        self.transform.view.set_lookat(eye, at, up);
        self.transform.update();
    }

    pub fn init_texture(&mut self) {
        for j in 0..256 {
            for i in 0..256 {
                let x = i / 32;
                let y = j / 32;
                self.texture[j][i] = match (x + y) & 1 > 0 {
                    true => { 0xffffff }
                    false => { 0x3fbcef }
                };
            }
        }
        self.set_texture(256, 256);
    }
}