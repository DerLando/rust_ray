use std::ops;
use super::super::scene::{Camera, Transformation, Transformable};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {

    pub const fn new() -> Vector3 {
        Vector3{
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub const fn new_from_values(x: f64, y: f64, z: f64) -> Vector3 {
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

    pub fn as_normalized(&self) -> Vector3 {
        let length = self.calculate_length();
        if length == 0.0 {return Vector3::new()}
        Vector3::new_from_values(self.x / length, self.y / length, self.z / length)
    }

    pub fn project_onto(&self, other: &Vector3) -> Vector3 {
        let other_normalized = other.as_normalized();
        let dot = other_normalized.dot_product(&self);
        println!("dot is {:?}", dot);
        other_normalized * dot
    }

    pub fn distance_to(&self, other: &Vector3) -> f64 {
        (self - other).calculate_length()
    }
}

impl Transformable for Vector3 {
    fn transform(&self, transform: &Transformation) -> Self {
        // transformation logic implemented in mul operator overload
        self * transform
    }
}

impl ops::Mul<&Transformation> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: &Transformation) -> Vector3 {
        let x = self.x * rhs.matrix[0][0] + self.y * rhs.matrix[1][0] + self.z * rhs.matrix[2][0] + rhs.matrix[3][0];
        let y = self.x * rhs.matrix[0][1] + self.y * rhs.matrix[1][1] + self.z * rhs.matrix[2][1] + rhs.matrix[3][1];
        let z = self.x * rhs.matrix[0][2] + self.y * rhs.matrix[1][2] + self.z * rhs.matrix[2][2] + rhs.matrix[3][2];
        let w = self.x * rhs.matrix[0][3] + self.y * rhs.matrix[1][3] + self.z * rhs.matrix[2][3] + rhs.matrix[3][3];
        println!("vec transform: x: {}, y: {}, z: {}, w: {}", x, y, z, w);
        if w != 1.0 && w != 0.0 {
            Vector3::new_from_values(x / w, y / w, z / w)
        }
        else {
            Vector3::new_from_values(x, y, z)
        }
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vector3::new_from_values(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<f64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Vector3 {
        Vector3::new_from_values(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Add<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Vector3 {
        Vector3::new_from_values(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Vector3 {
        Vector3::new_from_values(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Neg for &Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3::new_from_values(-self.x, -self.y, -self.z)
    }
}