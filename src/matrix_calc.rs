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

    pub fn matrix_mul
}