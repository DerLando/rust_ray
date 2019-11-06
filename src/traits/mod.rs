use super::geometry::{Vector3, Ray};

pub enum RayIntersectionResult {
    None,
    Some(Vector3)
}

pub trait RayCast {
    fn intersect_ray(&self, ray: &Ray) -> RayIntersectionResult;
}