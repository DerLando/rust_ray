use super::Color;

#[derive(Debug)]
pub struct Material {
    pub color: Color,
    pub albedo: f64,
}

impl Material {
    pub const fn default() -> Material {
        Material {
            color: Color {
                red: 0.2,
                green: 0.2,
                blue: 0.2
            },
            albedo: 50.0
        }
    }
}