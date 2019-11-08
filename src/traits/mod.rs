use super::geometry::{Vector3, Ray};

pub struct RayIntersection {
    pub point: Vector3,
    pub normal: Vector3,
}

impl RayIntersection {
    pub fn new_from_point_and_normal(point: &Vector3, normal: &Vector3) -> RayIntersection {
        RayIntersection {
            point: *point,
            normal: *normal
        }
    }
}

pub enum RayIntersectionResult {
    None,
    Some(RayIntersection)
}

pub trait RayCast {
    fn intersect_ray(&self, ray: &Ray) -> RayIntersectionResult;
}