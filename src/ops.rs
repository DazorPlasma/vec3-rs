#![deny(unsafe_code, warnings, clippy::all)]

use crate::{Vector3, Vector3Coordinate};

impl<T: Vector3Coordinate> std::ops::Add<Vector3<T>> for Vector3<T> {
    type Output = Self;
    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Vector3Coordinate> std::ops::AddAssign<Vector3<T>> for Vector3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Vector3Coordinate> std::ops::Sub<Vector3<T>> for Vector3<T> {
    type Output = Self;
    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Vector3Coordinate> std::ops::SubAssign<Vector3<T>> for Vector3<T> {
    fn sub_assign(&mut self, rhs: Vector3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Vector3Coordinate> std::ops::Mul<T> for Vector3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Vector3Coordinate> std::ops::Mul<Vector3<T>> for Vector3<T> {
    type Output = Self;
    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: Vector3Coordinate> std::ops::MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T: Vector3Coordinate> std::ops::MulAssign<Vector3<T>> for Vector3<T> {
    fn mul_assign(&mut self, rhs: Vector3<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T: Vector3Coordinate> std::ops::Div<T> for Vector3<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Vector3Coordinate> std::ops::Div<Vector3<T>> for Vector3<T> {
    type Output = Self;
    fn div(self, rhs: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T: Vector3Coordinate> std::ops::DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T: Vector3Coordinate> std::ops::DivAssign<Vector3<T>> for Vector3<T> {
    fn div_assign(&mut self, rhs: Vector3<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
