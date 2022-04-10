use std::f32::consts::PI;
use crate::matrix_calc::Matrix4f;
use crate::vector_calc::Vector4f;
use crate::{HEIGHT, WIDTH};

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
        m.mul(self.world, self.view);
        self.transform.mul(m, self.projection);
    }

    pub fn init() -> Transform {
        let aspect = WIDTH as f32 / HEIGHT as f32;
        let mut ret = Transform {
            world: Matrix4f::new(),
            view: Matrix4f::new(),
            projection: Matrix4f::new(),
            transform: Matrix4f::new(),
            w: WIDTH as f32,
            h: HEIGHT as f32,
       };
        ret.world.set_identity();
        ret.view.set_identity();
        //println!("{} {} {} {}", ret.view.m[0][3], ret.view.m[1][3], ret.view.m[2][3], ret.view.m[3][3]);
        ret.projection.set_perspective(3.1415926 * 0.5, aspect, 1.0, 500.0);
        //println!("{} {} {} {}", ret.projection.m[0][3], ret.projection.m[1][3], ret.projection.m[2][3], ret.projection.m[3][3]);
        ret.update();
        //println!("{} {} {} {}", ret.transform.m[0][3], ret.transform.m[1][3], ret.transform.m[2][3], ret.transform.m[3][3]);
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
        return check;
    }

    pub fn homogenize(&self, y: &mut Vector4f, x: Vector4f) {
        let rhw: f32 = 1.0 / x.w;
        y.x = (x.x * rhw + 1.0) * self.w * 0.5;
        y.y = (1.0 - x.y * rhw) * self.h * 0.5;
        y.z = x.z * rhw;
        y.w = 1.0;
        //println!("{} {} {} {}", y.x, y.y, y.z, y.w);
    }
}