// function with no parameters
fn greet(){
    println!("Hello from a function!");
}

// functions with paramaters
fn square(x: i32) -> i32 {
    
    // return x * x;
    // the return keyword is optional in Rust, so you can just write the expression
    x * x
}

// funtion with multiple parameters
fn add(x: i32, y: i32) -> i32 {
    x + y
}


fn main() {
    let res = greet();
    println!("The result of the greet function with nothing returned is: {:?}", res);
    let num = 5;
    let result = square(num);
    println!("The square of {} is: {}", num, result);
    let result_add = add(3, 4);
    println!("The sum of 3 and 4 is: {}", result_add);
}

