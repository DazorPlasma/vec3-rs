#![deny(unsafe_code, warnings, clippy::all)]

mod convert;
mod float_lerp;
mod ops;
mod consts;

use float_lerp::Lerp;
use rand::{thread_rng, Rng};

/// Represents a vector in 3D space.
/// 
/// Doesn't allow for NaN coordinates.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    /// Creates a new Vector3 with the specified coordinates.
    ///
    /// # Panics
    ///
    /// Panics if any of the coordinates is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec3_rs::Vector3;
    ///
    /// let v = Vector3::new(1.0, 2.0, 3.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        if x.is_nan() || y.is_nan() || z.is_nan() {
            panic!("Vector3 cannot have NaN coordinates!");
        }
        Vector3 { x, y, z }
    }

    // Constructors for creating Vector3 instances from different types

    pub fn from_i32(x: i32, y: i32, z: i32) -> Self {
        Vector3 {x: x as f64, y:  y as f64, z: z as f64}
    }

    pub fn from_u32(x: u32, y: u32, z: u32) -> Self {
        Vector3 {x: x as f64, y:  y as f64, z: z as f64}
    }

    pub fn from_i64(x: i64, y: i64, z: i64) -> Self {
        Vector3 {x: x as f64, y:  y as f64, z: z as f64}
    }

    pub fn from_u64(x: u64, y: u64, z: u64) -> Self {
        Vector3 {x: x as f64, y:  y as f64, z: z as f64}
    }

    /// Generates a random Vector3 with components in the range [0.0, 1.0).
    pub fn random() -> Self {
        Vector3 {
            x: thread_rng().gen(),
            y: thread_rng().gen(),
            z: thread_rng().gen(),
        }
    }

    /// Scales the vector such that its magnitude becomes 1.
    pub fn normalize(&mut self) {
        *self /= self.magnitude();
    }

    /// Computes the magnitude (length) of the vector.
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Linearly interpolates between this vector and another vector by a given ratio.
    pub fn lerp(&self, target: &Self, alpha: f64) -> Self {
        Vector3 {
            x: self.x.lerp(target.x, alpha),
            y: self.y.lerp(target.y, alpha),
            z: self.z.lerp(target.z, alpha),
        }
    }

    /// Computes the dot product between this vector and another vector.
    pub fn dot(&self, target: &Self) -> f64 {
        self.x * target.x + self.y * target.y + self.z * target.z
    }

    /// Computes the cross product between this vector and another vector.
    pub fn cross(&self, target: &Self) -> Vector3 {
        Vector3 {
            x: self.y * target.z - self.z * target.y,
            y: self.z * target.x - self.x * target.z,
            z: self.x * target.y - self.y * target.x,
        }
    }

    /// Computes the component-wise maximum of this vector and another vector.
    pub fn max(&self, target: &Self) -> Self {
        Vector3 {
            x: self.x.max(target.x),
            y: self.y.max(target.y),
            z: self.z.max(target.z),
        }
    }

    /// Computes the component-wise minimum of this vector and another vector.
    pub fn min(&self, target: &Self) -> Self {
        Vector3 {
            x: self.x.min(target.x),
            y: self.y.min(target.y),
            z: self.z.min(target.z),
        }
    }

    /// Computes the angle in radians between this vector and another vector.
    pub fn angle(&self, target: &Self) -> f64 {
        let dot_product = self.dot(target);
        let magnitude_product = self.magnitude() * target.magnitude();
        (dot_product / magnitude_product).acos()
    }

    /// Computes the angle in degrees between this vector and another vector.
    pub fn angle_deg(&self, target: &Self) -> f64 {
        self.angle(target) * (180.0 / std::f64::consts::PI)
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
        (self.x - target.x).abs() <= epsilon
            && (self.y - target.y).abs() <= epsilon
            && (self.z - target.z).abs() <= epsilon
    }

    /// Retrieves the X component of the vector.
    pub fn get_x(&self) -> f64 {
        self.x
    }

    /// Retrieves the Y component of the vector.
    pub fn get_y(&self) -> f64 {
        self.y
    }

    /// Retrieves the Z component of the vector.
    pub fn get_z(&self) -> f64 {
        self.z
    }
}

impl std::fmt::Display for Vector3 {
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
        let mut test_vec = Vector3::new(1.0, 2.3, 100.123);
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
}