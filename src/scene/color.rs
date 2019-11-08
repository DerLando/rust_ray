use std::ops;

// TODO: This should rather be a material struct
#[derive(Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64
}

impl Color {

    pub const fn black() -> Color {
        Color {red: 0.0, green: 0.0, blue: 0.0}
    }

    pub const fn red() -> Color {
        Color {red: 1.0, green: 0.0, blue: 0.0}
    }

    pub const fn blue() -> Color {
        Color {red: 0.0, green: 0.0, blue: 1.0}
    }

    pub const fn green() -> Color {
        Color {red: 0.0, green: 1.0, blue: 0.0}
    }

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }
}


impl ops::Mul<&Color> for &Color {
    type Output = Color;

    fn mul(self, other: &Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl ops::Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}