# vec3-rs

This crate provides a simple and efficient implementation of 3D vectors in Rust, with various utility functions for vector manipulation and operations.

## Features

- Represents 3D vectors in Cartesian space.
- Prevents creation of vectors with NaN coordinates.
- Includes predefined constant vectors for X, Y, and Z axes.
- Supports basic vector operations such as addition, subtraction, dot product, cross product, etc.
- Provides methods for vector normalization, linear interpolation, and angle calculations.
- Allows fuzzy equality comparison within a specified epsilon.

## Example
```rust
use vec3_rs::Vector3;

fn main() {
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(3.0, 1.0, 2.0);

    // Basic operations
    let sum = v1 + v2;
    let difference = v1 - v2;
    let dot_product = v1.dot(&v2);
    let cross_product = v1.cross(&v2);

    // Other methods
    let normalized = v1.normalize();
    let lerp_result = v1.lerp(&v2, 0.5);
    let angle = v1.angle(&v2);
    let fuzzy_equal = v1.fuzzy_equal(&v2, 0.001);

    println!("Sum: {:?}", sum);
    println!("Normalized: {:?}", normalized);
}
```