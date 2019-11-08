
// TODO: This should rather be a material struct
#[derive(Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64
}

impl Color {
    pub const fn red() -> Color {
        Color {red: 1.0, green: 0.0, blue: 0.0}
    }

    pub const fn blue() -> Color {
        Color {red: 0.0, green: 0.0, blue: 1.0}
    }

    pub const fn green() -> Color {
        Color {red: 0.0, green: 1.0, blue: 0.0}
    }
}