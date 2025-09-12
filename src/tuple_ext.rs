pub trait TupleExt {
    fn is_point(&self) -> bool;
    fn is_vector(&self) -> bool;

}

impl TupleExt for (f32, f32, f32, f32) {
    fn is_point(&self) -> bool {
        self.3 == 1.0
    }
    fn is_vector(&self) -> bool {
        self.3 == 0.0
    }
}

pub fn point(x: f32, y: f32, z: f32) -> (f32, f32, f32, f32)
{
    (x, y, z, 1.0)
}

pub  fn vector(x: f32, y: f32, z: f32) -> (f32, f32, f32, f32)
{
    (x, y, z, 0.0)
}