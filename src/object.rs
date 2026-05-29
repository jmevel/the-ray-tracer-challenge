use crate::Sphere;

#[derive(Debug, Clone)]
pub enum Object {
    Sphere(Sphere),
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Sphere(this_sphere), Self::Sphere(r0)) => this_sphere == r0,
        }
    }
}
