#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]

mod consts;
mod convert;
mod float_lerp;
mod ops;
mod ops_scalar;

use float_lerp::Lerp;
use num_traits::clamp;
use std::any::type_name;

#[cfg(feature = "random")]
use rand::{rngs::ThreadRng, Rng};

thread_local! {
    #[cfg(feature = "random")]
    static RNG: std::cell::RefCell<ThreadRng> = std::cell::RefCell::new(rand::rng());
}

pub trait Vector3Coordinate:
    num_traits::Num
    + num_traits::ToPrimitive
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
    T: num_traits::Num
        + num_traits::ToPrimitive
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
#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
pub struct Vector3<T: Vector3Coordinate> {
    x: T,
    y: T,
    z: T,
}

impl<T> PartialOrd for Vector3<T>
where
    T: Vector3Coordinate + num_traits::Float,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.magnitude().partial_cmp(&other.magnitude())
    }
}

impl<T: Vector3Coordinate + num_traits::Float> Vector3<T> {
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
    /// assert!(is_approx_equal)
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
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Computes the angle in radians between this vector and another vector.
    #[must_use]
    #[inline]
    pub fn angle(&self, target: Self) -> T {
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
    pub fn angle_deg(&self, target: Self) -> T {
        const COEFF: f64 = 180.0 / std::f64::consts::PI;
        self.angle(target)
            * T::from(COEFF).unwrap_or_else(|| {
                panic!("failed to express {COEFF:?} as type {}", type_name::<T>())
            })
    }

    /// Scales the vector such that its magnitude becomes 1.
    #[inline]
    pub fn normalize(&mut self) {
        *self /= self.magnitude();
    }

    /// Computes the distance between this vector and another vector.
    #[must_use]
    #[inline]
    pub fn distance(&self, target: Self) -> T {
        (*self - target).magnitude()
    }

    /// Projects this vector onto another vector.
    #[must_use]
    #[inline]
    pub fn project(&self, on_normal: Self) -> Self {
        on_normal * (self.dot(on_normal) / on_normal.dot(on_normal))
    }

    /// Reflects this vector off a surface defined by a normal.
    #[must_use]
    #[inline]
    pub fn reflect(&self, normal: Self) -> Self {
        let two = T::one() + T::one();
        *self - (normal * (self.dot(normal) * two))
    }

    /// Inverts the components of the vector.
    #[must_use]
    #[inline]
    pub fn inverse(&self) -> Self {
        let one = T::one();
        Self {
            x: one / self.x,
            y: one / self.y,
            z: one / self.z,
        }
    }

    /// Returns a new vector with the absolute value of each component.
    #[must_use]
    #[inline]
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    /// Returns a new vector with the ceiling of each component.
    #[must_use]
    #[inline]
    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }

