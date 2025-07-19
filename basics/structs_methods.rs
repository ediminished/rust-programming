struct User{
    name: String,
    age: u8,
    active: bool,
}

struct Color(u8, u8, u8);

struct UnitStruct;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let user1 = User {
        name: String::from("Alice"),
        age: 30,
        active: true,
    };

    let user2 = User {
        name: String::from("Bob"),
        age: 25,
        active: false,
    };

    let black = Color(0, 0, 0);

    let rect = Rectangle { width: 10, height: 5 };
    println!("Rectangle area: {}", rect.area());

    println!("User 1: {}, Age: {}, Active: {}", user1.name, user1.age, user1.active);
    println!("User 2: {}, Age: {}, Active: {}", user2.name, user2.age, user2.active);
    println!("Black Color RGB: ({}, {}, {})", black.0, black.1, black.2);

    // this is associated function to create a square
    // using the Rectangle struct
    let square = Rectangle::square(4);
    println!("Square area: {}", square.area());

    // this will not compile because `square` is an associated function and not a method on an instance
    // Uncommenting the next line will cause a compilation error
    // let square_area = square.area(4); // This line is incorrect
    // let rect2 = Rectangle { width: 3, height: 4 };
    // println!("Rectangle 2 area: {}", rect2.square(4));
}