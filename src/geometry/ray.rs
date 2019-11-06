use super::super::geometry::{Vector3};

#[derive(Debug)]
pub struct Ray {
    origin: Vector3,
    direction: Vector3
}

impl Ray {

    pub fn new() -> Ray {
        Ray {
            origin: Vector3::new(),
            direction: Vector3::new()
        }
    }

    pub fn new_from_vectors(origin: &Vector3, direction: &Vector3) -> Ray {
        Ray {
            origin: *origin,
            direction: *direction
        }
    }

    pub fn distance_to(&self, other: &Vector3) -> f64 {
        let u = other - &self.origin;
        if u.dot_product(&self.direction) < 0.0 {
            (other - &self.origin).calculate_length()
        }
        else {
            let puv = u.project_onto(&self.direction);
            let other_projected = &self.origin + &puv;
            (&other_projected - &other).calculate_length()
        }
    }
}