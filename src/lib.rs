//! This crate provides a simple implementation of 3D vectors in Rust. Supports any numeric type trough num-traits.

#![cfg_attr(not(feature = "std"), no_std)]

mod consts;
mod convert;
#[cfg(any(feature = "std", feature = "libm"))]
mod float_lerp;
mod ops;
mod ops_scalar;

#[cfg(any(feature = "std", feature = "libm"))]
use float_lerp::Lerp;
#[cfg(any(feature = "std", feature = "libm"))]
use num_traits::clamp;
#[cfg(feature = "random")]
use rand::{
    RngExt,
    distr::uniform::{SampleRange, SampleUniform},
    make_rng,
    rngs::SmallRng,
};

#[cfg(feature = "random")]
thread_local! {
    static RNG: std::cell::RefCell<SmallRng> = std::cell::RefCell::new(make_rng());
}

/// Trait representing accepted coordonate kind `T` for `Vector3<T>`.
pub trait Vector3Coordinate:
    num_traits::Num
    + num_traits::ToPrimitive
    + PartialOrd
    + core::fmt::Display
    + core::ops::AddAssign
    + core::ops::SubAssign
    + core::ops::MulAssign
    + core::ops::DivAssign
    + Clone
{
}

impl<T> Vector3Coordinate for T where
    T: num_traits::Num
        + num_traits::ToPrimitive
        + PartialOrd
        + core::fmt::Display
        + core::ops::AddAssign
        + core::ops::SubAssign
        + core::ops::MulAssign
        + core::ops::DivAssign
        + Clone
{
}

/// Represents a vector in 3D space.
#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vector3<T: Vector3Coordinate> {
    x: T,
    y: T,
    z: T,
}

