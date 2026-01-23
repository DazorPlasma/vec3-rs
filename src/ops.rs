#![allow(clippy::use_self)]
use crate::{Vector3, Vector3Coordinate};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! impl_binary_op {
    ($trait:ident, $method:ident, $op:tt) => {
        // &Vector3 op &Vector3
        impl<'a, 'b, T: Vector3Coordinate> $trait<&'b Vector3<T>> for &'a Vector3<T> {
            type Output = Vector3<T>;
            fn $method(self, rhs: &'b Vector3<T>) -> Self::Output {
                Vector3 {
                    x: self.x.clone() $op rhs.x.clone(),
                    y: self.y.clone() $op rhs.y.clone(),
                    z: self.z.clone() $op rhs.z.clone(),
                }
            }
        }

        // Vector3 op &Vector3
        impl<'a, T: Vector3Coordinate> $trait<&'a Vector3<T>> for Vector3<T> {
            type Output = Vector3<T>;
            fn $method(self, rhs: &'a Vector3<T>) -> Self::Output {
                Vector3 {
                    x: self.x $op rhs.x.clone(),
                    y: self.y $op rhs.y.clone(),
                    z: self.z $op rhs.z.clone(),
                }
            }
        }

        // &Vector3 op Vector3
        impl<'a, T: Vector3Coordinate> $trait<Vector3<T>> for &'a Vector3<T> {
            type Output = Vector3<T>;
            fn $method(self, rhs: Vector3<T>) -> Self::Output {
                Vector3 {
                    x: self.x.clone() $op rhs.x,
                    y: self.y.clone() $op rhs.y,
                    z: self.z.clone() $op rhs.z,
                }
            }
        }

        // Vector3 op Vector3
        impl<T: Vector3Coordinate> $trait<Vector3<T>> for Vector3<T> {
            type Output = Vector3<T>;
            fn $method(self, rhs: Vector3<T>) -> Self::Output {
                Vector3 {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                    z: self.z $op rhs.z,
                }
            }
        }
    };
}

impl_binary_op!(Add, add, +);
impl_binary_op!(Sub, sub, -);
// impl_binary_op!(Mul, mul, *);
// impl_binary_op!(Div, div, /);

macro_rules! impl_scalar_op {
    ($trait:ident, $method:ident, $op:tt) => {
        // &Vector3 op T
        impl<'a, T: Vector3Coordinate> $trait<T> for &'a Vector3<T> {
            type Output = Vector3<T>;
            fn $method(self, rhs: T) -> Self::Output {
                Vector3 {
                    x: self.x.clone() $op rhs.clone(),
                    y: self.y.clone() $op rhs.clone(),
                    z: self.z.clone() $op rhs,
                }
            }
        }

        // Vector3 op T
        impl<T: Vector3Coordinate> $trait<T> for Vector3<T> {
            type Output = Vector3<T>;
            fn $method(self, rhs: T) -> Self::Output {
                Vector3 {
                    x: self.x $op rhs.clone(),
                    y: self.y $op rhs.clone(),
                    z: self.z $op rhs,
                }
            }
        }
    };
}

impl_scalar_op!(Mul, mul, *);
impl_scalar_op!(Div, div, /);

impl<T: Vector3Coordinate> AddAssign<Vector3<T>> for Vector3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Vector3Coordinate> SubAssign<Vector3<T>> for Vector3<T> {
    fn sub_assign(&mut self, rhs: Vector3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Vector3Coordinate> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.clone();
        self.y *= rhs.clone();
        self.z *= rhs;
    }
}

// impl<T: Vector3Coordinate> MulAssign<Vector3<T>> for Vector3<T> {
//     fn mul_assign(&mut self, rhs: Vector3<T>) {
//         self.x *= rhs.x;
//         self.y *= rhs.y;
//         self.z *= rhs.z;
//     }
// }

impl<T: Vector3Coordinate> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs.clone();
        self.y /= rhs.clone();
        self.z /= rhs;
    }
}

// impl<T: Vector3Coordinate> DivAssign<Vector3<T>> for Vector3<T> {
//     fn div_assign(&mut self, rhs: Vector3<T>) {
//         self.x /= rhs.x;
//         self.y /= rhs.y;
//         self.z /= rhs.z;
//     }
// }

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

impl<T: Vector3Coordinate + Neg<Output = T>> Neg for &Vector3<T> {
    type Output = Vector3<T>;
    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x.clone(),
            y: -self.y.clone(),
            z: -self.z.clone(),
        }
    }
}

