// enum lets us define a type that can be one of several variants
// each variant can have different data associated with it
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// enums with data
enum Shape {
    Circle(f64), // Circle with radius
    Rectangle { width: f64, height: f64 }, // Rectangle with width and height
    Triangle { base: f64, height: f64 }, // Triangle with base and height
}

// Option<T> type is a special enum in Rust
// that can be either Some(T) or None
// enum Option<T> {
//     Some(T),
//     None,
// }

// use Option<T> to handle cases where a value might be absent

fn main() {
    let dir = Direction::Up;
    match dir {
        Direction::Up => println!("Moving up!"),
        Direction::Down => println!("Moving down!"),
        Direction::Left => println!("Moving left!"),
        Direction::Right => println!("Moving right!"),
    }

    let s = Shape::Rectangle { width: 10.0, height: 5.0 };
    match s {
        Shape::Circle(radius) => println!("Circle with radius: {}", radius),
        Shape::Rectangle { width, height } => println!("Rectangle with width: {}, height: {}", width, height),
        Shape::Triangle { base, height } => println!("Triangle with base: {}, height: {}", base, height),
    }

    let maybe_name: Option<&str> = Some("Akshay");
    let maybe_none: Option<&str> = None;
    println!("{}", maybe_name.unwrap_or("Guest"));
    println!("{}", maybe_none.unwrap_or("Guest"));
}