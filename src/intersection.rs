use crate::Sphere;

#[derive(Debug, Clone)]
pub enum Object {
    Sphere(Sphere),
}

#[derive(Debug, Clone)]
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
            (Self::Sphere(this_sphere), Self::Sphere(r0)) => this_sphere == r0,
        }
    }
}
