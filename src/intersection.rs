use std::cmp::Ordering;

use crate::Object;

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

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl Eq for Intersection {}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.total_cmp(&other.t)
    }
}
