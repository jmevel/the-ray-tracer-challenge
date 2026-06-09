pub fn float_equals(first: &f32, second: &f32) -> bool {
    const EPSILON: f32 = 0.0001;
    f32::abs(first - second) < EPSILON
}
