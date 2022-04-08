use crate::calc;

use std::num;
use crate::matrix_calc::Matrix4f;

#[derive(Clone, Copy)]
pub struct Vector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4f {
    pub fn new() -> Vector4f {
        Vector4f {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0
        }
    }

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

    pub fn dotproduct(&self, y: Vector4f) -> f32 {
        return self.x * y.x + self.y * y.y + self.z * y.z;
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
        let len = self.length();
        if len != 0.0 {
            let inv = 1.0 / len;
            self.x *= inv;
            self.y *= inv;
            self.z *= inv;
        }
    }

    pub fn matrix_apply(&mut self, x: Vector4f, m: Matrix4f) {
        let mut X = x.x;
        let mut Y = x.y;
        let mut Z = x.z;
        let mut W = x.w;
        self.x = X * m.m[0][0] + Y * m.m[1][0] + Z * m.m[2][0] + W * m.m[3][0];
        self.y = Y * m.m[0][1] + Y * m.m[1][1] + Z * m.m[2][1] + W * m.m[3][1];
        self.z = Z * m.m[0][2] + Y * m.m[1][2] + Z * m.m[2][2] + W * m.m[3][2];
        self.w = W * m.m[0][3] + Y * m.m[1][3] + Z * m.m[2][3] + W * m.m[3][3];
    }

}