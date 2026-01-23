use crate::{Vector3, Vector3Coordinate};

impl<T: Vector3Coordinate> Vector3<T> {
    /// Unit vector along the X-axis.
    #[must_use]
    #[inline]
    pub fn x_axis() -> Self {
        Self {
            x: T::one(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    /// Unit vector along the Y-axis.
    #[must_use]
    #[inline]
    pub fn y_axis() -> Self {
        Self {
            x: T::zero(),
            y: T::one(),
            z: T::zero(),
        }
    }

    /// Unit vector along the Z-axis.
    #[must_use]
    #[inline]
    pub fn z_axis() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::one(),
        }
    }

    /// Vector with all components set to 1.
    #[must_use]
    #[inline]
    pub fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one(),
            z: T::one(),
        }
    }

    /// Vector with all components set to 0.
    #[must_use]
    #[inline]
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
}
