fn main(){
    let name = "Akshay";
    // println for variables
    println!("Hello, world! {}", name);
    // println using format macro
    let msg = format!("Hello, world! {}", name);
    println!("{}", msg);
    // println using positional named arguments
    println!("Hello, {name}! {name} is a great name!", name=name);
    // printing floating point numbers
    let pi = 3.14159;
    // printing with default precision
    println!("The value of pi is approximately {}", pi);
    println!("The value of pi with two digits precision is approximately {:.2}", pi);
    let x = 42;
    // printing integers
    println!("The value of x is {}", x);
    // printing with padding of 3 digits
    println!("The value of x with padding of 3 digits is {:03}", x);
    println!("The value of x with padding of 3 digits is {:0>3}", x);
    // printing with padding of 3 spaces aligned to the right
    println!("The value of x with padding of 3 spaces is {:>5} aliged right", x);
    // printing with padding of 3 spaces aligned to the left
    println!("The value of x with padding of 3 spaces is {:<5} aligned left", x);
    // printing with padding of 3 spaces centered
    println!("The value of x with padding of 3 spaces is {:^5} aligned center", x);
    // binary, octal, and hexadecimal representations
    let x = 13;
    println!("The value of x in binary is {:b}", x);
    println!("The value of x in binary is {:#b} (with b prefix)", x);
    println!("The value of x in octal is {:o}", x);
    println!("The value of x in hexadecimal is {:x}", x);
    // printing with a sign
    println!("The value of x with a sign is {:+}", x);
}