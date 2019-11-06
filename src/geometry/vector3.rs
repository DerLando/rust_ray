use std::ops;

#[derive(Debug)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3 {

    pub fn new() -> Vector3 {
        Vector3{
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn new_from_values(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3{
            x: x,
            y: y,
            z: z
        }
    }

    pub fn dot_product(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn calculate_length(&self) -> f64 {
        self.dot_product(&self).sqrt()
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, factor: f64) -> Self {
        Vector3::new_from_values(self.x * factor, self.y * factor, self.z * factor)
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Self;

    fn add(self, rhs: Vector3) -> Self {
        Vector3::new_from_values(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}