#[cfg(any(feature = "std", feature = "libm"))]
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
    ///
    /// # Examples
    ///
    /// ```
    /// type Vector3 = vec3_rs::Vector3<f64>;
    ///
    /// let origin = Vector3::zero();
    /// let x_axis = Vector3::x_axis();
    /// assert_eq!(origin.lerp(&x_axis, 0.5), Vector3::new(0.5, 0, 0))
    ///
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// type Vector3 = vec3_rs::Vector3<f64>;
    ///
    /// let mut vector3 = Vector3::new(12354,7324,-7765).normalized();
    /// assert_eq!(vector3.magnitude(), 1.0);
    /// ```
    #[must_use]
    #[inline]
    pub fn magnitude(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Computes the angle in **radians** (0..2pi) between this vector and another vector.
    #[must_use]
    #[inline]
    pub fn angle(&self, target: &Self) -> T {
        let dot_product = self.dot(target);
        let magnitude_product = self.magnitude() * target.magnitude();
        (dot_product / magnitude_product).acos()
    }

    /// Computes the angle in **degrees** (0.0..360.0) between this vector and another vector.
    #[must_use]
    #[inline]
    pub fn angle_deg(&self, target: &Self) -> T
    where
        T: From<f64>,
    {
        const COEFF: f64 = 180.0 / core::f64::consts::PI;
        self.angle(target) * From::from(COEFF)
    }

    /// Scales the vector such that its magnitude becomes 1.
    #[inline]
    pub fn normalize(&mut self) {
        *self /= self.magnitude();
    }

    /// Copies the vector and scales it such that its magnitude becomes 1.
    #[must_use]
    #[inline]
    pub fn normalized(&self) -> Self {
        *self / self.magnitude()
    }

    /// Computes the distance between this vector and another vector.
    ///
    /// # Examples
    ///
    /// ```
    /// type Vector3 = vec3_rs::Vector3<f64>;
    ///
    /// let left = -Vector3::x_axis();
    /// let right = Vector3::x_axis();
    /// assert_eq!(left.distance(&right), 2.0);
    #[must_use]
    #[inline]
    pub fn distance(&self, target: &Self) -> T {
        (*self - *target).magnitude()
    }

    /// Projects this vector onto another vector.
    #[must_use]
    #[inline]
    pub fn project(&self, on_normal: &Self) -> Self {
        *on_normal * (self.dot(on_normal) / on_normal.dot(on_normal))
    }

    /// Reflects this vector off a surface defined by a normal.
    #[must_use]
    #[inline]
    pub fn reflect(&self, normal: &Self) -> Self {
        let two = T::one() + T::one();
        *self - (*normal * (self.dot(normal) * two))
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
    pub fn rotated(&self, axis: &Self, angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        let axis_normalized = axis.normalized();

        let term1 = *self * cos;
        let term2 = axis_normalized.cross(self) * sin;
        let term3_scalar = axis_normalized.dot(self) * (T::one() - cos);
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
    #[inline]
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

    /// Generates a random Vector3 with components in the given ranges.
    #[cfg(feature = "random")]
    #[must_use]
    #[inline]
    pub fn random_range(
        range_x: impl SampleRange<T>,
        range_y: impl SampleRange<T>,
        range_z: impl SampleRange<T>,
    ) -> Self
    where
        rand::distr::StandardUniform: rand::prelude::Distribution<T>,
        T: SampleUniform,
    {
        RNG.with_borrow_mut(|thread| Self {
            x: thread.random_range(range_x),
            y: thread.random_range(range_y),
            z: thread.random_range(range_z),
        })
    }
}

impl<T: Vector3Coordinate> Vector3<T> {
    /// Creates a new Vector3 with the specified coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// type Vector3 = vec3_rs::Vector3<f64>;
    ///
    /// let vector3 = Vector3::new(1.0, 2.0, 3.0);
    /// let also_ok = Vector3::new(1, 2, 3);
    /// assert_eq!(vector3, also_ok);
    ///
    /// // can also be created from arrays and tuples
    /// // but no automatic number conversion
    /// let from_array = Vector3::from([1.0, 2.0, 3.0]);
    /// let from_tuple = Vector3::from((1.0, 2.0, 3.0));
    /// assert_eq!(vector3, from_array);
    /// assert_eq!(vector3, from_tuple);
    ///
    /// // conversion is fallible with unknown sized data types
    /// let slice: &[f64] = [1.0, 2.0, 3.0].as_ref();
    /// let from_slice = Vector3::try_from(slice).unwrap();
    /// let from_vec = Vector3::try_from(vec![1.0, 2.0, 3.0]).unwrap();
    /// assert_eq!(vector3, from_slice);
    /// assert_eq!(vector3, from_vec);
    ///
    /// ```
    pub fn new<U: Into<T>, V: Into<T>, W: Into<T>>(x: U, y: V, z: W) -> Self {
        let (x, y, z) = (x.into(), y.into(), z.into());
        Self { x, y, z }
    }

    /// Computes the dot product between this vector and another vector.
    /// <https://en.wikipedia.org/wiki/Dot_product>
    #[must_use]
    #[inline]
    pub fn dot(&self, target: &Self) -> T {
        let (x, y, z): (T, T, T) = target.clone().into();
        self.x.clone() * x + self.y.clone() * y + self.z.clone() * z
    }

    /// Computes the cross product between this vector and another vector.
    /// <https://en.wikipedia.org/wiki/Cross_product>
    #[must_use]
    #[inline]
    pub fn cross(&self, target: &Self) -> Self {
        let (x, y, z): (T, T, T) = target.clone().into();
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

impl<T: Vector3Coordinate> core::fmt::Display for Vector3<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Vector3({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ops::Sub;

    #[test]
    fn angle() {
        let angle = core::f64::consts::PI / 2.0;
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
        let vec1: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
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
        let vec1: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
        let vec2 = Vector3::new(5.0, 0.0, -1.0);
        let dot_result = vec1.dot(&vec2);
        assert!((dot_result - 2.0f64).abs() <= f64::EPSILON);
    }

    #[test]
    fn cross_product() {
        let vec1: Vector3<f64> = Vector3::new(1.0, 0.0, 0.0);
        let vec2 = Vector3::new(0.0, 1.0, 0.0);
        let cross_result = vec1.cross(&vec2);
        assert_eq!(cross_result, Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn max_components() {
        let vec1: Vector3<f64> = Vector3::new(1.0, 5.0, 3.0);
        let vec2 = Vector3::new(3.0, 2.0, 4.0);
        let max_result = vec1.max(&vec2);
        assert_eq!(max_result, Vector3::new(3.0, 5.0, 4.0));
    }

    #[test]
    fn min_components() {
        let vec1: Vector3<f64> = Vector3::new(1.0, 5.0, 3.0);
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
        let v1: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 6.0, 8.0);
        assert!((v1.distance(&v2) - (50.0f64).sqrt()).abs() <= f64::EPSILON);
    }

    #[test]
    fn project() {
        let v: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
        let on_normal = Vector3::new(1.0, 0.0, 0.0);
        let expected = Vector3::new(1.0, 0.0, 0.0);
        assert_eq!(v.project(&on_normal), expected);
    }

    #[test]
    fn reflect() {
        let v: Vector3<f64> = Vector3::new(1.0, -1.0, 0.0);
        let normal = Vector3::new(0.0, 1.0, 0.0);
        let expected = Vector3::new(1.0, 1.0, 0.0);
        assert_eq!(v.reflect(&normal), expected);
    }

    #[test]
    fn inverse() {
        let v: Vector3<f64> = Vector3::new(2.0, 4.0, 8.0);
        let expected = Vector3::new(0.5, 0.25, 0.125);
        assert_eq!(v.inverse(), expected);
    }

    #[test]
    fn abs() {
        let v: Vector3<f64> = Vector3::new(-1.0, -2.0, 3.0);
        let expected = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.abs(), expected);
    }

    #[test]
    fn ceil() {
        let v: Vector3<f64> = Vector3::new(1.1, 2.9, 3.0);
        let expected = Vector3::new(2.0, 3.0, 3.0);
        assert_eq!(v.ceil(), expected);
    }

    #[test]
    fn floor() {
        let v: Vector3<f64> = Vector3::new(1.1, 2.9, 3.0);
        let expected = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.floor(), expected);
    }

    #[test]
    fn round() {
        let v: Vector3<f64> = Vector3::new(1.1, 2.9, 3.5);
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
        let angle = core::f64::consts::FRAC_PI_2;
        let rotated = v.rotated(&axis, angle);
        let expected = Vector3::new(0.0, 1.0, 0.0);
        assert!(rotated.fuzzy_equal(&expected, 1e-15));
    }

    #[test]
    fn from_spherical() {
        let radius = 1.0;
        let polar = core::f64::consts::FRAC_PI_2;
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
        type Vector3 = super::Vector3<f64>;

        let v1 = Vector3::new(1, 2.5, 3);
        let v2 = Vector3::from([9.0, 1.0, 4.0]);

        let v3 = (v1 + v2) - (v1 - v2);
        let v3 = v3.cross(&v2);
        let v3 = v3 * v3.dot(&v1);
        let v3 = v3 * 10.0 / 3.2;
        let v3 = v3.normalized();
        let v3 = v3.lerp(&Vector3::zero(), 0.25);
        let v3 = v3.floor();

        println!("{v3}");
        println!("{}", v3.angle(&Vector3::z_axis()));
        println!("{}", v3.fuzzy_equal(&Vector3::z_axis(), 2.0));
    }
    #[test]
    fn conversion_box() {
        let correct = Vector3::new(1, 2, 3);
        let x = String::from("Vector3(1,2,3)").into_boxed_str();
        assert_eq!(x.parse::<Vector3<i32>>().unwrap(), correct);
    }
    #[test]
    fn from_slice() {
        let correct = Vector3::new(1, 2, 3);
        let arr = [1, 2, 3].as_ref();
        let x = Vector3::try_from(arr).unwrap();
        assert_eq!(correct, x);
    }

    #[test]
    fn from_box_slice() {
        let correct = Vector3::new(1, 2, 3);
        let arr: Box<[i32]> = Box::from([1, 2, 3].as_ref());
        let x = Vector3::try_from(arr).unwrap();
        assert_eq!(correct, x);
    }
}
