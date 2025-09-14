pub trait TupleExt {
    fn is_point(&self) -> bool;
    fn is_vector(&self) -> bool;
    fn equals(&self, other: &(f32, f32, f32, f32)) -> bool;
    fn add(&self, other: &(f32, f32, f32, f32)) -> (f32, f32, f32, f32);
    fn subtract(&self, other: &(f32, f32, f32, f32)) -> (f32, f32, f32, f32);
    fn negate(&self) -> (f32, f32, f32, f32);
}

impl TupleExt for (f32, f32, f32, f32) {
    fn is_point(&self) -> bool {
        self.3 == 1.0
    }
    fn is_vector(&self) -> bool {
        self.3 == 0.0
    }
    fn equals(&self, other: &(f32, f32, f32, f32)) -> bool {
        float_equals(self.0, other.0)
            && float_equals(self.1, other.1)
            && float_equals(self.2, other.2)
            && float_equals(self.3, other.3)
    }
    fn add(&self, other: &(f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
        (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
    fn subtract(&self, other: &(f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
        (
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
        )
    }

    fn negate(&self) -> (f32, f32, f32, f32) {
        (-self.0, -self.1, -self.2, -self.3)
    }
}

pub fn point(x: f32, y: f32, z: f32) -> (f32, f32, f32, f32) {
    (x, y, z, 1.0)
}

pub fn vector(x: f32, y: f32, z: f32) -> (f32, f32, f32, f32) {
    (x, y, z, 0.0)
}

fn float_equals(first: f32, second: f32) -> bool {
    const EPSILON: f32 = 0.00001;
    f32::abs(first - second) < EPSILON
}
