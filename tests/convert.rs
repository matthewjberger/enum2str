use enum2str::EnumStr;

#[derive(EnumStr)]
enum Object {
    Generic(String),

    #[enum2str("Color: {}. Shape: {}.")]
    Complex(Color, Shape),
}

#[derive(EnumStr, Debug, PartialEq)]
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
fn color_variant_names() {
    assert_eq!(
        Color::variant_names(),
        vec![
            "Green".to_string(),
            "Red".to_string(),
            "Blue".to_string(),
            "Custom".to_string(),
            "Unique".to_string(),
        ]
    );
}

#[test]
fn object_variant_names() {
    assert_eq!(
        Object::variant_names(),
        vec!["Generic".to_string(), "Complex".to_string()]
    );
}

#[test]
fn plain_to_string() {
    assert_eq!(Color::Blue { _hue: 3 }.to_string(), "Blue");
}

#[test]
fn custom_to_string() {
    assert_eq!(
        Color::Custom {
            _red: 8,
            _green: 7,
            _blue: 6
        }
        .to_string(),
        "Custom Color"
    );
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
fn custom_template() {
    assert_eq!(
        Color::Custom {
            _red: 1,
            _green: 2,
            _blue: 3
        }
        .template(),
        "Custom Color"
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

#[test]
fn complex_args() {
    assert_eq!(
        Object::Complex(Color::Green, Shape::Circle(2)).arguments(),
        vec!["Green", "Circle with radius: 2"],
    );
}

#[derive(EnumStr, Debug, PartialEq, Eq)]
enum SpecialEnum {
    #[allow(unused)]
    #[enum2str("SomeString")]
    SomeValue(u32),
}

#[test]
fn special_string() {
    assert_eq!(SpecialEnum::SomeValue(100).to_string(), "SomeString");
}

#[test]
fn special_template() {
    assert_eq!(SpecialEnum::SomeValue(100).template(), "SomeString");
}

#[test]
fn special_args() {
    assert_eq!(SpecialEnum::SomeValue(100).arguments().len(), 0);
}

#[test]
fn test_from_str() {
    use std::str::FromStr;
    assert_eq!(
        Color::from_str(&Color::Green.to_string()).unwrap(),
        Color::Green
    );
    assert_eq!(Color::from_str("Burgundy").unwrap(), Color::Red);
    assert!(SpecialEnum::from_str(&SpecialEnum::SomeValue(1).to_string()).is_err());
    assert!(Color::from_str("NotAColor").is_err());
}
