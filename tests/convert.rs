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
