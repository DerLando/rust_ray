use super::Color;

#[derive(Debug)]
pub struct Material {
    pub color: Color,
}

impl Material {
    pub const fn default() -> Material {
        Material {
            color: Color {
                red: 0.2,
                green: 0.2,
                blue: 0.2
            }
        }
    }
}