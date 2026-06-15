use std::ops::Deref;

use crate::Intersection;

#[derive(Debug, Clone)]
pub struct Intersections(pub Vec<Intersection>);

impl Intersections {
    pub fn hit(&self) -> Option<&Intersection> {
        let mut positive_elements = self
            .0
            .iter()
            .filter(|i| i.t().is_sign_positive())
            .peekable();

        if positive_elements.peek().is_none() {
            return None;
        }

        Some(
            positive_elements
                .min()
                .expect("Can't determine hit of intersections"),
        )
    }
}

impl Deref for Intersections {
    type Target = Vec<Intersection>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
