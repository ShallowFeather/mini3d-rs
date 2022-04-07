use crate::calc;

use std::num;

pub struct Vector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4f {
    pub fn length(&self) -> f32 {
        let sq = self.x * self.x + self.y * self.y + self.z * self.z;
        return f32::sqrt(sq);
    }

    pub fn add(&mut self, x: Vector4f, y: Vector4f) {
        self.x = x.x + y.x;
        self.y = x.y + y.y;
        self.z = x.z + y.z;
        self.w = 1.0;
    }

    pub fn sub(&mut self, x: Vector4f, y: Vector4f) {
        self.x = x.x - y.x;
        self.y = x.y - y.y;
        self.z = x.z - y.z;
        self.z = 1.0;
    }

    pub fn dotproduct(x: Vector4f, y: Vector4f) -> f32 {
        return x.x * y.x + x.y * y.y + x.z * y.z;
    }

    pub fn crossproduct(&mut self, x: Vector4f, y: Vector4f) {
        self.x = x.y * y.z - x.z * y.y;
        self.y = x.z * y.x - x.x * y.z;
        self.z = x.x * y.y - x.y * y.x;
        self.w = 1.0;
    }

    pub fn interp(&mut self, x1: Vector4f, x2: Vector4f, t: f32) {
        self.x = calc::interp(x1.x, x2.x, t);
        self.y = calc::interp(x1.y, x2.y, t);
        self.z = calc::interp(x1.z, x2.z, t);
        self.w = 1.0;
    }

    pub fn normalize(&mut self) {
        let len = self.vector_length();
        if len != 0.0 {
            let inv = 1.0 / len;
            self.x *= inv;
            self.y *= inv;
            self.z *= inv;
        }
    }
}