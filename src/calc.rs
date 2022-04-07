

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