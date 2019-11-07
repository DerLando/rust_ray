use super::super::geometry::{Ray, Vector3};
use super::{Transformation, Transformable};

/// Camera struct as an abstraction of a view into a scene
/// Cameras -by convention- have a different coordinate system then regular geometry
/// While we imagine the euclidean 3d space as an x-y-floor-plane with an upwards pointing z-axis,
/// cameras are aligned to their image plane as x-y-plane and have their z-axis pointing away from the image,
/// they are looking at
#[derive(Debug)]
pub struct Camera {
    origin: Vector3,
    direction: Vector3,
    up: Vector3,
    pub camera_to_world: Transformation
}

impl Camera {

    #[inline]
    pub const fn default_origin() -> Vector3 {
        Vector3{x: 0.0, y: 0.0, z: 0.0}
    }

    #[inline]
    pub const fn default_direction() -> Vector3 {
        Vector3{x: 0.0, y: 0.0, z: -1.0}
    }

    #[inline]
    pub const fn default_up() -> Vector3 {
        Vector3{x: 0.0, y: 0.0, z: 0.0}
    }

    pub const fn default() -> Camera {
        Camera {
            origin: Camera::default_origin(),
            direction: Camera::default_direction(),
            up: Camera::default_up(),
            camera_to_world: Transformation::identity()
        }
    }

    pub fn new_from_transformation(tranform: &Transformation) -> Camera {
        let mut camera = Camera::default();
        camera.camera_to_world = &camera.camera_to_world * tranform;
        camera
    }

    pub fn get_origin(&self) -> Vector3 {
        Camera::default_origin().transform(&self.camera_to_world)
    }

    pub fn get_direction(&self) -> Vector3 {
        Camera::default_direction().transform(&self.camera_to_world)
    }

    pub fn get_up(&self) -> Vector3 {
        Camera::default_up().transform(&self.camera_to_world)
    }
}

impl Transformable for Camera {
    fn transform(&self, transform: &Transformation) -> Self {
        // transformation logic implemented in mul operator overload
        Camera::new_from_transformation(&(&self.camera_to_world * transform))
    }
}