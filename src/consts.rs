#![allow(unused)]

use crate::Vector3;

/// Predefined constant vector along the X-axis.
pub const X_AXIS: Vector3<f64> = Vector3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};

/// Predefined constant vector along the Y-axis.
pub const Y_AXIS: Vector3<f64> = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

/// Predefined constant vector along the Z-axis.
pub const Z_AXIS: Vector3<f64> = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

/// Predefined constant zero vector.
pub const VECTOR3_ZERO: Vector3<f64> = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

/// Predefined constant unit vector.
pub const VECTOR3_ONE: Vector3<f64> = Vector3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn const_sum() {
        let one = VECTOR3_ONE;
        assert_eq!(one, VECTOR3_ZERO + X_AXIS + Y_AXIS + Z_AXIS);
    }
}
