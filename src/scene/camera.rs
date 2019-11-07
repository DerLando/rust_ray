use super::super::geometry::{Ray, Vector3};

#[derive(Debug)]
pub struct Camera {
    pub origin: Vector3,
    pub direction: Vector3,
    pub up: Vector3,
}