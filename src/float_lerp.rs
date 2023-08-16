#![deny(unsafe_code, warnings, clippy::all)]

pub trait Lerp<T: num::Float> {
    /// Performs linear interpolation between `self` and `goal` based on the given `alpha`.
    ///
    /// The `alpha` parameter should be a float between 0.0 and 1.0. A value of 0.0 results in the same
    /// value as `self`, while a value of 1.0 results in the same value as `goal`.
    ///
    /// # Arguments
    ///
    /// * `self` - The starting value for interpolation.
    /// * `goal` - The target value for interpolation.
    /// * `alpha` - The interpolation factor, ranging from 0.0 to 1.0.
    ///
    /// # Returns
    ///
    /// The interpolated value between `self` and `goal`.
    fn lerp(&self, goal: T, alpha: T) -> T;
}

impl<T: num::Float> Lerp<T> for T {
    fn lerp(&self, goal: T, alpha: T) -> T {
        *self + (goal - *self) * alpha
    }
}
