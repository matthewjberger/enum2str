# enum2str

[<img alt="github" src="https://img.shields.io/badge/github-matthewjberger/enum2str-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/matthewjberger/enum2str)
[<img alt="crates.io" src="https://img.shields.io/crates/v/enum2str.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/enum2str)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-enum2str-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/enum2str)

enum2str is a rust derive macro that creates a Display impl for enums. 
This is useful for strongly typing composable sets of strings.

## Usage

Add this to your `Cargo.toml`:

```toml
enum2str = "0.1.8"
```

Example:

```rust
use enum2str::EnumStr;

#[derive(EnumStr)]
enum Object {
    Generic(String),

    #[enum2str("Color: {}. Shape: {}.")]
    Complex(Color, Shape),

    // Variant fields can be ignored
    #[enum2str("Material")]
    Material(Color),
}

#[derive(EnumStr)]
enum Color {
    Green,

    #[enum2str("Burgundy")]
    Red,

    Blue {
        _hue: u8,
    },

    #[enum2str("Custom Color")]
    Custom {
        _red: u8,
        _green: u8,
        _blue: u8,
    },

    #[enum2str("Unique - {label}_{id}")]
    Unique {
        id: u8,
        label: String,
    },
}

#[derive(EnumStr)]
enum Shape {
    #[enum2str("Circle with radius: {}")]
    Circle(u8),
}

#[test]
fn unit_to_string() {
    assert_eq!(Color::Green.to_string(), "Green");
}

#[test]
fn unit_override_string() {
    assert_eq!(Color::Red.to_string(), "Burgundy");
}

#[test]
fn unnamed_to_string() {
    assert_eq!(Object::Generic("Hello!".to_string()).to_string(), "Hello!");
}

#[test]
fn nested_to_string() {
    assert_eq!(
        Object::Complex(Color::Green, Shape::Circle(2)).to_string(),
        "Color: Green. Shape: Circle with radius: 2."
    );
}

#[test]
fn unit_template() {
    assert_eq!(Color::Green.template(), "Green");
}

#[test]
fn unit_override_template() {
    assert_eq!(Color::Red.template(), "Burgundy");
}

#[test]
fn unnamed_template() {
    assert_eq!(Shape::Circle(2).template(), "Circle with radius: {}");
}

#[test]
fn nested_template() {
    assert_eq!(
        Object::Complex(Color::Green, Shape::Circle(2)).template(),
        "Color: {}. Shape: {}."
    );
}

#[test]
fn unit_args() {
    assert_eq!(Color::Green.arguments().len(), 0);
}

#[test]
fn unnamed_args() {
    assert_eq!(
        Object::Generic("Hello!".to_string()).arguments(),
        vec!["Hello!".to_string()]
    );
}

#[test]
fn complex_args() {
    assert_eq!(
        Object::Complex(Color::Green, Shape::Circle(2)).arguments(),
        vec!["Green", "Circle with radius: 2"],
    );
}

#[test]
fn plain_to_string() {
    assert_eq!(Color::Blue { _hue: 3 }.to_string(), "Blue");
}

#[test]
fn unique_to_string() {
    assert_eq!(
        Color::Unique {
            label: "unique_color".to_string(),
            id: 3
        }
        .to_string(),
        "Unique - unique_color_3",
    );
}

#[test]
fn custom_args() {
    assert_eq!(
        Color::Unique {
            label: "unique_color".to_string(),
            id: 3
        }
        .arguments()
        .len(),
        2
    );
}
```
