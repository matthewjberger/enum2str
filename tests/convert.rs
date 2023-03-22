use enum2str::EnumStr;

#[derive(EnumStr)]
enum Object {
    Generic(String),

    #[enum2str("Color: {}. Shape: {}.")]
    Complex(Color, Shape),
}

#[derive(EnumStr)]
enum Color {
    #[enum2str("Burgundy")]
    Red,
    Green,
}

#[derive(EnumStr)]
enum Shape {
    Circle,
}

#[test]
fn enum2str() {
    assert_eq!(Color::Green.to_string(), "Green");

    assert_eq!(Color::Red.to_string(), "Burgundy");

    assert_eq!(Object::Generic("Hello!".to_string()).to_string(), "Hello!");

    assert_eq!(
        Object::Complex(Color::Green, Shape::Circle).to_string(),
        "Color: Green. Shape: Circle."
    );
}
