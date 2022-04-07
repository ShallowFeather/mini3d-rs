use std::f32::consts::PI;
use crate::matrix_calc::Matrix4f;
use crate::vector_calc::Vector4f;

#[derive(Clone, Copy)]
pub struct Transform {
    world: Matrix4f,
    view: Matrix4f,
    projection: Matrix4f,
    transform: Matrix4f,
    w: f32,
    h: f32,
}

impl Transform {
    pub fn update(&mut self) {
        let mut m: Matrix4f = Matrix4f {
            m: [[0., 0., 0., 0.],
                [0., 0., 0., 0.],
                [0., 0., 0., 0.],
                [0., 0., 0., 0.]]
        };
        m.mul(self.world.clone(), self.view.clone());
        self.transform.mul(m, self.projection.clone());
    }

    pub fn init(&mut self, width: i32, height: i32) {
        let aspect = width as f32 / height as f32;
        self.world.set_identity();
        self.view.set_identity();
        self.projection.set_perspective(PI * 0.5, aspect, 1.0, 500.0);
        self.w = width as f32;
        self.h = height as f32;
        self.update();
    }

    pub fn apply(&self, y: &mut Vector4f, x: Vector4f) {
        y.matrix_apply(x, self.transform);
    }

    pub fn check_cvv(v: Vector4f) -> i32 {
        let w = v.w;
        let mut check = 0;
        if v.z < 0.0 {
            check |= 1;
        }
        if v.z > w {
            check |= 2;
        }
        if v.x < -w {
            check |= 4;
        }
        if v.x > w {
            check |= 8;
        }
        if v.y < -w {
            check |= 16;
        }
        if v.y > w {
            check |= 32;
        }
        return check;
    }

    pub fn homogenize(&self, y: &mut Vector4f, x: Vector4f) {
        let rhw: f32 = 1.0 / x.w;
        y.x = (x.x * rhw + 1.0) * self.w * 0.5;
        y.y = (1.0 - x.y * rhw) * self.h * 0.5;
        y.z = x.z * rhw;
        y.w = 1.0;
    }
}