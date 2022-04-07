use std::intrinsics::sinf32;
use std::num;
use crate::vector_calc::Vector4f;

pub struct Matrix4f {
    pub m: [[f32; 4]; 4],
}

impl Matrix4f {
    pub fn add(&mut self, x: Matrix4f, y: Matrix4f) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] = x.m[i][j] + y.m[i][j];
            }
        }
    }

    pub fn sub(&mut self, x: Matrix4f, y: Matrix4f) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] = x.m[i][j] - y.m[i][j];
            }
        }
    }

    pub fn mul(&mut self, x: Matrix4f, y: Matrix4f) {
        let mut z: Matrix4f = Matrix4f { m: [[f32; 4]; 4] };
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    z.m[i][j] += x.m[i][k] * y.m[k][j];
                }
            }
        }
        self.m = z.m;
    }

    pub fn scale(&mut self, x: Matrix4f, f: f32) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] = x.m[i][j] * f;
            }
        }
    }

    pub fn set_identity(&mut self) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] = 0.0;
            }
        }
        self.m[0][0] = 1.0;
        self.m[1][1] = 1.0;
        self.m[2][2] = 1.0;
        self.m[3][3] = 1.0;
    }

    pub fn set_zero(&mut self) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] = 0.0;
            }
        }
    }

    pub fn set_translate(&mut self, x: f32, y: f32, z: f32) {
        self.set_identity();
        self.m[3][0] = x;
        self.m[3][1] = y;
        self.m[3][2] = z;
    }

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.set_identity();
        self.m[0][0] = x;
        self.m[1][1] = y;
        self.m[2][2] = z;
    }

    pub fn set_rotation(&mut self, mut x: f32, mut y: f32, mut z: f32, dolta: f32) {
        let qsin = (dolta * 0.5).sin();
        let qcos = (dolta * 0.5).cos();
        let vec = Vector4f{ x, y, z, w: 1.0 };
        x = vec.x * qsin;
        y = vec.y * qsin;
        z = vec.z * qsin;
        self.m[0][0] = 1 - 2 * y * y - 2 * z * z;
        self.m[1][0] = 2 * x * y - 2 * w * z;
        self.m[2][0] = 2 * x * z + 2 * w * y;
        self.m[0][1] = 2 * x * y + 2 * w * z;
        self.m[1][1] = 1 - 2 * x * x - 2 * z * z;
        self.m[2][1] = 2 * y * z - 2 * w * x;
        self.m[0][2] = 2 * x * z - 2 * w * y;
        self.m[1][2] = 2 * y * z + 2 * w * x;
        self.m[2][2] = 1 - 2 * x * x - 2 * y * y;
        self.m[0][3] = 0.0;
        self.m[1][3] = 0.0;
        self.m[2][3] = 0.0;
        self.m[3][0] = 0.0;;
        self.m[3][1] = 0.0;
        self.m[3][2] = 0.0;
        self.m[3][3] = 0.0;
    }

    //Camera
    pub fn set_lookat(&mut self, eye: Vector4f, at: Vector4f, up: Vector4f) {

    }
}