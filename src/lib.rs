#![deny(unsafe_code, warnings, clippy::all)]

pub mod consts;
mod convert;
mod float_lerp;
mod ops;

use float_lerp::Lerp;
use rand::{thread_rng, Rng};

pub trait Vector3Coordinate:
    num::Num
    + num::ToPrimitive
    + PartialOrd
    + std::fmt::Display
    + std::fmt::Debug
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::ops::DivAssign
    + Clone
    + Copy
{
}
impl Vector3Coordinate for f64 {}
impl Vector3Coordinate for f32 {}
impl Vector3Coordinate for i8 {}
impl Vector3Coordinate for i16 {}
impl Vector3Coordinate for i32 {}
impl Vector3Coordinate for i64 {}
impl Vector3Coordinate for i128 {}
impl Vector3Coordinate for u8 {}
impl Vector3Coordinate for u16 {}
impl Vector3Coordinate for u32 {}
impl Vector3Coordinate for u64 {}
impl Vector3Coordinate for u128 {}

/// Represents a vector in 3D space.
#[derive(Debug, PartialOrd, PartialEq, Default, Clone, Copy)]
pub struct Vector3<T: Vector3Coordinate> {
    x: T,
    y: T,
    z: T,
}

impl<T: Vector3Coordinate + num::Float> Vector3<T>
where
    rand::distributions::Standard: rand::prelude::Distribution<T>,
{
    /// Generates a random Vector3 with components in the range [0.0, 1.0).
    pub fn random() -> Self {
        Vector3 {
            x: thread_rng().gen(),
            y: thread_rng().gen(),
            z: thread_rng().gen(),
        }
    }

    /// Checks if this vector is approximately equal to another vector within a given epsilon.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec3_rs::Vector3;
    ///
    /// let v1 = Vector3::new(0.1, 0.2, 0.3);
    /// let v2 = Vector3::new(0.101, 0.199, 0.299);
    ///
    /// let epsilon = 0.01;
    /// let is_approx_equal = v1.fuzzy_equal(&v2, epsilon);
    /// println!("Are v1 and v2 approximately equal? {}", is_approx_equal);
    /// ```
    pub fn fuzzy_equal(&self, target: &Self, epsilon: f64) -> bool {
        (self.x - target.x).abs().to_f64().unwrap() <= epsilon
            && (self.y - target.y).abs().to_f64().unwrap() <= epsilon
            && (self.z - target.z).abs().to_f64().unwrap() <= epsilon
    }

    /// Linearly interpolates between this vector and another vector by a given ratio.
    pub fn lerp(&self, target: &Self, alpha: T) -> Self {
        Vector3 {
            x: self.x.lerp(target.x, alpha),
            y: self.y.lerp(target.y, alpha),
            z: self.z.lerp(target.z, alpha),
        }
    }
}

impl Vector3<f64> {
    /// Scales the vector such that its magnitude becomes 1.
    pub fn normalize(&mut self) {
        *self /= self.magnitude();
    }
}

impl Vector3<f32> {
    /// Scales the vector such that its magnitude becomes 1.
    pub fn normalize(&mut self) {
        *self /= self.magnitude() as f32;
    }
}

impl<T: Vector3Coordinate> Vector3<T> {
    /// Creates a new Vector3 with the specified coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec3_rs::Vector3;
    ///
    /// let vector3 = Vector3::new(1.0, 2.0, 3.0);
    /// ```
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3 { x, y, z }
    }

    /// Computes the magnitude (length) of the vector.
    pub fn magnitude(&self) -> f64 {
        let mag2 = self.x * self.x + self.y * self.y + self.z * self.z;
        mag2.to_f64().unwrap().sqrt()
    }

    /// Computes the dot product between this vector and another vector.
    pub fn dot(&self, target: &Self) -> T {
        self.x * target.x + self.y * target.y + self.z * target.z
    }

    /// Computes the cross product between this vector and another vector.
    pub fn cross(&self, target: &Self) -> Self {
        Vector3 {
            x: self.y * target.z - self.z * target.y,
            y: self.z * target.x - self.x * target.z,
            z: self.x * target.y - self.y * target.x,
        }
    }

    /// Computes the component-wise maximum of this vector and another vector.
    pub fn max(&self, target: &Self) -> Self {
        let x = if self.x > target.x { self.x } else { target.x };
        let y = if self.y > target.y { self.y } else { target.y };
        let z = if self.z > target.z { self.z } else { target.z };
        Vector3 { x, y, z }
    }

    /// Computes the component-wise minimum of this vector and another vector.
    pub fn min(&self, target: &Self) -> Self {
        let x = if self.x < target.x { self.x } else { target.x };
        let y = if self.y < target.y { self.y } else { target.y };
        let z = if self.z < target.z { self.z } else { target.z };
        Vector3 { x, y, z }
    }

    /// Computes the angle in radians between this vector and another vector.
    pub fn angle(&self, target: &Self) -> f64 {
        let dot_product = self.dot(target).to_f64().unwrap();
        let magnitude_product = self.magnitude() * target.magnitude();
        (dot_product / magnitude_product).acos()
    }

    /// Computes the angle in degrees between this vector and another vector.
    pub fn angle_deg(&self, target: &Self) -> f64 {
        self.angle(target) * (180.0 / std::f64::consts::PI)
    }

    /// Retrieves the X component of the vector.
    pub fn get_x(&self) -> T {
        self.x
    }

    /// Retrieves the Y component of the vector.
    pub fn get_y(&self) -> T {
        self.y
    }

    /// Retrieves the Z component of the vector.
    pub fn get_z(&self) -> T {
        self.z
    }
}

