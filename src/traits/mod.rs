use super::geometry::{Vector3};

pub enum RayIntersectionResult {
    None,
    Some(Vector3)
}

pub trait RayCast {
    fn intersect_ray(&self) -> RayIntersectionResult;
}