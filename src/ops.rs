#![deny(unsafe_code, warnings, clippy::all)]

use crate::Vector3;

/// Implementation of addition (`+`) operation between two `Vector3` instances.
impl std::ops::Add<Vector3> for Vector3 {
    type Output = Self;
    /// Adds two `Vector3` instances component-wise.
    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// Implementation of addition assignment (`+=`) operation for `Vector3`.
impl std::ops::AddAssign for Vector3 {
    /// Adds the components of another `Vector3` to this `Vector3` instance in-place.
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

/// Implementation of subtraction (`-`) operation between two `Vector3` instances.
impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Self;
    /// Subtracts the components of the second `Vector3` from the first `Vector3` component-wise.
    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// Implementation of subtraction assignment (`-=`) operation for `Vector3`.
impl std::ops::SubAssign for Vector3 {
    /// Subtracts the components of another `Vector3` from this `Vector3` instance in-place.
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

/// Implementation of multiplication (`*`) operation between a `Vector3` and a scalar value.
impl std::ops::Mul<f64> for Vector3 {
    type Output = Self;
    /// Multiplies each component of a `Vector3` by a scalar value.
    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

/// Implementation of multiplication (`*`) operation between two `Vector3` instances.
impl std::ops::Mul<Vector3> for Vector3 {
    type Output = Self;
    /// Multiplies two `Vector3` instances component-wise.
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

/// Implementation of multiplication assignment (`*=`) operation for `Vector3` with a scalar.
impl std::ops::MulAssign<f64> for Vector3 {
    /// Multiplies each component of this `Vector3` instance by a scalar value in-place.
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

/// Implementation of multiplication assignment (`*=`) operation between two `Vector3` instances.
impl std::ops::MulAssign<Vector3> for Vector3 {
    /// Multiplies each component of this `Vector3` instance by the corresponding component of another in-place.
    fn mul_assign(&mut self, rhs: Vector3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

/// Implementation of division (`/`) operation between a `Vector3` and a scalar value.
impl std::ops::Div<f64> for Vector3 {
    type Output = Option<Self>;
    /// Divides each component of a `Vector3` by a scalar value.
    /// Returns `None` if the divisor is NaN.
    fn div(self, rhs: f64) -> Self::Output {
        if rhs.is_nan() {
            return None;
        }
        Some(Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        })
    }
}

/// Implementation of division (`/`) operation between two `Vector3` instances.
impl std::ops::Div<Vector3> for Vector3 {
    type Output = Option<Self>;
    /// Divides each component of a `Vector3` by the corresponding component of another.
    /// Returns `None` if any component of the divisor is NaN.
    fn div(self, rhs: Vector3) -> Self::Output {
        let new_vector = Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        };

        if new_vector.x.is_nan() || new_vector.y.is_nan() || new_vector.z.is_nan() {
            return None;
        }

        Some(new_vector)
    }
}

/// Implementation of division assignment (`/=`) operation for `Vector3` with a scalar.
impl std::ops::DivAssign<f64> for Vector3 {
    /// Divides each component of this `Vector3` instance by a scalar value in-place.
    ///
    /// # Panics
    ///
    /// Panics if any resulting component becomes NaN.
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;

        if self.x.is_nan() || self.y.is_nan() || self.z.is_nan() {
            panic!("Division assignment resulted in NaN!")
        }
    }
}

/// Implementation of division assignment (`/=`) operation between two `Vector3` instances.
impl std::ops::DivAssign<Vector3> for Vector3 {
    /// Divides each component of this `Vector3` instance by the corresponding component of another in-place.
    ///
    /// # Panics
    ///
    /// Panics if any component of the divisor becomes NaN.
    fn div_assign(&mut self, rhs: Vector3) {
        if rhs.x.is_nan() || rhs.y.is_nan() || rhs.z.is_nan() {
            panic!("Cannot divide vector by a non-normal-vector!")
        }
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
