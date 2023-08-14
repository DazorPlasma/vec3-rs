#![deny(unsafe_code, warnings, clippy::all)]

use crate::Vector3;
use thiserror::Error;

impl From<(f64, f64, f64)> for Vector3 {
    fn from(value: (f64, f64, f64)) -> Self {
        Vector3 {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl From<Vector3> for (f64, f64, f64) {
    fn from(value: Vector3) -> Self {
        (value.x, value.y, value.z)
    }
}

impl From<[f64; 3]> for Vector3 {
    fn from(value: [f64; 3]) -> Self {
        Vector3 {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl From<Vector3> for [f64; 3] {
    fn from(value: Vector3) -> Self {
        [value.x, value.y, value.z]
    }
}

#[derive(Error, Debug)]
pub enum ParseVector3Error {
    #[error("failed to parse numbers")]
    ParseNumberError(#[from] std::num::ParseFloatError),
    #[error("invalid format")]
    InvalidFormat,
    #[error("invalid Vec<f64>")]
    InvalidVec,
}

impl TryFrom<&str> for Vector3 {
    type Error = ParseVector3Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < 14 {
            return Err(ParseVector3Error::InvalidFormat);
        }

        if &value[..8] != "Vector3(" || !value.ends_with(')') {
            return Err(ParseVector3Error::InvalidFormat);
        }

        let data = &value[8..value.len() - 1];
        let mut new_vector: [f64; 3] = [0.0, 0.0, 0.0];
        for (index, coord) in data.split(',').enumerate() {
            new_vector[index] = coord.trim().parse::<f64>()?;
        }

        Ok(Vector3::from(new_vector))
    }
}

impl TryFrom<Vec<f64>> for Vector3 {
    type Error = ParseVector3Error;
    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        let x = value.first().ok_or(ParseVector3Error::InvalidVec)?;
        let y = value.get(1).ok_or(ParseVector3Error::InvalidVec)?;
        let z = value.get(2).ok_or(ParseVector3Error::InvalidVec)?;

        Ok(Vector3::new(*x, *y, *z))
    }
}
