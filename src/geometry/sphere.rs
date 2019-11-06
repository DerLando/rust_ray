use super::super::geometry::{Vector3};
use super::super::traits::{RayCast, RayIntersectionResult};

#[derive(Debug)]
pub struct Sphere {
    origin: Vector3,
    radius: f64
}

// impl RayCast for Sphere {
//     fn intersect_ray(&self) -> RayIntersectionResult {
        
//     }
// }