    /// Returns a new vector with the floor of each component.
    #[must_use]
    #[inline]
    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }

    /// Returns a new vector with the rounded value of each component.
    #[must_use]
    #[inline]
    pub fn round(&self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
        }
    }

    /// Returns a new vector with each component clamped to a given range.
    #[must_use]
    #[inline]
    pub fn clamp(&self, min: T, max: T) -> Self {
        Self {
            x: clamp(self.x, min, max),
            y: clamp(self.y, min, max),
            z: clamp(self.z, min, max),
        }
    }

    /// Rotates the vector around an axis by a given angle in radians.
    #[must_use]
    #[inline]
    pub fn rotated(&self, axis: Self, angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        let axis_normalized = axis / axis.magnitude();

        let term1 = *self * cos;
        let term2 = axis_normalized.cross(*self) * sin;
        let term3_scalar = axis_normalized.dot(*self) * (T::one() - cos);
        let term3 = axis_normalized * term3_scalar;

        term1 + term2 + term3
    }

    /// Creates a new `Vector3` from spherical coordinates.
    ///
    /// # Arguments
    ///
    /// * `radius` - The distance from the origin.
    /// * `polar` - The polar angle (the angle from the z-axis, in radians).
    /// * `azimuth` - The azimuth angle (the angle from the x-axis in the xy-plane, in radians).
    #[must_use]
    #[inline]
    pub fn from_spherical(radius: T, polar: T, azimuth: T) -> Self {
        let (sin_polar, cos_polar) = polar.sin_cos();
        let (sin_azimuth, cos_azimuth) = azimuth.sin_cos();
        Self {
            x: radius * sin_polar * cos_azimuth,
            y: radius * sin_polar * sin_azimuth,
            z: radius * cos_polar,
        }
    }

    /// Generates a random Vector3 with components in the range [0.0, 1.0).
    #[cfg(feature = "random")]
    #[must_use]
    pub fn random() -> Self
    where
        rand::distr::StandardUniform: rand::prelude::Distribution<T>,
    {
        RNG.with_borrow_mut(|thread| Self {
            x: thread.random(),
            y: thread.random(),
            z: thread.random(),
        })
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
    pub fn dot(&self, target: Self) -> T {
        let (x, y, z) = target.into();
        self.x.clone() * x + self.y.clone() * y + self.z.clone() * z
    }

    /// Computes the cross product between this vector and another vector.
    #[must_use]
    #[inline]
    pub fn cross(&self, target: Self) -> Self {
        let (x, y, z) = target.into();
        Self {
            x: self.y.clone() * z.clone() - self.z.clone() * y.clone(),
            y: self.z.clone() * x.clone() - self.x.clone() * z,
            z: self.x.clone() * y - self.y.clone() * x,
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
        let calc_angle = Vector3::<f64>::x_axis().angle(Vector3::<f64>::y_axis());
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
        let dot_result = vec1.dot(vec2);
        assert!((dot_result - 2.0f64).abs() <= f64::EPSILON);
    }

    #[test]
    fn cross_product() {
        let vec1 = Vector3::new(1.0, 0.0, 0.0);
        let vec2 = Vector3::new(0.0, 1.0, 0.0);
        let cross_result = vec1.cross(vec2);
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
    fn distance() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 6.0, 8.0);
        assert!((v1.distance(v2) - (50.0f64).sqrt()).abs() <= f64::EPSILON);
    }

    #[test]
    fn project() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let on_normal = Vector3::new(1.0, 0.0, 0.0);
        let expected = Vector3::new(1.0, 0.0, 0.0);
        assert_eq!(v.project(on_normal), expected);
    }

    #[test]
    fn reflect() {
        let v = Vector3::new(1.0, -1.0, 0.0);
        let normal = Vector3::new(0.0, 1.0, 0.0);
        let expected = Vector3::new(1.0, 1.0, 0.0);
        assert_eq!(v.reflect(normal), expected);
    }

    #[test]
    fn inverse() {
        let v = Vector3::new(2.0, 4.0, 8.0);
        let expected = Vector3::new(0.5, 0.25, 0.125);
        assert_eq!(v.inverse(), expected);
    }

    #[test]
    fn abs() {
        let v = Vector3::new(-1.0, -2.0, 3.0);
        let expected = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.abs(), expected);
    }

    #[test]
    fn ceil() {
        let v = Vector3::new(1.1, 2.9, 3.0);
        let expected = Vector3::new(2.0, 3.0, 3.0);
        assert_eq!(v.ceil(), expected);
    }

    #[test]
    fn floor() {
        let v = Vector3::new(1.1, 2.9, 3.0);
        let expected = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.floor(), expected);
    }

    #[test]
    fn round() {
        let v = Vector3::new(1.1, 2.9, 3.5);
        let expected = Vector3::new(1.0, 3.0, 4.0);
        assert_eq!(v.round(), expected);
    }

    #[test]
    fn clamp() {
        let v = Vector3::new(0.0, 5.0, 10.0);
        let min = 1.0;
        let max = 9.0;
        let expected = Vector3::new(1.0, 5.0, 9.0);
        assert_eq!(v.clamp(min, max), expected);
    }

    #[test]
    fn rotated() {
        let v = Vector3::new(1.0, 0.0, 0.0);
        let axis = Vector3::new(0.0, 0.0, 1.0);
        let angle = std::f64::consts::FRAC_PI_2;
        let rotated = v.rotated(axis, angle);
        let expected = Vector3::new(0.0, 1.0, 0.0);
        assert!(rotated.fuzzy_equal(&expected, 1e-15));
    }

    #[test]
    fn from_spherical() {
        let radius = 1.0;
        let polar = std::f64::consts::FRAC_PI_2;
        let azimuth = 0.0;
        let v = Vector3::from_spherical(radius, polar, azimuth);
        let expected = Vector3::new(1.0, 0.0, 0.0);
        assert!(v.fuzzy_equal(&expected, 1e-15));
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
        let dot_product = v1.dot(v2);
        let cross_product = v1.cross(v2);

        // Other methods
        let lerp_result = v1.lerp(&v2, 0.5);
        let angle = v1.angle(v2);
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
