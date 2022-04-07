use crate::calc::interp;
use crate::vector_calc::Vector4f;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub struct Texcoord {
    pub u: f32,
    pub v: f32,
}

pub struct Vertex {
    pub pos: Vector4f, // Point
    pub tc: Texcoord,
    pub color: Color,
    pub rhw: f32,
}

pub struct Edge {
    pub v: Vertex,
    pub v1: Vertex,
    pub v2: Vertex,
}

pub struct Trapezoid {
    pub top: f32,
    pub bottom: f32,
    pub left: Edge,
    pub right: Edge,
}

pub struct Scanline {
    pub v: Vertex,
    pub step: Vertex,
    pub x: i32,
    pub y: i32,
    pub w: i32,
}

impl Vertex {
    pub fn rhw_init(&mut self) {
        let rhw = 1.0 / self.pos.w;
        self.rhw = rhw;
        self.tc.u *= rhw;
        self.tc.v *= rhw;
        self.color.r *= rhw;
        self.color.g *= rhw;
        self.color.b *= rhw;
    }

    pub fn interp(&mut self, x1: Vertex, x2: Vertex, t: f32) {
        self.pos.interp(x1.pos, x2.pos, t);
        self.tc.u = interp(x1.tc.u, x2.tc.u, t);
        self.tc.v = interp(x1.tc.v, x2.tc.v, t);
        self.color.r = interp(x1.color.r, x2.color.r, t);
        self.color.g = interp(x1.color.g, x2.color.g, t);
        self.color.b = interp(x1.color.b, x2.color.b, t);
        self.rhw = interp(x1.rhw, x2.rhw, t);
    }

    pub fn division(&mut self, x1: Vertex, x2: Vertex, w: f32) {
        let inv = 1.0 / w;
        self.pos.x = (x2.pos.x - x1.pos.x) * inv;
        self.pos.y = (x2.pos.y - x1.pos.y) * inv;
        self.pos.z = (x2.pos.z - x1.pos.z) * inv;
        self.pos.w = (x2.pos.w - x1.pos.w) * inv;
        self.tc.u = (x2.tc.u - x1.tc.u) * inv;
        self.tc.v = (x2.tc.v - x1.tc.v) * inv;
        self.color.r = (x2.color.r - x1.color.r) * inv;
        self.color.g = (x2.color.g - x1.color.g) * inv;
        self.color.b = (x2.color.b - x1.color.b) * inv;
        self.rhw = (x2.rhw - x1.rhw) * inv;
    }
}