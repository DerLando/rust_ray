use super::super::scene::{Camera, Transformable, SceneObject, Material, Color, Light, DirectionalLight, PointLight};
use super::super::geometry::{Sphere, Vector3, Plane};
use super::super::traits::{RayCast};
use std::boxed::Box;

pub struct ObjectTable {
    pub objects: Vec<SceneObject>
}

impl ObjectTable {

    pub fn new() -> ObjectTable {
        ObjectTable {
            objects: Vec::new()
        }
    }

    pub fn push_object(&mut self, object: SceneObject) {
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

pub struct LightTable {
    lights: Vec<Light>
}

impl LightTable {

    pub fn new() -> LightTable {
        LightTable {
            lights: Vec::new()
        }
    }

    pub fn push_directional_light(&mut self, light: DirectionalLight) {
        self.lights.push(Light::Directional(light))
    }

    pub fn push_point_light(&mut self, light: PointLight) {
        self.lights.push(Light::Point(light))
    }

    pub fn push_light(&mut self, light: Light) {
        self.lights.push(light)
    }
}

pub struct Document {
    pub object_table: ObjectTable,
    pub camera_table: CameraTable,
    pub light_table: LightTable,
}

impl Document {
    pub fn default_test() -> Document {
        let sphere = Sphere::new_from_values(&Vector3::new_from_values(1.0, 5.0, 1.1), 1.0);
        let mut object_table = ObjectTable::new();
        object_table.push_object(SceneObject::new_from_object(Box::new(sphere)));

        let sphere1 = Sphere::new_from_values(&Vector3::new_from_values(-0.5, 2.5, 1.0), 0.2);
        let material = Material{color: Color::red()};
        object_table.push_object(SceneObject::new_with_material(Box::new(sphere1), material));

        let sphere2 = Sphere::new_from_values(&Vector3::new_from_values(0.0, 3.75, 0.8), 0.7);
        let material = Material{color: Color::blue()};
        object_table.push_object(SceneObject::new_with_material(Box::new(sphere2), material));

        let mut plane = Plane::world_xy();
        plane.origin = Vector3::new_from_values(0.0, 0.0, 0.0);
        let plane_material = Material{color: Color::green()};
        object_table.push_object(SceneObject::new_with_material(Box::new(plane), plane_material));

        let camera = Camera::new_from_position_and_target(&Vector3::new_from_values(0.0, 0.0, 0.6), &Vector3::new_from_values(0.0, 0.6, 0.3));
        let mut camera_table = CameraTable::new();
        camera_table.push_camera(camera);

        let dir_light = DirectionalLight::default();
        let mut light_table = LightTable::new();
        light_table.push_directional_light(dir_light);

        Document{
            object_table: object_table,
            camera_table: camera_table,
            light_table: light_table,
        }
    }
}