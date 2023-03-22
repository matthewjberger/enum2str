# enum2str

[<img alt="github" src="https://img.shields.io/badge/github-matthewjberger/enum2str-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/matthewjberger/enum2str)
[<img alt="crates.io" src="https://img.shields.io/crates/v/enum2str.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/enum2str)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-enum2str-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/enum2str)

enum2str is a rust derive macro that creates a Display impl for enums. 
This is useful for strongly typing composable sets of strings.

## Usage

Add this to your `Cargo.toml`:

```toml
enum2str = "0.1.0"
```

Example:

```rust
use enum2str::EnumStr;

#[derive(EnumStr)]
enum Object {
    Simple,

    #[enum2str("Color: {}. Shape: {}.")]
    Complex(Color, Shape),
}

#[derive(EnumStr)]
enum Color {
    Red,
    Green,
    SlateGray,
}

#[derive(EnumStr)]
enum Shape {
    Circle,
}

#[test]
fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    assert_eq!(Object::Simple.to_string(), "Simple");
    assert_eq!(
        Object::Complex(Color::Green, Shape::Circle).to_string(),
        "Color: Green. Shape: Circle."
    );
    assert_eq!(Color::Red.to_string(), "Red");
    assert_eq!(Color::SlateGray.to_string(), "SlateGray");
    Ok(())
}
```
