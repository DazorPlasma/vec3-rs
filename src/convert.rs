use crate::{Vector3, Vector3Coordinate};

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

#[cfg(feature = "std")]
impl<T: Vector3Coordinate> From<Vector3<T>> for [T; 3] {
    fn from(value: Vector3<T>) -> Self {
        [value.x, value.y, value.z]
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParseVector3Error {
    #[error("failed to parse #{0}th component")]
    StringParseComponentError(usize),
    #[error("invalid format")]
    InvalidStringFormat,
    #[error("invalid vector length: expected 3, got {0}")]
    InvalidVec(usize),
    #[error("invalid slice length: expected 3, got {0}")]
    InvalidSlice(usize),
}

#[cfg(feature = "std")]
impl<T: Vector3Coordinate> TryFrom<Vec<T>> for Vector3<T> {
    type Error = ParseVector3Error;
    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        let array: [T; 3] = value
            .try_into()
            .map_err(|v: Vec<T>| ParseVector3Error::InvalidVec(v.len()))?;
        Ok(Self::from(array))
    }
}

#[cfg(feature = "std")]
impl<T: Vector3Coordinate> TryFrom<std::collections::VecDeque<T>> for Vector3<T> {
    type Error = ParseVector3Error;
    fn try_from(mut value: std::collections::VecDeque<T>) -> Result<Self, Self::Error> {
        let x = value
            .pop_front()
            .ok_or(ParseVector3Error::InvalidVec(value.len()))?;
        let y = value
            .pop_front()
            .ok_or(ParseVector3Error::InvalidVec(value.len()))?;
        let z = value
            .pop_front()
            .ok_or(ParseVector3Error::InvalidVec(value.len()))?;

        Ok(Self::new(x, y, z))
    }
}

impl<T: Vector3Coordinate> TryFrom<&[T]> for Vector3<T> {
    type Error = ParseVector3Error;
    fn try_from(value: &[T]) -> Result<Self, Self::Error> {
        let array: &[T; 3] = value
            .as_array()
            .ok_or(ParseVector3Error::InvalidSlice(value.len()))?;

        Ok(Self::from(array.clone()))
    }
}

impl<T: Vector3Coordinate> TryFrom<Box<[T]>> for Vector3<T> {
    type Error = ParseVector3Error;
    fn try_from(value: Box<[T]>) -> Result<Self, Self::Error> {
        let array: &[T; 3] = value
            .as_array()
            .ok_or(ParseVector3Error::InvalidSlice(value.len()))?;

        Ok(Self::from(array.clone()))
    }
}

impl<T> core::str::FromStr for Vector3<T>
where
    T: Vector3Coordinate + core::str::FromStr,
{
    type Err = ParseVector3Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(data) = s.strip_prefix("Vector3(") else {
            return Err(ParseVector3Error::InvalidStringFormat);
        };
        let Some(data) = data.strip_suffix(")") else {
            return Err(ParseVector3Error::InvalidStringFormat);
        };

        let mut components = data
            .split(',')
            .take(3)
            .enumerate()
            .flat_map(|(index, coord)| {
                coord
                    .trim()
                    .parse::<T>()
                    .map_err(|_| ParseVector3Error::StringParseComponentError(index + 1))
            });

        // dear rust, collect into array/tuple when?
        // do i seriously need to pull itertools for this??
        let x = components
            .next()
            .ok_or(ParseVector3Error::StringParseComponentError(1))?;
        let y = components
            .next()
            .ok_or(ParseVector3Error::StringParseComponentError(2))?;
        let z = components
            .next()
            .ok_or(ParseVector3Error::StringParseComponentError(3))?;

        Ok(Self::new(x, y, z))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec_string() {
        let v1: Vector3<i32> = Vector3::new(1, 2, 3);
        let v2: Vector3<i32> = String::from("Vector3( 1,2,     3)").parse().unwrap();
        assert_eq!(v1, v2);
    }

    #[test]
    fn vec_str() {
        let v1: Vector3<f64> = Vector3::new(1, 2, 3);
        let v2 = "Vector3(1,  2 ,3 )".parse().unwrap();
        assert_eq!(v1, v2);
    }

    #[test]
    fn vec_tuple() {
        let v1 = Vector3::new(1, 2, 3);
        let v2 = (1, 2, 3).into();
        assert_eq!(v1, v2);
    }

    #[test]
    fn vec_array() {
        let v1 = Vector3::new(1, 2, 3);
        let v2 = [1, 2, 3].into();
        assert_eq!(v1, v2);
    }

    #[test]
    fn vec_vec() {
        let v1 = Vector3::new(1, 2, 3);
        let v2 = vec![1, 2, 3].try_into().unwrap();
        assert_eq!(v1, v2);
    }

    #[test]
    fn vec_deq() {
        let v1 = Vector3::new(1, 2, 3);
        let v2 = std::collections::VecDeque::from([1, 2, 3])
            .try_into()
            .unwrap();
        assert_eq!(v1, v2);
    }
}
