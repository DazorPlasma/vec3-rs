use std::str::FromStr;

use crate::{Vector3, Vector3Coordinate};
use thiserror::Error;

impl<T: Vector3Coordinate> From<(T, T, T)> for Vector3<T> {
    fn from(value: (T, T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl<T: Vector3Coordinate> From<Vector3<T>> for (T, T, T) {
    fn from(value: Vector3<T>) -> Self {
        (value.x, value.y, value.z)
    }
}

impl<T: Vector3Coordinate> From<[T; 3]> for Vector3<T> {
    fn from(value: [T; 3]) -> Self {
        let [x, y, z] = value;
        Self { x, y, z }
    }
}

impl<T: Vector3Coordinate> From<Vector3<T>> for [T; 3] {
    fn from(value: Vector3<T>) -> Self {
        [value.x, value.y, value.z]
    }
}

#[derive(Error, Debug)]
pub enum ParseVector3Error {
    #[error("failed to parse #{0}th component")]
    StringParseComponentError(usize),
    #[error("invalid format")]
    InvalidStringFormat,
    #[error("invalid vector length: expected 3, got {0}")]
    InvalidVec(usize),
}

impl<T> FromStr for Vector3<T>
where
    T: Vector3Coordinate + FromStr,
{
    type Err = ParseVector3Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 14 {
            return Err(ParseVector3Error::InvalidStringFormat);
        }

        if &s[..8] != "Vector3(" || !s.ends_with(')') {
            return Err(ParseVector3Error::InvalidStringFormat);
        }

        let data = &s[8..s.len() - 1];
        let mut new_vector: [T; 3] = [T::zero(), T::zero(), T::zero()];
        for (index, coord) in data.split(',').take(3).enumerate() {
            new_vector[index] = coord
                .trim()
                .parse::<T>()
                .map_err(|_| ParseVector3Error::StringParseComponentError(index + 1))?;
        }

        Ok(Self::from(new_vector))
    }
}

macro_rules! impl_try_from_stringlike {
    ($($string_type:ty),*) => {
        $(
            impl<T> TryFrom<$string_type> for Vector3<T>
            where
                T: Vector3Coordinate + FromStr,
            {
                type Error = ParseVector3Error;

                fn try_from(s: $string_type) -> Result<Self, Self::Error> {
                    let s_str: &str = s.as_ref();
                    s_str.parse::<Self>()
                }
            }
        )*
    };
}

impl_try_from_stringlike!(&str, String, Box<str>, std::borrow::Cow<'_, str>);

impl<T: Vector3Coordinate + std::fmt::Debug> TryFrom<Vec<T>> for Vector3<T> {
    type Error = ParseVector3Error;
    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        let array: [T; 3] = value
            .try_into()
            .map_err(|v: Vec<T>| ParseVector3Error::InvalidVec(v.len()))?;
        Ok(Self::from(array))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn text() {
        let v1: Vector3<i32> = Vector3::new(1, 2, 3);
        let v2 = Vector3::try_from(String::from("Vector3( 1,2,     3)")).unwrap();
        assert!(v1 == v2);
    }
}
