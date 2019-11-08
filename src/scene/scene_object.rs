use super::{Material, Color};
use super::super::traits::{RayCast};

pub struct SceneObject {
    pub object: Box<dyn RayCast>,
    pub material: Material
}

impl SceneObject {
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