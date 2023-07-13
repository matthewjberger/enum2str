use enum2str::EnumStr;

#[derive(EnumStr)]
enum Object {
    Generic(String),

    #[enum2str("Color: {}. Shape: {}.")]
    Complex(Color, Shape),
}

#[derive(EnumStr)]
enum Color {
    Green,

    #[enum2str("Burgundy")]
    Red,
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

#[derive(EnumStr)]
enum SpecialEnum {
    #[enum2str("SomeString")]
    SomeString(u32),
}

#[test]
fn special_string() {
    assert_eq!(SpecialEnum::SomeString(100).to_string(), "SomeString");
}

#[test]
fn special_template() {
    assert_eq!(SpecialEnum::SomeString(100).template(), "SomeString");
}

#[test]
fn special_args() {
    assert_eq!(SpecialEnum::SomeString(100).arguments().len(), 0);
}
