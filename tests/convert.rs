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
fn unit_number_of_args() {
    assert_eq!(Color::Green.number_of_args(), 0);
}

#[test]
fn unnamed_number_of_args() {
    assert_eq!(Object::Generic("Hello!".to_string()).number_of_args(), 1);
}

#[test]
fn complex_number_of_args() {
    assert_eq!(
        Object::Complex(Color::Green, Shape::Circle(2)).number_of_args(),
        2
    );
}
