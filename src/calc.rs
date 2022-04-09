use std::borrow::Borrow;
use crate::vertex::{Edge, Vertex};
use crate::vector_calc::Vector4f;

#[derive(Clone)]
pub struct Trapezoid {
    pub top: f32,
    pub bottom: f32,
    pub left: Edge,
    pub right: Edge,
}

#[derive(Clone)]
pub struct Scanline {
    pub v: Vertex,
    pub step: Vertex,
    pub x: i32,
    pub y: i32,
    pub w: i32,
}

#[derive(Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Clone)]
pub struct Texcoord {
    pub u: f32,
    pub v: f32,
}

pub fn CMID(x: i32, min: i32, max: i32) -> i32{
    if x < min {
        return min;
    }
    else {
        if x > max {
            return max;
        }
        else {
            return x;
        }
    }
}

pub fn interp(x1: f32, x2: f32, t: f32) -> f32 {
    return x1 + (x2 - x1) * t;
}

pub fn trapezoid_init_triangle(trap: &mut [Trapezoid; 2], mut p1: Vertex,
                               mut p2: Vertex, mut p3: Vertex) -> i32 {
    let mut p = Vertex {
        pos: Vector4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
        tc: Texcoord { u: 0.0, v: 0.0 },
        color: Color { r: 0.0, g: 0.0, b: 0.0 },
        rhw: 0.0
    };
    let mut k = 0.0;
    let mut x = 0.0;

    if p1.pos.y < p2.pos.y {
        p = p1;
        p1 = p2;
        p2 = p;
    }
    if p1.pos.y > p3.pos.y {
        p = p1;
        p1 = p3;
        p3 = p;
    }
    if p2.pos.y < p3.pos.y {
        p = p2;
        p2 = p3;
        p3 = p;
    }
    if p1.pos.y == p2.pos.y && p1.pos.y == p3.pos.y {
        return 0;
    }
    if p1.pos.x == p2.pos.x && p1.pos.x == p3.pos.x {
        return 0;
    }

    if p1.pos.y == p2.pos.y {
        if p1.pos.x >p2.pos.x {
            p = p1;
            p1 = p2;
            p2 = p;
        }
        trap[0].top = p1.pos.y;
        trap[0].bottom = p3.pos.y;
        trap[0].left.v1 = p1.clone();
        trap[0].left.v2 = p3.clone();
        trap[0].right.v1 = p2.clone();
        trap[0].right.v2 = p3.clone();
        if trap[0].top < trap[0].bottom {
            return 1;
        }
        else {
            return 0;
        }
    }

    if p2.pos.y == p3.pos.y {
        if p2.pos.x > p3.pos.x {
            p = p2;
            p2 = p3;
            p3 = p;
        }
        trap[0].top = p1.pos.y;
        trap[0].bottom = p3.pos.y;
        trap[0].left.v1 = p1.clone();
        trap[0].left.v2 = p2.clone();
        trap[0].right.v1 = p1.clone();
        trap[0].right.v2 = p3.clone();
        if trap[0].top < trap[0].bottom {
            return 1;
        }
        else {
            return 0;
        }
    }

    trap[0].top = p1.pos.y;
    trap[0].bottom = p2.pos.y;
    trap[1].top = p2.pos.y;
    trap[1].bottom = p3.pos.y;

    k = (p3.pos.y - p1.pos.y) / (p2.pos.y - p1.pos.y);
    x = p1.pos.x + (p2.pos.x - p1.pos.x) * k;

    if x <= p3.pos.x {
        trap[0].left.v1 = p1.clone();
        trap[0].left.v2 = p2.clone();
        trap[0].right.v1 = p1.clone();
        trap[0].right.v2 = p3.clone();
        trap[1].left.v1 = p2.clone();
        trap[1].left.v2 = p3.clone();
        trap[1].right.v1 = p1.clone();
        trap[1].right.v2 = p3.clone();
    }
    else {
        trap[0].left.v1 = p1.clone();
        trap[0].left.v2 = p3.clone();
        trap[0].right.v1 = p1.clone();
        trap[0].right.v2 = p2.clone();
        trap[1].left.v1 = p1.clone();
        trap[1].left.v2 = p3.clone();
        trap[1].right.v1 = p1.clone();
        trap[1].right.v2 = p3.clone();
    }
    return 2;
}

pub fn trapezoid_edge_interp(trap: &mut Trapezoid, y: f32) {
    let s1 = trap.left.v2.pos.y - trap.left.v1.pos.y;
    let s2 = trap.right.v2.pos.y - trap.right.v1.pos.y;
    let t1 = (y - trap.left.v1.pos.y) / s1;
    let t2 = (y - trap.right.v1.pos.y) / s2;
    trap.left.v.interp(trap.left.v1.clone(), trap.left.v2.clone(), t1);
    trap.right.v.interp(trap.right.v1.clone(), trap.right.v2.clone(), t2);
}

pub fn trapezoid_init_scan_line(trap: Trapezoid, scanline: &mut Scanline, y: i32) {
    let width = trap.right.v.pos.x - trap.left.v.pos.x;
    scanline.x = (trap.left.v.pos.x + 0.5) as i32;
    scanline.w = (trap.right.v.pos.x + 0.5) as i32 - scanline.x;
    scanline.y = y;
    scanline.v = trap.left.v.clone();
    if trap.left.v.pos.x >= trap.right.v.pos.x {
        scanline.w = 0;
    }
    scanline.step.division(trap.left.v, trap.right.v, width)
}

pub fn map(val: f64, start1: f64, end1: f64, start2: f64, end2: f64) -> f64 {
    start2 + (end2 - start2) * ((val - start1) / (end1 - start1))
}

pub fn swap<'a>(x1: &'a mut usize, x2: &'a mut usize) {
    let x = *x1;
    *x1 = *x2;
    *x2 = x;
}