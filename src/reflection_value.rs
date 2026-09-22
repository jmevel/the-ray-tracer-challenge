use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ReflectionValue {
    value: f32,
}

impl ReflectionValue {
    pub fn new(value: f32) -> ReflectionValue {
        if value < 0.0 || value > 1.0 {
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

impl Deref for ReflectionValue {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
