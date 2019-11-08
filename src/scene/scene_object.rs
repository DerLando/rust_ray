use super::{Material, Color};
use super::super::traits::{RayCast};
use super::super::geometry::{Plane};

pub struct SceneObject {
    pub object: Box<dyn RayCast>,
    pub material: Material
}

impl SceneObject {

    pub fn new() -> SceneObject {
        SceneObject {
            object: Box::new(Plane::world_xy()),
            material: Material::default()
        }
    }

    pub fn new_from_object(object: Box<RayCast>) -> SceneObject {
        SceneObject {
            object: object,
            material: Material::default()
        }
    }

    pub fn new_with_material(object: Box<RayCast>, material: Material) -> SceneObject {
        SceneObject {
            object: object,
            material: material
        }
    }
}