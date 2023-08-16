# vec3-rs

https://crates.io/crates/vec3-rs

This crate provides a simple and efficient implementation of 3D vectors in Rust, with various utility functions for vector manipulation and operations.

## Features

- Represents 3D vectors in Cartesian space.
- Includes predefined constant vectors for X, Y, and Z axes.
- Supports basic vector operations such as addition, subtraction, dot product, cross product, etc.
- Supports all primitive number types.
- Provides methods for vector normalization, linear interpolation, and angle calculations.
- Allows fuzzy equality comparison within a specified epsilon.

## Example
```rust
use vec3_rs::Vector3;

fn main() {
    let mut v1: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
    let mut v2: Vector3<f64> = Vector3::new(3.0, 1.0, 2.0);

    // Basic operations
    let sum = v1 + v2;
    let difference = v1 - v2;
    let dot_product = v1.dot(&v2);
    let cross_product = v1.cross(&v2);

    // Other methods
    let lerp_result = v1.lerp(&v2, 0.5);
    let angle = v1.angle(&v2);
    let fuzzy_equal = v1.fuzzy_equal(&v2, 0.001);

    println!("Sum: {sum}");
    println!("Difference: {difference}");
    println!("Dot product: {dot_product}");
    println!("Cross product: {cross_product}");
    println!("Lerp 50%: {lerp_result}");
    println!("Angle: {angle}");
    print!("Are they close enough?: {fuzzy_equal}");

    v1.normalize();
    v2.normalize();

    println!("v1 normalized: {v1}");
    println!("v2 normalized: {v2}");
}
```
