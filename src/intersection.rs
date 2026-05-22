use crate::Sphere;

#[derive(Debug)]
pub enum Object {
    Sphere(Sphere),
}

#[derive(Debug)]
pub struct Intersection {
    t: f32,
    object: Object,
}

impl Intersection {
    pub fn new(t: f32, object: Object) -> Self {
        Self { t, object }
    }

    pub fn t(&self) -> f32 {
        self.t
    }
    pub fn object(&self) -> &Object {
        &self.object
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Sphere(l0), Self::Sphere(r0)) => l0 == r0,
        }
    }
}
