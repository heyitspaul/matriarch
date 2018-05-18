# Matriarch
A Linear Algebra and Matrix library written in Rust.

## Getting Started

Simply put Matriarch in your `Cargo.toml` file:

```toml
[dependencies]
"matriarch" = "0.1.0"
```

and then you're ready to go!

```rust
extern crate matriarch;

use matriarch::Vec2;

fn main() {
    let vec2 = Vec2::new();
    let new_vec2 = vec2 + Vec2 { x: 1.0, y: 3.0 };
    println!("{:?}", new_vec2);
    //=> Vec2 { x: 1.0, y: 3.0 }
}
```

## TODO

* Implement the rest of the library
* Implement Determinants, Eigen values, and Eigenvectors
* Add OpenGL convenience functions (MVP, Ortho, etc.)
* Documentation
* Crates.io release

## Code of Conduct

Interacting with the Matriarch community in any shape or form requires you adhere to our simple and easy to follow [Code of Conduct](https://github.com/theassailant/matriarch/blob/master/CODE_OF_CONDUCT.md).

## License

Matriarch is licensed under the [MIT License](https://opensource.org/licenses/MIT).

## Contributors

- [TheAssailant](https://github.com/TheAssailant) - creator, maintainer
