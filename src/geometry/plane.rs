use super::{Vector3, Ray};
use super::super::traits::{RayCast, RayIntersectionResult};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Plane {
    pub origin: Vector3,
    pub x_axis: Vector3,
    pub y_axis: Vector3
}

impl Plane {
    pub const fn world_xy() -> Plane {
        Plane {
            origin: Vector3::new(),
            x_axis: Vector3::x_axis(),
            y_axis: Vector3::y_axis()
        }
    }

    pub fn normal(&self) -> Vector3 {
        Vector3::cross_product(&self.x_axis, &self.y_axis).as_normalized()
    }
}

impl RayCast for Plane {
    fn intersect_ray(&self, ray: &Ray) -> RayIntersectionResult {
        // find to for ray:P = O + t*D so that P lies on the plane
        // t = (-D - (N dot O))/(N dot R) with N being the normal of the plane
        let normal = self.normal();
        let neg_d = normal.dot_product(&self.origin);
        let t = (neg_d - (normal.dot_product(&ray.origin))) / normal.dot_product(&ray.direction);

        match t.partial_cmp(&0.0_f64).expect("No NAN please :/") {
            Ordering::Less => RayIntersectionResult::None, // intersection point lies behind ray origin
            _ => RayIntersectionResult::Some(ray.vec_at_length(t))
        }
    }
}
