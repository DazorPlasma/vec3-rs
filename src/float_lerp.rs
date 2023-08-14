#[deny(unsafe_code, warnings, clippy::all)]

/// A trait for linear interpolation (lerp) between two values of type `T` using a blending factor `alpha`.
///
/// The `Lerp` trait provides a method to perform linear interpolation between two values, which
/// smoothly transitions between them based on the given blending factor `alpha`.
///
/// # Examples
///
/// ```
/// trait Lerp<T> {
///     fn lerp(&self, goal: T, alpha: f64) -> T;
/// }
///
/// impl Lerp<f64> for f64 {
///     fn lerp(&self, goal: f64, alpha: f64) -> f64 {
///         self + (goal - self) * alpha
///     }
/// }
///
/// let start = 0.0;
/// let end = 10.0;
/// let alpha = 0.5;
/// let interpolated_value = start.lerp(end, alpha);
/// println!("Interpolated value: {}", interpolated_value);
/// ```
pub trait Lerp<T> {
    /// Performs linear interpolation between `self` and a `goal` value using the provided blending factor `alpha`.
    ///
    /// The resulting value represents a smooth transition from `self` to `goal` based on the given `alpha`,
    /// where `alpha` is typically in the range [0.0, 1.0].
    ///
    /// # Parameters
    ///
    /// - `goal`: The target value to interpolate towards.
    /// - `alpha`: The blending factor determining the interpolation point between `self` and `goal`.
    ///
    /// # Returns
    ///
    /// The interpolated value between `self` and `goal` based on the given `alpha`.
    fn lerp(&self, goal: T, alpha: f64) -> T;
}

/// Implementation of the `Lerp` trait for `f64` type.
impl Lerp<f64> for f64 {
    /// Performs linear interpolation between two `f64` values.
    ///
    /// The interpolation is calculated as: `self + (goal - self) * alpha`.
    ///
    /// # Parameters
    ///
    /// - `goal`: The target value to interpolate towards.
    /// - `alpha`: The blending factor determining the interpolation point between `self` and `goal`.
    ///
    /// # Returns
    ///
    /// The interpolated `f64` value between `self` and `goal` based on the given `alpha`.
    fn lerp(&self, goal: f64, alpha: f64) -> f64 {
        self + (goal - self) * alpha
    }
}
