# vec3-rs

[This crate](https://crates.io/crates/vec3-rs) provides a simple implementation of 3D vectors in Rust.
Supports any numeric type trough [num-traits](https://crates.io/crates/num-traits).

## Example

```rust
type Vector3 = Vector3<f64>;

fn main() {
   let v1 = Vector3::new(1, 2.5, 3);
   let v2 = Vector3::from([9.0, 1.0, 4.0]);

   let v3 = (v1 + v2) - (v1 - v2);
   let v3 = v3.cross(&v2);
   let v3 = v3 * v3.dot(&v1);
   let v3 = v3 * 10.0 / 3.2;
   let v3 = v3.normalized();
   let v3 = v3.lerp(&Vector3::zero(), 0.25);
   let v3 = v3.floor();

   println!("{v3}");
   println!("{}", v3.angle(&Vector3::z_axis()));
   println!("{}", v3.fuzzy_equal(&Vector3::z_axis(), 2.0));
}
```
