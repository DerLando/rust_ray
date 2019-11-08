use super::super::scene::{Camera, Document, SceneObject, Color};
use super::super::geometry::{Ray, Vector3};
use super::super::traits::{RayCast, RayIntersectionResult, RayIntersection};
use image::{DynamicImage, Rgba, GenericImage};

#[derive(Debug)]
pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub field_of_view: f64,
}

impl Renderer {

    pub const fn new_from_values(width: u32, height: u32, fov: f64) -> Renderer {
        Renderer {
            width: width,
            height: height,
            field_of_view: fov
        }
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }

    pub fn pixel_scale(&self) -> f64 {
        (self.field_of_view * 0.5).to_radians().tan()
    }

    pub fn render(&self, document: &Document, camera: &Camera) -> DynamicImage {

        // Lets do this real slow
        let mut image = DynamicImage::new_rgb8(self.width, self.height);
        let black = image::Rgba([135, 206, 250, 0]);

        let pixel_scale = self.pixel_scale();
        let aspect_ratio = self.aspect_ratio();
        let camera_origin = camera.get_origin();

        let direction_to_light = Vector3::new_from_values(-0.1, 0.0, -1.0);

        let mut closest_scene_object: &SceneObject = &SceneObject::new();

        for x in 0..self.width {
            for y in 0..self.height {
                // hard to explain, look at this for now:
                // https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays
                let dir_x = (2.0 * (x as f64 + 0.5) / self.width as f64 - 1.0) * aspect_ratio * pixel_scale;
                let dir_y = (1.0 - 2.0 * (y as f64 + 0.5) / self.height as f64) * pixel_scale;
                let direction = Vector3::new_from_values(dir_x, dir_y, -1.0);

                // transform direction vector from camera to world space
                let direction = &direction * &camera.camera_to_world;
                let direction = direction.as_normalized();

                // cast ray from camera origin in pixel direction
                let ray = Ray::new_from_vectors(&camera_origin, &direction);

                // iterate over all document objects and intersect
                let mut distance_to_camera = 1000000.0_f64;
                let mut object_found = false;
                let mut color = Color::black();
                for object_reference in document.object_table.objects.iter() {
                    match object_reference.object.intersect_ray(&ray) {
                        RayIntersectionResult::None => (),
                        RayIntersectionResult::Some(int) => {
                            // test if closest to camera
                            let distance = int.point.distance_to_squared(&camera_origin);
                            if distance < distance_to_camera {
                                distance_to_camera = distance;
                                closest_scene_object = object_reference;
                                object_found = true;

                                let intensity = document.light_table.get_intensity(0);
                                let light_power = (int.normal.dot_product(&(direction_to_light * -1.0))).max(0.0) * intensity;
                                let light_reflected = closest_scene_object.material.albedo / std::f64::consts::PI;
                                color = &(&(&closest_scene_object.material.color * document.light_table.get_color(0)) * light_power) * light_reflected;
                                color = color.clamp();
                            }
                        }
                    }
                }

                if object_found {
                    let color_rgba = image::Rgba([(color.red * 255.0) as u8, (color.green * 255.0) as u8, (color.blue * 255.0) as u8, 0]);
                    image.put_pixel(x, y, color_rgba)
                }
                else {
                    image.put_pixel(x, y, black);
                }

            }
        }

        image
    }
}