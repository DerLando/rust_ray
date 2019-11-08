use super::super::geometry::{Vector3, Ray};
use super::super::traits::{RayCast, RayIntersectionResult, RayIntersection};
use super::super::scene::{Color};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Sphere {
    pub origin: Vector3,
    pub radius: f64,
    pub color: Color
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            origin: Vector3::new(),
            radius: 0.0,
            color: Color::red()
        }
    }

    pub fn new_from_values(origin: &Vector3, radius: f64) -> Sphere {
        Sphere {
            origin: *origin,
            radius: radius,
            color: Color::red()
        }
    }

    pub fn normal_at(&self, vector: &Vector3) -> Vector3 {
        (vector - &self.origin).as_normalized()
    }
}

impl RayCast for Sphere {
    fn intersect_ray(&self, ray: &Ray) -> RayIntersectionResult {
        let origin_projected = ray.project_vec_onto_self(&self.origin);
        let distance_to_ray = origin_projected.distance_to(&self.origin);

        println!("origin_projected is: {:?}, distance to ray is {}", origin_projected, distance_to_ray);

        match distance_to_ray.partial_cmp(&self.radius).expect("No NAN please :/") {
            Ordering::Greater => RayIntersectionResult::None,
            Ordering::Equal =>{
                let ray_int = RayIntersection::new_from_point_and_normal(&origin_projected, &self.normal_at(&origin_projected));
                RayIntersectionResult::Some(ray_int)
            },
            Ordering::Less => {
                let len_ray_origin_to_intersection = origin_projected.distance_to(&ray.origin) - (self.radius * self.radius - distance_to_ray * distance_to_ray).sqrt();
                println!("len_ray_to_int is: {}", len_ray_origin_to_intersection);
                let int = ray.vec_at_length(len_ray_origin_to_intersection);
                let ray_int = RayIntersection::new_from_point_and_normal(&int, &self.normal_at(&int));
                RayIntersectionResult::Some(ray_int)
            }
        }
    }
}
