use super::super::scene::{Camera, Document};
use super::super::geometry::{Ray, Vector3};
use super::super::traits::{RayCast, RayIntersectionResult};
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
        let black = image::Rgba([0, 0, 0, 0]);
        let red = image::Rgba([0, 254, 0, 0]);

        let pixel_scale = self.pixel_scale();
        let aspect_ratio = self.aspect_ratio();
        let camera_origin = camera.get_origin();

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
                for object_reference in document.object_table.objects.iter() {
                    match object_reference.intersect_ray(&ray) {
                        RayIntersectionResult::None => {
                            image.put_pixel(x, y, black)
                        },
                        RayIntersectionResult::Some(_) => {
                            // omit intersection result for now
                            // in the final code we will have to only draw the int nearest to the camera!
                            image.put_pixel(x, y, red)
                        }
                    }
                }
            }
        }

        image
    }
}