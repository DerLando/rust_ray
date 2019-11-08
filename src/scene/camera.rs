use super::super::geometry::{Ray, Vector3};
use super::{Transformation, Transformable};

/// Camera struct as an abstraction of a view into a scene
/// Cameras -by convention- have a different coordinate system then regular geometry
/// While we imagine the euclidean 3d space as an x-y-floor-plane with an upwards pointing z-axis,
/// cameras are aligned to their image plane as x-y-plane and have their z-axis pointing away from the image,
/// they are looking at
#[derive(Debug, PartialEq)]
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

    #[inline]
    pub const fn default_arbitrary() -> Vector3 {
        Vector3{x: 0.0, y: 1.0, z: 0.0}
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

    pub fn new_from_position_and_target(position: &Vector3, target: &Vector3) -> Camera {
        // compute looking direction, up direction, and right direction from input vectors
        let v_direction = (position - target).as_normalized();
        let v_right = Vector3::cross_product(&Camera::default_arbitrary().as_normalized(), &v_direction);
        let v_up = Vector3::cross_product(&v_direction, &v_right);

        // set up default camera to change transform on
        let mut camera = Camera::default();

        // right vector goes in first 3 fields of first row
        camera.camera_to_world.matrix[0][0] = v_right.x;
        camera.camera_to_world.matrix[0][1] = v_right.y;
        camera.camera_to_world.matrix[0][2] = v_right.z;

        // up vector goes in first 3 fields of second row
        camera.camera_to_world.matrix[1][0] = v_up.x;
        camera.camera_to_world.matrix[1][1] = v_up.y;
        camera.camera_to_world.matrix[1][2] = v_up.z;

        // direction vector goes in first 3 fields of third row
        camera.camera_to_world.matrix[2][0] = v_direction.x;
        camera.camera_to_world.matrix[2][1] = v_direction.y;
        camera.camera_to_world.matrix[2][2] = v_direction.z;

        // position vec goes in the first 3 fields of the last row
        camera.camera_to_world.matrix[3][0] = position.x;
        camera.camera_to_world.matrix[3][1] = position.y;
        camera.camera_to_world.matrix[3][2] = position.z;

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