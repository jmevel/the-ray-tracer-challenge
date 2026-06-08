#[derive(Debug)]
pub struct ReflectionValue {
    value: f32,
}

impl ReflectionValue {
    pub fn new(value: f32) -> ReflectionValue {
        if value < 0f32 || value > 1f32 {
            panic!("{value} must be between 0 and 1");
        }

        Self { value }
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}

impl PartialEq<f32> for ReflectionValue {
    fn eq(&self, other: &f32) -> bool {
        &self.value == other
    }
}
