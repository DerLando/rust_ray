use super::super::geometry::{Vector3};

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
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
            direction: direction.as_normalized()
        }
    }

    pub fn vec_at_length(&self, t: f64) -> Vector3 {
        &self.origin + &(self.direction * t)
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

    pub fn project_vec_onto_self(&self, other: &Vector3) -> Vector3 {
        let u = other - &self.origin;
        println!("u is {:?}", u);
        if u.dot_product(&self.direction) < 0.0 {
            self.origin
        }
        else {
            &self.origin + &u.project_onto(&self.direction)
        }
    }
}