impl<T: Vector3Coordinate> std::fmt::Display for Vector3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector3({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle() {
        let angle = 1.5707963267948966;
        let calc_angle = consts::X_AXIS.angle(&consts::Y_AXIS);
        assert_eq!(calc_angle, angle);
    }

    #[test]
    fn create() {
        let my_vec = Vector3::new(1.3, 0.0, -5.35501);
        assert_eq!(my_vec.get_x(), 1.3);
        assert_eq!(my_vec.get_y(), 0.0);
        assert_eq!(my_vec.get_z(), -5.35501);
    }

    #[test]
    fn sum() {
        let vec1 = Vector3::new(1.0, 2.0, 3.0);
        let vec2 = Vector3::new(5.0, 0.0, -1.0);
        assert_eq!(vec1 + vec2, Vector3::new(6.0, 2.0, 2.0));
    }

    #[test]
    fn normalization() {
        let mut test_vec: Vector3<f64> = Vector3::new(1.0, 2.3, 100.123);
        test_vec.normalize();
        assert_eq!(
            test_vec,
            Vector3::new(0.00998458316076644, 0.02296454126976281, 0.9996864198054183)
        );
        assert!((1.0 - test_vec.magnitude()).abs() < 0.00000001);
    }

    #[test]
    fn lerp() {
        let start = Vector3::new(0.0, 0.0, 0.0);
        let end = Vector3::new(1.0, 2.0, 3.0);
        let lerp_result = start.lerp(&end, 0.75);
        assert_eq!(lerp_result, Vector3::new(0.75, 1.5, 2.25));
    }

    #[test]
    fn dot_product() {
        let vec1 = Vector3::new(1.0, 2.0, 3.0);
        let vec2 = Vector3::new(5.0, 0.0, -1.0);
        let dot_result = vec1.dot(&vec2);
        assert_eq!(dot_result, 2.0);
    }

    #[test]
    fn cross_product() {
        let vec1 = Vector3::new(1.0, 0.0, 0.0);
        let vec2 = Vector3::new(0.0, 1.0, 0.0);
        let cross_result = vec1.cross(&vec2);
        assert_eq!(cross_result, Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn max_components() {
        let vec1 = Vector3::new(1.0, 5.0, 3.0);
        let vec2 = Vector3::new(3.0, 2.0, 4.0);
        let max_result = vec1.max(&vec2);
        assert_eq!(max_result, Vector3::new(3.0, 5.0, 4.0));
    }

    #[test]
    fn min_components() {
        let vec1 = Vector3::new(1.0, 5.0, 3.0);
        let vec2 = Vector3::new(3.0, 2.0, 4.0);
        let min_result = vec1.min(&vec2);
        assert_eq!(min_result, Vector3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn fuzzy_equality() {
        let vec1 = Vector3::new(1.0, 2.0, 3.0);
        let vec2 = Vector3::new(1.01, 1.99, 3.01);
        let epsilon = 0.02;
        let fuzzy_equal_result = vec1.fuzzy_equal(&vec2, epsilon);
        assert!(fuzzy_equal_result);
    }
    #[test]
    fn nan_dont_panic() {
        let mut vec1: Vector3<f64> = Vector3::default();
        vec1 /= std::f64::NAN;
    }
    #[test]
    fn readme_example() {
        let mut v1: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
        let mut v2: Vector3<f64> = Vector3::new(3.0, 1.0, 2.0);

        // Basic operations
        let sum = v1 + v2;
        let difference = v1 - v2;
        let dot_product = v1.dot(&v2);
        let cross_product = v1.cross(&v2);

        // Other methods
        let lerp_result = v1.lerp(&v2, 0.5);
        let angle = v1.angle(&v2);
        let fuzzy_equal = v1.fuzzy_equal(&v2, 0.001);

        println!("Sum: {sum}");
        println!("Difference: {difference}");
        println!("Dot product: {dot_product}");
        println!("Cross product: {cross_product}");
        println!("Lerp 50%: {lerp_result}");
        println!("Angle: {angle}");
        print!("Are they close enough?: {fuzzy_equal}");

        v1.normalize();
        v2.normalize();

        println!("v1 normalized: {v1}");
        println!("v2 normalized: {v2}");
    }
}
