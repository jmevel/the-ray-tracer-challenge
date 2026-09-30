pub const EPSILON: f32 = 0.00001;
pub fn float_equals(first: &f32, second: &f32) -> bool {
    f32::abs(first - second) < EPSILON
}
