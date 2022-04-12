use std::f32::consts::PI;
use std::num;
use crate::vector_calc::Vector4f;

#[derive(Clone, Copy)]
pub struct Matrix4f {
    pub m: [[f32; 4]; 4],
}

impl Matrix4f {
    pub fn new() -> Matrix4f {
        Matrix4f {
            m:  [[0., 0., 0., 0.],
                [0., 0., 0., 0.],
                [0., 0., 0., 0.],
                [0., 0., 0., 0.]],
        }
    }

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
        let mut z: Matrix4f = Matrix4f {
            m: [[0., 0., 0., 0.],
                [0., 0., 0., 0.],
                [0., 0., 0., 0.],
                [0., 0., 0., 0.]]
        };
        for i in 0..4 {
            for j in 0..4 {
                z.m[j][i] = (x.m[j][0] * y.m[0][i])
                    + (x.m[j][1] * y.m[1][i])
                    + (x.m[j][2] * y.m[2][i])
                    + (x.m[j][3] * y.m[3][i]);
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
        self.m[0][0] = 1.; self.m[1][1] = 1.; self.m[2][2] = 1.; self.m[3][3] = 1.;
        self.m[0][1] = 0.; self.m[0][2] = 0.; self.m[0][3] = 0.;
        self.m[1][0] = 0.; self.m[1][2] = 0.; self.m[1][3] = 0.;
        self.m[2][0] = 0.; self.m[2][1] = 0.; self.m[2][3] = 0.;
        self.m[3][0] = 0.; self.m[3][1] = 0.; self.m[3][2] = 0.;
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
        let mut vec = Vector4f{ x, y, z, w: 1.0 };
        let w = qcos;
        vec.normalize();
        x = vec.x * qsin;
        y = vec.y * qsin;
        z = vec.z * qsin;
        self.m[0][0] = 1. - 2. * y * y - 2. * z * z;
        self.m[1][0] = 2. * x * y - 2. * w * z;
        self.m[2][0] = 2. * x * z + 2. * w * y;
        self.m[0][1] = 2. * x * y + 2. * w * z;
        self.m[1][1] = 1. - 2. * x * x - 2. * z * z;
        self.m[2][1] = 2. * y * z - 2. * w * x;
        self.m[0][2] = 2. * x * z - 2. * w * y;
        self.m[1][2] = 2. * y * z + 2. * w * x;
        self.m[2][2] = 1. - 2. * x * x - 2. * y * y;
        self.m[0][3] = 0.0;
        self.m[1][3] = 0.0;
        self.m[2][3] = 0.0;
        self.m[3][0] = 0.0;
        self.m[3][1] = 0.0;
        self.m[3][2] = 0.0;
        self.m[3][3] = 1.0;
    }

    //Camera
    pub fn set_lookat(&mut self, eye: Vector4f, at: Vector4f, up: Vector4f) {
        let mut xaxis = Vector4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
        let mut yaxis = Vector4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
        let mut zaxis = Vector4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };

        zaxis.sub(at, eye);

        zaxis.normalize();

        xaxis.crossproduct(up, zaxis);
        //println!("{} {} {} {}", xaxis.x, xaxis.y, xaxis.z, xaxis.w);
        xaxis.normalize();
        yaxis.crossproduct(zaxis, xaxis);

        self.m[0][0] = xaxis.x;
        self.m[1][0] = xaxis.y;
        self.m[2][0] = xaxis.z;
        self.m[3][0] = -xaxis.dotproduct(eye);

        self.m[0][1] = yaxis.x;
        self.m[1][1] = yaxis.y;
        self.m[2][1] = yaxis.z;
        self.m[3][1] = -yaxis.dotproduct(eye);

        self.m[0][2] = zaxis.x;
        self.m[1][2] = zaxis.y;
        self.m[2][2] = zaxis.z;
        self.m[3][2] = -zaxis.dotproduct(eye);

        self.m[0][3] = 0.0;
        self.m[1][3] = 0.0;
        self.m[2][3] = 0.0;
        self.m[3][3] = 1.0;
    }

    pub fn set_perspective(&mut self, eye_fov: f32, aspect_ratio: f32, zNear: f32, zFar: f32) {
        let fax = 1.0 / (eye_fov * 0.5).tan();
        self.set_zero();
        self.m[0][0] = (fax / aspect_ratio);
        self.m[1][1] = fax;
        self.m[2][2] = zFar / (zFar - zNear);
        self.m[3][2] = -zNear * zFar / (zFar - zNear);
        self.m[2][3] = 1.0;
    }
}