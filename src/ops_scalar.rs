#![allow(clippy::use_self)]

use crate::{Vector3, Vector3Coordinate};
use core::ops::{Div, DivAssign, Mul, MulAssign, Neg};

impl<T: Vector3Coordinate + Neg<Output = T>> Neg for Vector3<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Vector3Coordinate + Neg<Output = T> + Copy> Neg for &Vector3<T> {
    type Output = Vector3<T>;
    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Vector3Coordinate> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs.clone(),
            y: self.y * rhs.clone(),
            z: self.z * rhs,
        }
    }
}

impl<T: Vector3Coordinate> Div<T> for Vector3<T> {
    type Output = Vector3<T>;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs.clone(),
            y: self.y / rhs.clone(),
            z: self.z / rhs,
        }
    }
}

impl<T: Vector3Coordinate> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.clone();
        self.y *= rhs.clone();
        self.z *= rhs;
    }
}

impl<T: Vector3Coordinate> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs.clone();
        self.y /= rhs.clone();
        self.z /= rhs;
    }
}
