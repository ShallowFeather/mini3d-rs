use std::f32::consts::PI;
use crate::matrix_calc::Matrix4f;
use crate::vector_calc::Vector4f;

#[derive(Clone, Copy)]
pub struct Transform {
    pub world: Matrix4f,
    pub view: Matrix4f,
    pub projection: Matrix4f,
    pub transform: Matrix4f,
    pub w: f32,
    pub h: f32,
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

    pub fn init(width: usize, height: usize) -> Transform {
        let aspect = width as f32 / height as f32;
        let mut ret = Transform {
            world: Matrix4f::new(),
            view: Matrix4f::new(),
            projection: Matrix4f::new(),
            transform: Matrix4f::new(),
            w: 0.0,
            h: 0.0
        };
        ret.world.set_identity();
        ret.view.set_identity();
        ret.projection.set_perspective(PI * 0.5, aspect, 1.0, 500.0);
        ret.w = width as f32;
        ret.h = height as f32;
        ret.update();
        return ret;
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
        if v.z >  w {
            check |= 2;
        }
        if v.x < -w {
            check |= 4;
        }
        if v.x >  w {
            check |= 8;
        }
        if v.y < -w {
            check |= 16;
        }
        if v.y > w {
            check |= 32;
        }
        println!("{}", check);
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