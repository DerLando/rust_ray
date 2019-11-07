use super::super::scene::{Camera, Transformable};
use super::super::geometry::{Sphere, Vector3};
use super::super::traits::{RayCast};
use std::boxed::Box;

pub struct ObjectTable {
    pub objects: Vec<Box<dyn RayCast>>
}

// TODO: Think about wrapping document-objects in an enum, for more type-safety
impl ObjectTable {

    pub fn new() -> ObjectTable {
        ObjectTable {
            objects: Vec::new()
        }
    }

    pub fn push_object(&mut self, object: Box<RayCast>) {
        self.objects.push(object)
    }
}

pub struct CameraTable {
    pub cameras: Vec<Camera>
}

impl CameraTable {

    pub fn new() -> CameraTable {
        CameraTable {
            cameras: Vec::new()
        }
    }

    pub fn push_camera(&mut self, camera: Camera) {
        self.cameras.push(camera)
    }
}

pub struct Document {
    pub object_table: ObjectTable,
    pub camera_table: CameraTable
}

impl Document {
    pub fn default_test() -> Document {
        let sphere = Sphere::new_from_values(&Vector3::new_from_values(0.0, 0.0, -5.0), 1.0);
        let mut object_table = ObjectTable::new();
        object_table.push_object(Box::new(sphere));

        // let sphere1 = Sphere::new_from_values(&Vector3::new_from_values(0.0, 0.0, 0.0), 1.0);
        // object_table.push_object(Box::new(sphere1));

        let camera = Camera::default();
        let mut camera_table = CameraTable::new();
        camera_table.push_camera(camera);

        Document{
            object_table: object_table,
            camera_table: camera_table
        }
    }
}