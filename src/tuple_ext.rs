pub trait TupleExt {
    fn is_point(&self) -> bool;
    fn is_vector(&self) -> bool;
    fn point(x: f32, y: f32, z: f32) -> Self;
    fn vector(x: f32, y: f32, z: f32) -> Self;
}

impl TupleExt for (f32, f32, f32, f32) {
    fn is_point(&self) -> bool {
        self.3 == 1.0
    }
    fn is_vector(&self) -> bool {
        self.3 == 0.0
    }
    fn point(x: f32, y: f32, z: f32) -> Self {
        (x, y, z, 1.0)
    }
    fn vector(x: f32, y: f32, z: f32) -> Self {
        (x, y, z, 0.0)
    }
}