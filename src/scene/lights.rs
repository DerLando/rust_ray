use super::{Color};
use super::super::geometry::{Vector3};

pub enum Light {
    Directional(DirectionalLight),
    Point(PointLight)
}

pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f64
}

impl DirectionalLight {
    pub fn default() -> DirectionalLight {
        DirectionalLight {
            direction: Vector3::z_axis() * -1.0,
            color: Color{red: 0.9, green: 0.9, blue: 0.9},
            intensity: 0.9
        }
    }
}

pub struct PointLight {

}