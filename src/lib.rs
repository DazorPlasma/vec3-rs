#![deny(unsafe_code)]
#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]

pub mod consts;
mod convert;
mod float_lerp;
mod ops;

use float_lerp::Lerp;
use rand::Rng;

pub trait Vector3Coordinate:
    num::Num
    + num::ToPrimitive
    + PartialOrd
    + std::fmt::Display
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::ops::DivAssign
    + Clone
{
}

impl<T> Vector3Coordinate for T where
    T: num::Num
        + num::ToPrimitive
        + PartialOrd
        + std::fmt::Display
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::ops::MulAssign
        + std::ops::DivAssign
        + Clone
{
}

/// Represents a vector in 3D space.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Vector3<T: Vector3Coordinate> {
    x: T,
    y: T,
    z: T,
}

impl<T: Vector3Coordinate + num::Float> Vector3<T>
where
    rand::distr::StandardUniform: rand::prelude::Distribution<T>,
{
    /// Generates a random Vector3 with components in the range [0.0, 1.0).
    #[must_use]
    pub fn random() -> Self {
        let mut thread = rand::rng();
        Self {
            x: thread.random(),
            y: thread.random(),
            z: thread.random(),
        }
    }
}

impl<T: Vector3Coordinate + num::Float> Vector3<T> {
    /// Checks if this vector is approximately equal to another vector within a given epsilon.
    ///
    /// # Panics
    ///
    /// Panics if `epsilon` is negative.
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
    #[must_use]
    #[inline]
    pub fn fuzzy_equal(&self, target: &Self, epsilon: T) -> bool {
        assert!(epsilon.is_sign_positive());
        // unrolled for performance
        (self.x - target.x).abs() <= epsilon
            && (self.y - target.y).abs() <= epsilon
            && (self.z - target.z).abs() <= epsilon
    }

    /// Linearly interpolates between this vector and another vector by a given ratio.
    #[must_use]
    #[inline]
    pub fn lerp(&self, target: &Self, alpha: T) -> Self {
        Self {
            x: self.x.lerp(target.x, alpha),
            y: self.y.lerp(target.y, alpha),
            z: self.z.lerp(target.z, alpha),
        }
    }

    /// Computes the magnitude (length) of the vector.
    #[must_use]
    #[inline]
    pub fn magnitude(&self) -> T {
        let mag2 = self.x * self.x + self.y * self.y + self.z * self.z;
        mag2.sqrt()
    }

    /// Computes the angle in radians between this vector and another vector.
    #[must_use]
    #[inline]
    pub fn angle(&self, target: &Self) -> T {
        let dot_product = self.dot(target);
        let magnitude_product = self.magnitude() * target.magnitude();
        (dot_product / magnitude_product).acos()
    }

    /// Computes the angle in degrees between this vector and another vector.
    ///
    /// # Panics
    ///
    /// Panics if `T`, the vector's datatype, cannot be converted to a f64.
    #[must_use]
    #[inline]
    pub fn angle_deg(&self, target: &Self) -> T {
        const COEFF: f64 = 180.0 / std::f64::consts::PI;
        self.angle(target) * T::from(COEFF).unwrap()
    }

    /// Scales the vector such that its magnitude becomes 1.
    #[inline]
    pub fn normalize(&mut self) {
        *self /= self.magnitude();
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
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Computes the dot product between this vector and another vector.
    #[must_use]
    #[inline]
    pub fn dot(&self, target: &Self) -> T {
        self.x.clone() * target.x.clone()
            + self.y.clone() * target.y.clone()
            + self.z.clone() * target.z.clone()
    }

    /// Computes the cross product between this vector and another vector.
    #[must_use]
    #[inline]
    pub fn cross(&self, target: &Self) -> Self {
        Self {
            x: self.y.clone() * target.z.clone() - self.z.clone() * target.y.clone(),
            y: self.z.clone() * target.x.clone() - self.x.clone() * target.z.clone(),
            z: self.x.clone() * target.y.clone() - self.y.clone() * target.x.clone(),
        }
    }

    /// Computes the component-wise maximum of this vector and another vector.
    #[must_use]
    #[inline]
    pub fn max(&self, target: &Self) -> Self {
        let x = if self.x > target.x {
            self.x.clone()
        } else {
            target.x.clone()
        };
        let y = if self.y > target.y {
            self.y.clone()
        } else {
            target.y.clone()
        };
        let z = if self.z > target.z {
            self.z.clone()
        } else {
            target.z.clone()
        };
        Self { x, y, z }
    }

    /// Computes the component-wise minimum of this vector and another vector.
    #[must_use]
    #[inline]
    pub fn min(&self, target: &Self) -> Self {
        let x = if self.x < target.x {
            self.x.clone()
        } else {
            target.x.clone()
        };
        let y = if self.y < target.y {
            self.y.clone()
        } else {
            target.y.clone()
        };
        let z = if self.z < target.z {
            self.z.clone()
        } else {
            target.z.clone()
        };
        Self { x, y, z }
    }

    /// Retrieves the X component of the vector.
    pub const fn x(&self) -> &T {
        &self.x
    }

    /// Retrieves the Y component of the vector.
    pub const fn y(&self) -> &T {
        &self.y
    }

    /// Retrieves the Z component of the vector.
    pub const fn z(&self) -> &T {
        &self.z
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
    use std::ops::Sub;

    #[test]
    fn angle() {
        let angle = std::f64::consts::PI / 2.0;
        let calc_angle = Vector3::<f64>::x_axis().angle(&Vector3::<f64>::y_axis());
        assert!(calc_angle.sub(angle) <= f64::EPSILON);
    }

    #[test]
    fn create() {
        let my_vec: Vector3<f64> = Vector3::new(1.3, 0.0, -5.35501);
        assert!((my_vec.x() - 1.3f64).abs() <= f64::EPSILON);
        assert!((my_vec.y() - 0.0f64).abs() <= f64::EPSILON);
        assert!((my_vec.z() - -5.35501f64).abs() <= f64::EPSILON);
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
            Vector3::new(
                0.009_984_583_160_766_44,
                0.022_964_541_269_762_81,
                0.999_686_419_805_418_3
            )
        );
        assert!((1.0 - test_vec.magnitude()).abs() <= f64::EPSILON);
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
        assert!((dot_result - 2.0f64).abs() <= f64::EPSILON);
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
        vec1 /= f64::NAN;
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
